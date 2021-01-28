#[macro_use]
extern crate lazy_static;
extern crate proc_macro;
extern crate proc_macro2;
#[macro_use]
extern crate proc_macro_error;
#[macro_use]
extern crate quote;
extern crate regex;
extern crate syn;
extern crate validation;

use std::collections::HashMap;

use proc_macro2::Span;
use proc_macro_error::proc_macro_error;
use syn::export::ToTokens;
use syn::{parse_quote, spanned::Spanned};

use crate::field_validation::FieldValidation;
use crate::lit::lit_to_string;
use crate::quotation::FieldQuoter;
use crate::types::ValidationType;

mod field_validation;
mod lit;
mod quotation;
mod types;

/// Derives and generates a rule (or later, a series or rules) that
/// the particular field must comply to in order to be valid
///
/// <br>
///
/// # Usage
///
/// The library requires a usage of the derive macro, the package `validation_derive` is necessary for this
///
/// <br>
///
/// `FieldValidate` implies to the code to inherit and impl the `FieldValidation` trait. The result is
/// an auto-generated implementation of the rule that is implemented for the struct's field. `validate` is the
/// keyword to map the particular rule (and later on, rules) that one wants to enforce to the field.
///
/// <br>
/// Refer to the individual rules that are part of the `validation` module for further rules
#[proc_macro_derive(FieldValidate, attributes(validate))]
#[proc_macro_error]
pub fn derive_field_validation(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let syntax = syn::parse(input).unwrap();
    impl_field_validation(&syntax).into()
}

fn impl_field_validation(syntax: &syn::DeriveInput) -> proc_macro2::TokenStream {
    // Validate that we can make the derive function. This can only be done when we have
    // a valid struct and the struct does not have any tuple fields (aka (a,b): Blah)
    let fields = match syntax.data {
        syn::Data::Struct(syn::DataStruct { ref fields, .. }) => {
            if fields.iter().any(|field| field.ident.is_none()) {
                abort!(
                    fields.span(),
                    "struct has unnamed fields";
                    help = "#[derive(FieldValidate)] can only be used on structs with named fields";
                );
            }
            fields.iter().cloned().collect::<Vec<_>>()
        }
        _ => abort!(
            syntax.span(),
            "#[derive(FieldValidation)] can only be used with structs"
        ),
    };

    // List of the validation rules to implement at the end once fields and rules are mapped
    // TODO: error or abort if the derive has none?
    let mut validation_rules = vec![];

    // Check the field type
    let field_types = get_field_types(&fields);

    for field in &fields {
        let field_identity = field.ident.clone().unwrap();
        let (name, validations) = find_validations_for_field(field, &field_types);
        let field_type = field_types
            .get(&field_identity.to_string())
            .cloned()
            .unwrap();
        let field_quoter = FieldQuoter::new(field_identity, name, field_type);

        for validation in &validations {
            quotation::create_field_validation(&field_quoter, validation, &mut validation_rules);
        }
    }

    //Field validations are found and quoted
    //Generate the field validation code here
    let identity = &syntax.ident;

    //Syn library provides generics to help with generation
    //Use them per the generics of the implementation
    let (implementation_generics, type_generics, where_clause) = syntax.generics.split_for_impl();
    let implemented_syntax = quote!(
        impl #implementation_generics ::validation::FieldValidation for #identity #type_generics #where_clause {
            fn validate_fields(&self) -> ::validation::ValidationResult<()> {
                let mut errors = ::std::vec::Vec::new();

                #(#validation_rules)*

                if errors.is_empty() {
                    Ok(())
                } else {
                    let mut err = ::validation::ValidationError::FieldMismatch("not_null".to_string());
                    Err(err)
                }
            }
        }
    );

    //TODO: Debug statement, remove later
    println!("{}", implemented_syntax.to_string());
    implemented_syntax
}

//Finds the field types for each field of the struct [string, i32, etc...]
fn get_field_types(fields: &[syn::Field]) -> HashMap<String, String> {
    let mut types = HashMap::new();

    for field in fields {
        let field_identity = field.ident.clone().unwrap().to_string();
        let field_type = match field.ty {
            //A type like std::collection::HashMap
            syn::Type::Path(syn::TypePath { ref path, .. }) => {
                let mut tokens = proc_macro2::TokenStream::new();
                path.to_tokens(&mut tokens);
                tokens.to_string().replace(' ', "")
            }
            //A type like &'a T or &'a mut T
            syn::Type::Reference(syn::TypeReference {
                ref lifetime,
                ref elem,
                ..
            }) => {
                let mut tokens = proc_macro2::TokenStream::new();
                elem.to_tokens(&mut tokens);
                let mut name = tokens.to_string().replace(' ', "");
                if lifetime.is_some() {
                    name.insert(0, '&')
                }
                name
            }
            _ => {
                let mut field_type = proc_macro2::TokenStream::new();
                field.ty.to_tokens(&mut field_type);
                abort!(
                    field.ty.span(),
                    "Type `{}` of field `{}` not supported",
                    field_type,
                    field_identity
                )
            }
        };

        //TODO: Debugging statement, remove this later
        println!("{:?}", field_type);

        types.insert(field_identity, field_type);
    }

    types
}

