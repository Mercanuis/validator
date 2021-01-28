use crate::error::ValidationError;

pub type ValidationResult<T> = std::result::Result<T, ValidationError>;

/// Trait that implements a validation routine for a system/subsystem
///
/// The idea behind it is to
///
///  - Encourage common error handling among multiple systems
///  - Enforce systems to have a common method to validate state of a system
///  - Provide a common pattern that is 'front-end'. That is, that any system
///    would run this validation before allowing any transactions or operations
///    in the system, providing the safety that this object is 'safe' for consumption
///
/// `Validation` can be as complex as needed, but we enforce only two types (for now)
///
/// <br>
///
/// # State Validation
///
/// `StateValidation` is a trait to provide validation handling for any state of a given payload
/// This is the recommended trait to use if you need to validate on an entire state of a given DTO
/// This trait is meant to be implemented by the caller; if you want state validation, you will need
/// to provide the 'rules' as to what makes something valid. The validation cannot make these
/// determinations ahead of time because every caller's system and determination of valid state is different.
///
/// <br>
///
/// # Field Validation
///
/// `FieldValidation` is a trait to provide field validation. Unlike State validation, this can be done
/// on a field-by-field basis leveraging the syn library that helps parse Rust syntax. Any errors or validations
/// will be based on the passed by rules from the caller
///
/// TODO: Update as lib is fixed and tweaked
#[cfg_attr(doc_fc, doc(cfg(any(feature = "full"))))]
pub trait Validation: StateValidation + FieldValidation {
    fn validate(&self) -> ValidationResult<()>;
}
impl<T: Validation> Validation for &T {
    fn validate(&self) -> ValidationResult<()> {
        T::validate(*self)
    }
}

pub trait StateValidation {
    /// Provides a `ValidationResult` of the validity of the state of a struct
    /// The idea is to ensure that a struct is 'safe' for consumption of lower systems
    fn validate_state(&self) -> ValidationResult<()>;
}

impl<T: StateValidation> StateValidation for &T {
    fn validate_state(&self) -> ValidationResult<()> {
        T::validate_state(*self)
    }
}

pub trait FieldValidation {
    /// Provides the fields that the struct requires validation upon
    /// Typically this is custom per the structure's field, some structs will require
    /// different validation (or, none at all should the user wish it)
    fn validate_fields(&self) -> ValidationResult<()>;
}

impl<T: FieldValidation> FieldValidation for &T {
    fn validate_fields(&self) -> ValidationResult<()> {
        T::validate_fields(*self)
    }
}
