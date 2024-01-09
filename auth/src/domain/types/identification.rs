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
            Self::Email(_) => "email".to_string(),
            Self::PhoneNumber(_) => "phone_number".to_string(),
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

    pub fn from_string(value: String, identifier_type: String) -> Result<Self, String> {
        match identifier_type.as_str() {
            "email" => Ok(Self::Email(Email::try_from(value)
                .map_err(|err| err.to_string())?)),
            "phone_number" => Ok(Self::PhoneNumber(PhoneNumber::try_from(value)
                .map_err(|err| err.to_string())?)),
            _ => Err("Invalid identification value type".to_string()),
        }
    }
}

impl TryFrom<String> for IdentificationValue {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if Email::try_from(value.to_owned()).is_ok() {
            return Ok(Self::Email(Email::try_from(value).map_err(|err| err.to_string())?));
        }
        if PhoneNumber::try_from(value.to_owned()).is_ok() {
            return Ok(Self::PhoneNumber(PhoneNumber::try_from(value).map_err(|err| err.to_string())?));
        }
        Err("Invalid identification value".to_string())
    }
}

impl PartialEq<IdentificationValue> for IdentificationValue {
    fn eq(&self, other: &IdentificationValue) -> bool {
        self.get_value() == other.get_value()
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