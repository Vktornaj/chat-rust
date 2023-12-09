use chrono::{DateTime, Utc};

use common::domain::types::{
    id::Id, 
    email::Email,
    phone_number::PhoneNumber,
};
use serde::{Serialize, Deserialize};


#[derive(Clone, Serialize, Deserialize)]
pub enum IdentificationValue {
    Email(Email),
    PhoneNumber(PhoneNumber),
}

impl IdentificationValue {
    pub fn get_type(&self) -> String {
        match self {
            Self::Email(_) => "EMAIL".to_string(),
            Self::PhoneNumber(_) => "PHONE_NUMBER".to_string(),
        }
    }

    pub fn get_value(&self) -> String {
        match self {
            Self::Email(email) => email.to_string(),
            Self::PhoneNumber(phone_number) => phone_number.to_string(),
        }
    }

    pub fn get_value_as_email(&self) -> Option<Email> {
        match self {
            Self::Email(email) => Some(email.clone()),
            _ => None,
        }
    }

    pub fn get_value_as_phone_number(&self) -> Option<PhoneNumber> {
        match self {
            Self::PhoneNumber(phone_number) => Some(phone_number.clone()),
            _ => None,
        }
    }

    pub fn from_string(value: String, value_type: String) -> Result<Self, String> {
        match value_type.as_str() {
            "EMAIL" => Ok(Self::Email(Email::try_from(value)?)),
            "PHONE_NUMBER" => Ok(Self::PhoneNumber(PhoneNumber::try_from(value)?)),
            _ => Err("Invalid identification value type".to_string()),
        }
    }
}

impl TryFrom<String> for IdentificationValue {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if Email::try_from(value.to_owned()).is_ok() {
            return Ok(Self::Email(Email::try_from(value)?));
        }
        if PhoneNumber::try_from(value.to_owned()).is_ok() {
            return Ok(Self::PhoneNumber(PhoneNumber::try_from(value)?));
        }
        Err("Invalid identification value".to_string())
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Identification {
    pub id: Id,
    pub user_id: Id,
    pub identification_value: IdentificationValue,
    pub created_at:  DateTime<Utc>,
}

pub struct NewIdentification {
    pub user_id: Id,
    pub identification_value: IdentificationValue,
}