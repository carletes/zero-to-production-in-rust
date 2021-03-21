use validator::validate_email;

#[derive(Debug)]
pub struct SubscriberEmail(String);

impl SubscriberEmail {
    pub fn parse(s: String) -> Result<SubscriberEmail, String> {
        if validate_email(&s) {
            Ok(Self(s))
        } else {
            Err(format!("Invalid email {}", s))
        }
    }
}

impl AsRef<str> for SubscriberEmail {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::SubscriberEmail;
    use claim::assert_err;
    use fake::{faker::internet::en::SafeEmail, Fake};

    #[derive(Clone, Debug)]
    struct ValidEmail(pub String);

    impl quickcheck::Arbitrary for ValidEmail {
        fn arbitrary<G: quickcheck::Gen>(g: &mut G) -> Self {
            let email = SafeEmail().fake_with_rng(g);
            Self(email)
        }
    }

    #[quickcheck_macros::quickcheck]
    fn valid_emails_are_accepted(email: ValidEmail) -> bool {
        SubscriberEmail::parse(email.0).is_ok()
    }

    #[test]
    fn empty_email_string_is_rejected() {
        let email = "".to_string();
        assert_err!(SubscriberEmail::parse(email));
    }

    #[test]
    fn email_missing_at_symbol_is_rejected() {
        let email = "foo".to_string();
        assert_err!(SubscriberEmail::parse(email));
    }

    #[test]
    fn email_missing_addressee_is_rejected() {
        let email = "@example.com".to_string();
        assert_err!(SubscriberEmail::parse(email));
    }
}
