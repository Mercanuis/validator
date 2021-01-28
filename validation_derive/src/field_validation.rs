//! field_validation
//!
//! Module containing helper and utility classes used for creation of validation rules
//! TODO: Can we condense this, merging types, lit, and field_validation modules?

use crate::types::ValidationType;

///Helper struct to allow generation of a new validation rule
#[derive(Debug)]
pub struct FieldValidation {
    pub code: String,
    pub message: Option<String>,
    pub validator: ValidationType,
}

impl FieldValidation {
    pub fn new(validator: ValidationType) -> FieldValidation {
        FieldValidation {
            code: validator.code().to_string(),
            validator,
            message: None,
        }
    }
}
