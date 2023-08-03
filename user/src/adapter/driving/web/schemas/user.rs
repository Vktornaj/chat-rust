use chrono::{Utc, TimeZone, ParseError};
use serde::{Serialize, Deserialize};

use common::config::DATE_FORMAT;
use crate::{domain::user::User, application::port::driven::user_repository::NewUser};


#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserJson {
    pub email: Option<String>,
    pub phone_number: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub nationality: Option<String>,
    pub languages: Option<Vec<String>>,
}

impl UserJson {
    pub fn from_user(user: User) -> Self {
        UserJson { 
            email: user.email,
            phone_number: user.phone_number,
            first_name: user.first_name, 
            last_name: user.last_name,
            nationality: Some(user.nationality),
            languages: user.languages,
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