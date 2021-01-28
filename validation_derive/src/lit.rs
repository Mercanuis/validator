//! lit
//!
//! Module that helps with Literals (Lit) in the syn syntax tree. This is used by the derive a potential
//! renaming of a field that was done via Serde's derive macros. This allows the syntax tree to map
//! the rename to the Rust name: this prevents us from applying a rule to a mismanaged field

/// Converts the given `Lit` to a `Option<String>`
///
/// # Arguments
/// * `lit` - the `Lit` to convert
pub fn lit_to_string(lit: &syn::Lit) -> Option<String> {
    match *lit {
        syn::Lit::Str(ref s) => Some(s.value()),
        _ => None,
    }
}
