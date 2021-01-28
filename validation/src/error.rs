use std::fmt::{Display, Formatter, Result};

const BAD_REQUEST: i32 = 400;
const UNPROCESSABLE_ENTITY: i32 = 422;

/// Describes a validation error in the system
/// A validation error can occur in two ways
///
///  - An invalid value was passed to the system
///  - An invalid state was passed to the system
///
/// The struct is meant to provide a common language amongst interconnected
/// systems/microservices to describe a validation error
#[derive(Debug, PartialOrd, PartialEq)]
pub enum ValidationError {
    FieldMismatch(String),
    InvalidState(String),
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ValidationErrorResponse {
    pub error_code: i32,
    pub error_message: String,
}

impl ValidationErrorResponse {
    /// A default implementation that returns a `ValidationError` with 500 - Internal Server Error
    ///
    /// # Example
    /// ```
    /// use crate::validation::ValidationErrorResponse;
    /// let err = ValidationErrorResponse::default();
    /// assert_eq!(500, err.error_code);
    /// assert_eq!("Internal Server Error".to_string(), err.error_message);
    /// ```
    pub fn default() -> Self {
        ValidationErrorResponse {
            error_code: 500,
            error_message: "Internal Server Error".to_string(),
        }
    }

    /// Creates a new `ValidationError`
    ///
    /// # Arguments
    ///
    ///  * `error_code` - i32 HTTP Status code
    ///  * `error_message` - String representing the custom error message
    ///
    /// # Example
    /// ```
    /// use crate::validation::ValidationErrorResponse;
    /// let err = ValidationErrorResponse::new(400, "Message".to_string());
    /// assert_eq!(400, err.error_code);
    /// assert_eq!("Message".to_string(), err.error_message);
    /// ```
    pub fn new(error_code: i32, error_message: String) -> Self {
        ValidationErrorResponse {
            error_code,
            error_message,
        }
    }
}

impl From<ValidationError> for ValidationErrorResponse {
    fn from(e: ValidationError) -> Self {
        match e {
            ValidationError::FieldMismatch(ref str) => {
                ValidationErrorResponse::new(BAD_REQUEST, str.to_string())
            }
            ValidationError::InvalidState(ref str) => {
                ValidationErrorResponse::new(UNPROCESSABLE_ENTITY, str.to_string())
            }
        }
    }
}

impl Display for ValidationError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match *self {
            ValidationError::FieldMismatch(ref err) => err.fmt(f),
            ValidationError::InvalidState(ref err) => err.fmt(f),
        }
    }
}

impl std::error::Error for ValidationError {}

#[cfg(test)]
mod tests {
    use crate::error::*;

    #[test]
    fn test_from_validation_error_field_mismatch() {
        let err = ValidationError::FieldMismatch("Bad String".to_string());
        let resp = ValidationErrorResponse::from(err);
        assert_eq!(BAD_REQUEST, resp.error_code);
        assert_eq!("Bad String", resp.error_message);
    }

    #[test]
    fn test_from_validation_error_invalid_state() {
        let err = ValidationError::InvalidState("Bad Payload".to_string());
        let resp = ValidationErrorResponse::from(err);
        assert_eq!(UNPROCESSABLE_ENTITY, resp.error_code);
        assert_eq!("Bad Payload", resp.error_message);
    }
}
