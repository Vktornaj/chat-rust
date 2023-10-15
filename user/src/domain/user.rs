use chrono::{DateTime, Utc};
use common::types::{error::ErrorMsg, email::Email, phone_number::PhoneNumber, id::Id};


use super::types::{
    first_name::FirstName, 
    last_name::LastName, 
    birthday::Birthday, 
    nationality::Nationality, 
    language::Language, 
    password::Password,
};


#[derive(Clone)]
pub struct User {
    pub id: Id,
    pub email: Option<Email>,
    pub phone_number: Option<PhoneNumber>,
    pub hashed_password: String,
    pub first_name: FirstName,
    pub last_name: LastName,
    pub birthday: Birthday,
    pub nationality: Nationality,
    pub languages: Vec<Language>,
    pub created_at:  DateTime<Utc>,
    pub updated_at:  DateTime<Utc>,
}

pub struct NewUser {
    pub email: Option<Email>,
    pub phone_number: Option<PhoneNumber>,
    // you need to hash the password type Password before storing it
    pub hashed_password: String,
    pub first_name: FirstName,
    pub last_name: LastName,
    pub birthday: Birthday,
    pub nationality: Nationality,
    pub languages: Vec<Language>,
}

impl NewUser {
    pub fn new(
        email: Option<String>,
        phone_number: Option<String>,
        password: String,
        first_name: String,
        last_name: String,
        birthday: DateTime<Utc>,
        nationality: String,
        languages: Vec<String>,
    ) -> Result<Self, ErrorMsg> {
        if email.is_none() && phone_number.is_none() {
            return Err(ErrorMsg("email or phone number must be provided".to_string()));
        }
        let languages: Result<Vec<Language>, ErrorMsg> = languages.into_iter()
            .map(|x| Language::try_from(x))
            .collect();
        let hashed_password = match Password::try_from(password)?.hash_password() {
            Ok(hashed_password) => hashed_password,
            Err(e) => return Err(ErrorMsg(e.to_string())),
        };
        Ok(NewUser { 
            email: email.map(|x| Email::try_from(x)).transpose()?, 
            phone_number: phone_number.map(|x| PhoneNumber::try_from(x)).transpose()?, 
            hashed_password, 
            first_name: FirstName::try_from(first_name)?, 
            last_name: LastName::try_from(last_name)?, 
            birthday: Birthday::try_from(birthday)?, 
            nationality: Nationality::try_from(nationality)?, 
            languages: languages?,
        })
    }
}