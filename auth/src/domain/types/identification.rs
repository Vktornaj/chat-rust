use chrono::{DateTime, Utc};

use common::domain::types::{
    id::Id, 
    email::Email,
    phone_number::PhoneNumber,
};


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
}

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