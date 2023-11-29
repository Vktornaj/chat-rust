use chrono::{DateTime, Utc};

use common::domain::types::{
    id::Id, 
    email::Email,
    phone_number::PhoneNumber,
};


pub enum ContactValue {
    Email(Email),
    PhoneNumber(PhoneNumber),
}

pub struct Identification {
    pub id: Id,
    pub user_id: Id,
    pub identification_value: ContactValue,
    pub created_at:  DateTime<Utc>,
    pub updated_at:  DateTime<Utc>,
}