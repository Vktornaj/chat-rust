use chrono::{DateTime, Utc};
use uuid::Uuid;


pub enum Error {
    Invalid,
    Unknown
}

pub struct User {
    pub id: Option<Uuid>,
    pub email: Option<String>,
    pub phone_number: Option<String>,
    pub password: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub birthday: Option<DateTime<Utc>>,
    pub nationality: String,
    pub languages: Option<Vec<String>>,
    pub created_at:  Option<DateTime<Utc>>,
    pub updated_at:  Option<DateTime<Utc>>,
}

// TODO: add validations
impl User {
    pub fn new(
        id: Option<Uuid>,
        email: Option<String>,
        phone_number: Option<String>,
        password: String,
        first_name: Option<String>,
        last_name: Option<String>,
        birthday: Option<DateTime<Utc>>,
        nationality: String,
        languages: Option<Vec<String>>,
        created_at:  Option<DateTime<Utc>>,
        updated_at:  Option<DateTime<Utc>>,
    ) -> Result<Self, Error> {
        Ok(User { 
            id, 
            email, 
            phone_number, 
            password, 
            first_name, 
            last_name, 
            birthday, 
            nationality, 
            languages, 
            created_at, 
            updated_at 
        })
    }
}