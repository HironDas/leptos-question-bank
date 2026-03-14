#[derive(serde::Deserialize, serde::Serialize, validator::Validate, Debug, Clone)]
pub struct AddClassInput {
    #[validate(length(
        min = 1,
        max = 50,
        message = "Class name must be between 1 and 50 characters"
    ))]
    pub name: String,
    #[validate(length(
        min = 1,
        max = 120,
        message = "Class name in Bengali must be between 1 and 120 characters"
    ))]
    pub name_bn: String,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, validator::Validate)]
pub struct UpdateClassInput {
    #[validate(range(min = 1, message = "Invalid class ID"))]
    pub class_id: u32,
    #[validate(length(
        min = 1,
        max = 50,
        message = "Class name must be between 1 and 50 characters"
    ))]
    pub name: String,
    #[validate(length(
        min = 1,
        max = 120,
        message = "Class name in Bengali must be between 1 and 120 characters"
    ))]
    pub name_bn: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use leptos::logging;
    use validator::Validate;

    #[test]
    fn test_add_class_input_validation() {
        let valid_input = AddClassInput {
            name: "Class 1".to_string(),
            name_bn: "ক্লাস ১".to_string(),
        };
        assert!(valid_input.validate().is_ok());

        let invalid_input = AddClassInput {
            name: "".to_string(),
            name_bn: "ক্লাস ১".to_string(),
        };
        logging::log!("Testing invalid input: {:?}", invalid_input);
        assert!(invalid_input.validate().is_err());
    }

    #[test]
    fn test_add_class_input_validation_max_length() {
        let long_name = "A".repeat(51);
        let long_name_bn = "ক".repeat(121);

        let invalid_input_name = AddClassInput {
            name: long_name,
            name_bn: "ক্লাস ১".to_string(),
        };
        assert!(invalid_input_name.validate().is_err());

        let invalid_input_name_bn = AddClassInput {
            name: "Class 1".to_string(),
            name_bn: long_name_bn,
        };
        assert!(invalid_input_name_bn.validate().is_err());
    }
}
