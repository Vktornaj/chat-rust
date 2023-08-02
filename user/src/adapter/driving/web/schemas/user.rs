use chrono::{Utc, TimeZone, ParseError};
use serde::{Serialize, Deserialize};

use common::config::DATE_FORMAT;
use crate::{domain::user::User, application::port::driven::user_repository::NewUser};


#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserJson {
    pub first_name: String,
    pub last_name: String,
}

impl UserJson {
    pub fn from_user(user: User) -> Self {
        UserJson { 
            first_name: user.first_name.unwrap_or("".to_string()), 
            last_name: user.last_name.unwrap_or("".to_string())
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewUserJson {
    pub email: Option<String>,
    pub phone_number: Option<String>,
    pub password: String,
    pub first_name: String,
    pub last_name: String,
    pub birthday: String,
    pub nationality: String,
    pub languages: Vec<String>,
}

impl NewUserJson {
    pub fn to_new_user(&self) -> Result<NewUser, ParseError> {
        Ok(NewUser {
            email: self.email.clone(),
            phone_number: self.phone_number.clone(),
            password: self.password.clone(),
            first_name: self.first_name.clone(),
            last_name: self.last_name.clone(),
            birthday: Utc.datetime_from_str(&self.birthday, DATE_FORMAT)?,
            nationality: self.nationality.clone(),
            languages:  self.languages.clone(),
        })
    }
}