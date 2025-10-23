use serde::Serialize;
use validator::{ValidateEmail, ValidationError};

#[derive(Debug, Clone, Serialize)]
pub struct UserEmail(String);

impl UserEmail {
    pub fn parse(s: String) -> Result<Self, ValidationError> {
        if <String as ValidateEmail>::validate_email(&s) {
            Ok(UserEmail(s))
        } else {
            // Err("Invalid user email".to_string())
            Err(ValidationError::new("Invalid user email"))
        }
    }
}

impl AsRef<str> for UserEmail {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl PartialEq for UserEmail {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for UserEmail {}

#[cfg(test)]
mod tests {
    use super::*;
    use fake::{faker::internet::raw::SafeEmail, locales::EN, Fake};
    use proptest::prelude::*;
    use rand::{rngs::StdRng, SeedableRng};

    #[test]
    fn test_parse_valid_email() {
        let email = SafeEmail(EN).fake();
        let email = UserEmail::parse(email);
        assert!(email.is_ok());
    }

    #[test]
    fn test_parse_invalid_email() {
        let email = UserEmail::parse("invalid_email".to_string());
        assert!(email.is_err());
    }
    #[test]
    fn email_missing_at_symbol_is_rejected() {
        let email = UserEmail::parse("testexample.com".to_string());
        assert!(email.is_err());
    }

    #[test]
    fn email_missing_subjected_is_rejected() {
        let email = UserEmail::parse("@examplecom".to_string());
        assert!(email.is_err());
    }

    // prop tests
    #[derive(Clone, Debug)]
    struct UserEmailFixture(pub String);

    fn safe_email_strategy() -> impl Strategy<Value = UserEmailFixture> {
        any::<u64>().prop_map(|seed| {
            let mut rng = StdRng::seed_from_u64(seed);
            let email = SafeEmail(EN).fake_with_rng(&mut rng);
            // println!("Generated Email: {}", email);
            UserEmailFixture(email)
        })
    }

    proptest! {
        #[test]
        fn valid_emails_are_parsed_successfully(email in safe_email_strategy()) {
            let email = UserEmail::parse(email.0);
            assert!(email.is_ok());
        }
    }
}