fn find_validations_for_field(
    field: &syn::Field,
    field_types: &HashMap<String, String>,
) -> (String, Vec<FieldValidation>) {
    // Cloning the field ident twice to helps with a case where a struct has
    // renamed the field and allows us to compare it with what Rust compile a different name
    let rust_identity = field.ident.clone().unwrap().to_string();
    let mut field_identity = field.ident.clone().unwrap().to_string();

    //anonymous fn to handle any errors on invalid [validate] attributes
    let error = |span: Span, msg: &str| -> ! {
        abort!(
            span,
            "Invalid attribute #[validate] on field `{}`: {}",
            rust_identity,
            msg
        );
    };

    let _field_type = field_types.get(&field_identity).unwrap();

    let mut validators = vec![];
    let mut has_validate = false;

    for attr in &field.attrs {
        if attr.path != parse_quote!(validate) && attr.path != parse_quote!(serde) {
            continue;
        }

        if attr.path == parse_quote!(validate) {
            has_validate = true;
        }

        match attr.parse_meta() {
            Ok(syn::Meta::List(syn::MetaList { ref nested, ..})) => {
                let meta_items = nested.iter().collect::<Vec<_>>();
                //For the case of a serde rename check to see if we need to map to the Rust field name
                if attr.path == parse_quote!(serde) {
                    if let Some(s) = find_original_name(&meta_items) {
                        field_identity = s;
                    }
                    continue;
                }

                //We have a field and a valid validation, find the rule to match it to
                for meta_item in meta_items {
                    match *meta_item {
                        syn::NestedMeta::Meta(ref item) => match *item {
                            //not_null
                            syn::Meta::Path(ref name) => {
                                match name.get_ident().unwrap().to_string().as_ref() {
                                    "not_null" => {
                                        validators.push(FieldValidation::new(ValidationType::NotNull))
                                    }
                                    _ => {
                                        let mut ident = proc_macro2::TokenStream::new();
                                        name.to_tokens(&mut ident);
                                        abort!(name.span(), "Unexpected Validation: {}", ident)
                                    }
                                }
                            }
                            syn::Meta::NameValue(syn::MetaNameValue { ref path, lit: _, ..}) => {
                                let ident = path.get_ident().unwrap();
                                abort!(path.span(), "Unexpected Validation: {:?}", ident)
                            }
                            syn::Meta::List(syn::MetaList { ref path, nested: _, ..}) => {
                                let ident = path.get_ident().unwrap();
                                abort!(path.span(), "Unexpected Validation: {:?}", ident)
                            }
                        }
                        _ => unreachable!("Found a non Meta while looking for Validators"),
                    };
                }
            }
            Ok(syn::Meta::Path(_)) => abort!(attr.span(), "Unexpected nested value"),
            Ok(syn::Meta::NameValue(_)) => abort!(attr.span(), "Unexpected name=value argument"),
            Err(e) => unreachable!(
                "Received something other than a list of attributes while checking field `{}`: {:?}", field_identity, e),
        }

        if has_validate && validators.is_empty() {
            error(attr.span(), "there must be at least one validation rule");
        }
    }

    (field_identity, validators)
}

fn find_original_name(meta_items: &[&syn::NestedMeta]) -> Option<String> {
    let mut original_name = None;

    for meta_item in meta_items {
        match **meta_item {
            syn::NestedMeta::Meta(ref item) => match *item {
                syn::Meta::Path(_) => continue,
                syn::Meta::NameValue(syn::MetaNameValue {
                    ref path, ref lit, ..
                }) => {
                    let ident = path.get_ident().unwrap();
                    if ident == "rename" {
                        original_name = Some(lit_to_string(lit).unwrap());
                    }
                }
                syn::Meta::List(syn::MetaList { ref nested, .. }) => {
                    return find_original_name(&nested.iter().collect::<Vec<_>>());
                }
            },
            _ => unreachable!(),
        };
    }

    original_name
}
