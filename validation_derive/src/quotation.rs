//! quote
//!
//! Module that helps with generating the auto-generated code that is provided by the derive
//! Leverages use of the `quote` library to provide helpers in generating the quoted code
//!
//! The auto-generated code leverages the actual rules in the `validation` library; this code
//! simply makes the actual generated code and produces it via the specified rules and types
//! required by the rule itself
//!
//! Options and other fields are handled via the `COW_TYPE` and `NUMBER_TYPES` to prevent any
//! complications with trying to figure out types wrapped in `Option`

use crate::field_validation::FieldValidation;
use crate::types::ValidationType;
use regex::Regex;

lazy_static! {
    pub static ref COW_TYPE: Regex = Regex::new(r"Cow<'[a-z]+,str>").unwrap();
}

/// Constant to handle all known or valid types to use for numbers, including nested types
pub const NUMBER_TYPES: [&str; 36] = [
    "usize",
    "u8",
    "u16",
    "u32",
    "u64",
    "isize",
    "i8",
    "i16",
    "i32",
    "i64",
    "f32",
    "f64",
    "Option<usize>",
    "Option<u8>",
    "Option<u16>",
    "Option<u32>",
    "Option<u64>",
    "Option<isize>",
    "Option<i8>",
    "Option<i16>",
    "Option<i32>",
    "Option<i64>",
    "Option<f32>",
    "Option<f64>",
    "Option<Option<usize>>",
    "Option<Option<u8>>",
    "Option<Option<u16>>",
    "Option<Option<u32>>",
    "Option<Option<u64>>",
    "Option<Option<isize>>",
    "Option<Option<i8>>",
    "Option<Option<i16>>",
    "Option<Option<i32>>",
    "Option<Option<i64>>",
    "Option<Option<f32>>",
    "Option<Option<f64>>",
];

/// Struct helper to allow storing variables used in the generation of quoted code
#[derive(Debug)]
pub struct FieldQuoter {
    ident: syn::Ident,
    name: String,
    _type: String,
}

impl FieldQuoter {
    /// Generates a new FieldQuoter
    ///
    /// # Arguments
    ///
    /// * `ident` - A word of Rust code, per `syn::Ident`
    /// * `name` - Field name
    /// * `_type` - `String` representation of the field type
    pub fn new(ident: syn::Ident, name: String, _type: String) -> FieldQuoter {
        FieldQuoter { ident, name, _type }
    }

    pub fn quote_validate_parameter(&self) -> proc_macro2::TokenStream {
        let ident = &self.ident;

        if self._type.starts_with("Option<") {
            quote!(#ident)
        } else if COW_TYPE.is_match(&self._type.as_ref()) {
            quote!(&self.#ident.as_ref())
        } else if self._type.starts_with('&') || NUMBER_TYPES.contains(&self._type.as_ref()) {
            quote!(&self.#ident)
        } else {
            quote!(self.#ident)
        }
    }
}

/// Creates the new validation for the field, matched by the rule type
///
/// # Arguments
///
/// * `field_quoter` - `FieldQuoter` to help with validation generation
/// * `validation` - `FieldValidation` to add
/// * `validations` - `Vec<TokenStream>` of current and existing validation rules already generated
pub fn create_field_validation(
    field_quoter: &FieldQuoter,
    validation: &FieldValidation,
    validations: &mut Vec<proc_macro2::TokenStream>,
) {
    match validation.validator {
        ValidationType::NotNull { .. } => {
            validations.push(create_not_null_validation(&field_quoter, validation))
        }
    }
}

/// Generates the validation rule `not_null`
/// Returns the `TokenStream` of the generated rule
///
/// # Arguments
///
/// * `field_quoter` - `FieldQuoter` to help with validation generation
/// * `validation` - `FieldValidation` to add
pub fn create_not_null_validation(
    field_quoter: &FieldQuoter,
    validation: &FieldValidation,
) -> proc_macro2::TokenStream {
    let _field_name = &field_quoter.name;
    let ident = &field_quoter.ident;
    let validate_parameter = quote!(&self.#ident);

    let quoted_error = quote_err(&validation);
    let quoted = quote!(
        if !::validation::is_not_null(#validate_parameter) {
            #quoted_error
            errors.push(err)
        }
    );

    quoted
}

fn quote_err(validation: &FieldValidation) -> proc_macro2::TokenStream {
    let code = &validation.code;
    // let _add_message_quote = if let Some(ref m) = validation.message {
    //     quote!(err.message = Some(::std::borrow::Cow::from(#m));)
    // } else {
    //     quote!()
    // };

    quote!(
        let mut err = ::validation::ValidationError::FieldMismatch(#code.to_string());
    )
}
