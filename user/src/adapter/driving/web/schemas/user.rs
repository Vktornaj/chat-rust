use chrono::{Utc, TimeZone};
use serde::{Serialize, Deserialize};

use common::config::DATE_FORMAT;
use crate::{domain::user::{User, Email, ErrorMsg, PhoneNumber, Password, FirstName, LastName, Birthday, Nationality, Language}, application::port::driven::user_repository::NewUser};


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
    pub fn to_new_user(self) -> Result<NewUser, ErrorMsg> {
        let date = Utc.datetime_from_str(&self.birthday, DATE_FORMAT).unwrap();
        let languages: Result<Vec<Language>, ErrorMsg> = self.languages.into_iter()
            .map(|x| Language::try_from(x))
            .collect();

        Ok(NewUser {
            email: self.email.map(|x| Email::try_from(x)).transpose()?,
            phone_number: self.phone_number.map(|x| PhoneNumber::try_from(x)).transpose()?,
            password: Some(Password::try_from(self.password)?),
            hashed_password: None,
            first_name: FirstName::try_from(self.first_name)?,
            last_name: LastName::try_from(self.last_name)?,
            birthday: Birthday::try_from(date)?,
            nationality: Nationality::try_from(self.nationality)?,
            languages: languages?,
            
        })
    }
}