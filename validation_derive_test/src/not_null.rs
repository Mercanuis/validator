#[derive(Serialize)]
struct DumbObject {
    name: String,
}

#[derive(FieldValidate)]
struct Required {
    #[validate(not_null)]
    val: Option<DumbObject>,
    #[validate(not_null)]
    val_2: Option<DumbObject>,
}

#[cfg(test)]
mod tests {
    use crate::not_null::{DumbObject, Required};
    use validation::FieldValidation;

    #[test]
    fn validate_not_null_successful() {
        let r = Required {
            val: Some(DumbObject {
                name: String::new(),
            }),
            val_2: Some(DumbObject {
                name: String::new(),
            }),
        };

        assert!(r.validate_fields().is_ok());
    }

    #[test]
    fn validate_not_null_failed_one_null() {
        let r = Required {
            val: Some(DumbObject {
                name: String::new(),
            }),
            val_2: None,
        };

        assert!(r.validate_fields().is_err());
    }

    #[test]
    fn validate_not_null_failed_both_null() {
        let r = Required {
            val: None,
            val_2: None,
        };

        assert!(r.validate_fields().is_err());
    }
}
