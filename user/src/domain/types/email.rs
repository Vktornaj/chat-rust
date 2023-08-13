use regex::Regex;
use super::error::ErrorMsg;


#[derive(PartialEq, Debug, Clone)]
pub struct Email(String);

impl TryFrom<String> for Email {
    type Error = ErrorMsg;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() {
            return Err(ErrorMsg("Email is empty".to_string()))
        }
        let re = Regex::new(r"^[a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\.[a-zA-Z0-9-.]+$").unwrap();
        if !re.is_match(&value) {
            return Err(ErrorMsg("Email is invalid".to_string()))
        }
        Ok(Self(value))
    }
}

impl From<Email> for String {
    fn from(email: Email) -> Self {
        email.0
    }
}

#[cfg(test)]
mod tests_email {
    use super::*;

    #[test]
    fn test_email() {
        let email = Email::try_from("some@some.some".to_string());
        assert!(email.is_ok());
        let email = Email::try_from("some@some".to_string());
        assert!(email.is_err());
    }
}