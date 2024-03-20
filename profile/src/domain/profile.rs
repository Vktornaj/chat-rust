use chrono::{DateTime, Utc};

use common::domain::types::id::Id;
use super::types::{
    first_name::FirstName, 
    last_name::LastName, 
    birthday::Birthday, 
    nationality::Nationality, 
    language::Language, 
};


#[derive(Clone)]
pub struct Profile {
    pub id: Id,
    pub first_name: FirstName,
    pub last_name: LastName,
    pub birthday: Birthday,
    pub nationality: Nationality,
    pub languages: Vec<Language>,
    pub created_at:  DateTime<Utc>,
    pub updated_at:  DateTime<Utc>,
}

pub struct NewProfile {
    // you need to hash the password type Password before storing it
    pub user_id: Id,
    pub first_name: FirstName,
    pub last_name: LastName,
    pub birthday: Birthday,
    pub nationality: Nationality,
    pub languages: Vec<Language>,
}
