use chrono::{DateTime, Utc, NaiveDate};

use common::domain::types::{
    error::ErrorMsg,  
    id::Id,
};
use super::types::{
    first_name::FirstName, 
    last_name::LastName, 
    birthday::Birthday, 
    nationality::Nationality, 
    language::Language, 
};


#[derive(Clone)]
pub struct User {
    pub id: Id,
    pub first_name: FirstName,
    pub last_name: LastName,
    pub birthday: Birthday,
    pub nationality: Nationality,
    pub languages: Vec<Language>,
    pub created_at:  DateTime<Utc>,
    pub updated_at:  DateTime<Utc>,
}

pub struct NewUser {
    // you need to hash the password type Password before storing it
    pub user_id: Id,
    pub first_name: FirstName,
    pub last_name: LastName,
    pub birthday: Birthday,
    pub nationality: Nationality,
    pub languages: Vec<Language>,
}

impl NewUser {
    pub fn new(
        user_id: String,
        password: String,
        first_name: String,
        last_name: String,
        birthday: NaiveDate,
        nationality: String,
        languages: Vec<String>,
    ) -> Result<Self, ErrorMsg> {
        let languages: Result<Vec<Language>, ErrorMsg> = languages.into_iter()
            .map(|x| Language::try_from(x))
            .collect();
        Ok(NewUser {
            user_id: Id::try_from(user_id)?,
            first_name: FirstName::try_from(first_name)?, 
            last_name: LastName::try_from(last_name)?, 
            birthday: Birthday::try_from(birthday)?, 
            nationality: Nationality::try_from(nationality)?, 
            languages: languages?,
        })
    }
}