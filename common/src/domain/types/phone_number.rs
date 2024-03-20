use std::fmt::Display;

use super::error::ErrorMsg;
use regex::Regex;
use serde::{Deserialize, Serialize};


#[derive(PartialEq, Debug, Clone, Deserialize, Serialize)]
pub struct PhoneNumber(String);

impl TryFrom<String> for PhoneNumber {
    type Error = ErrorMsg;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() {
            return Err(ErrorMsg("Phone number is empty".to_string()))
        }
        let re = Regex::new(r"^\+[0-9]{1,3}[0-9]{3,14}$").unwrap();
        if !re.is_match(&value) {
            return Err(ErrorMsg("Phone number is invalid".to_string()))
        }
        Ok(Self(value))
    }
}

// TODO: implement Display trait

impl From<PhoneNumber> for String {
    fn from(phone_number: PhoneNumber) -> Self {
        phone_number.0
    }
}

impl Display for PhoneNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", PhoneNumber::from(self.clone()))
    }
}

#[cfg(test)]
mod tests_phone_number {
    use super::*;

    #[test]
    fn test_phone_number() {
        let phone_number = PhoneNumber::try_from("+380123456789".to_string());
        assert!(phone_number.is_ok());
        let phone_number = PhoneNumber::try_from("+380123456789012345678901234567890123456".to_string());
        assert!(phone_number.is_err());
        let phone_number = PhoneNumber::try_from("380123456789".to_string());
        assert!(phone_number.is_err());
        let phone_number = PhoneNumber::try_from("+380123a567890".to_string());
        assert!(phone_number.is_err());
    }
}