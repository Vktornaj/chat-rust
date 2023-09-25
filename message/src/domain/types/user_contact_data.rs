use user::types::{
    email::Email,
    phone_number::PhoneNumber
};
use super::error::ErrorMsg;


pub enum UserContactData {
    Email(Email),
    PhoneNumber(PhoneNumber),
}

impl TryFrom<String> for UserContactData {
    type Error = ErrorMsg;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if let Ok(email) = Email::try_from(value.clone()) {
            return Ok(Self::Email(email))
        } else if let Ok(phone_number) = PhoneNumber::try_from(value.clone()) {
            return Ok(Self::PhoneNumber(phone_number))
        } else {
            return Err(ErrorMsg("Invalid user contact data".to_string()))
        }
    }
}