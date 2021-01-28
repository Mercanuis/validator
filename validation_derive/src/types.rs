//! types
//!
//! Module that stores the enum for all validation rules in the derive
//! This enum is meant be a way to help with labeling the rule that
//! the derive should find and mapping what it attributes to

///Enum providing the mapping that allows the derive to determine which validation rule to generate
#[derive(Clone, Debug, PartialEq)]
pub enum ValidationType {
    //Indicates that the field cannot be None, or 'null' in the case of a DTO field.
    NotNull,
}

impl ValidationType {
    pub fn code(&self) -> &'static str {
        match *self {
            ValidationType::NotNull => "not_null",
        }
    }
}
