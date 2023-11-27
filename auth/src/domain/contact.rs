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

pub struct ContactDetails {
    pub id: Id,
    pub user_id: Id,
    pub contact_value: ContactValue,
    pub created_at:  DateTime<Utc>,
    pub updated_at:  DateTime<Utc>,
}