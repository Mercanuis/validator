//! validation
//!
//! This library is the main validation rules
//!
//! Module `validation` contains the main traits used for validation
//!
//! Module `error` contains main logic for handling validation errors
//!
//! All other modules should be considered the validation rules
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate uuid;

pub use crate::validation::{FieldValidation, StateValidation, Validation, ValidationResult};
pub use error::{ValidationError, ValidationErrorResponse};
pub use is_in_collection::is_in_collection;
pub use not_null::is_not_null;

pub mod error;
pub mod validation;

mod is_in_collection;
mod not_null;
