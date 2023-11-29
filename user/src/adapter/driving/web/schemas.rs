use serde::{Serialize, Deserialize};

use crate::domain::user::User;


#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserJson {
    pub first_name: String,
    pub last_name: String,
    pub nationality: String,
    pub languages: Vec<String>,
}

impl UserJson {
    pub fn from_user(user: User) -> Self {
        UserJson { 
            first_name: user.first_name.into(), 
            last_name: user.last_name.into(),
            nationality: user.nationality.into(),
            languages: user.languages.into_iter().map(|x| x.into()).collect(),
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewUserJson {
    pub first_name: String,
    pub last_name: String,
    pub birthday: String,
    pub nationality: String,
    pub languages: Vec<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserInfo {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub birthday: Option<String>,
    pub nationality: Option<String>,
    pub languages: Option<Vec<String>>
}