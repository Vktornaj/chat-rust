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
            email: user.email.map(|x| x.into()),
            phone_number: user.phone_number.map(|x| x.into()),
            first_name: user.first_name.map(|x| x.into()), 
            last_name: user.last_name.map(|x| x.into()),
            nationality: Some(user.nationality.into()),
            languages: user.languages.map(|x| x.into_iter()
                .map(|x| x.into()).collect()),
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
            password: Some(self.password.clone()),
            hashed_password: None,
            first_name: self.first_name.clone(),
            last_name: self.last_name.clone(),
            birthday: Utc.datetime_from_str(&self.birthday, DATE_FORMAT)?,
            nationality: self.nationality.clone(),
            languages:  self.languages.clone(),
        })
    }
}