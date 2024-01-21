use serde::{Serialize, Deserialize};

use crate::{
    domain::user::User, types::{
        birthday::Birthday, 
        language::Language, 
        first_name::FirstName, 
        last_name::LastName, 
        nationality::Nationality,
    }
};


#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserJson {
    pub firstname: Option<FirstName>,
    pub lastname: Option<LastName>,
    pub nationality: Option<Nationality>,
    pub birthday: Option<Birthday>,
    pub languages: Option<Vec<Language>>,
}

impl UserJson {
    pub fn from_user(user: User) -> Self {
        UserJson { 
            firstname: Some(user.first_name.into()), 
            lastname: Some(user.last_name.into()),
            nationality: Some(user.nationality.into()),
            birthday: Some(user.birthday.into()),
            languages: Some(user.languages.into()),
        }
    }
}