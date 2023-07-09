use serde::{Serialize, Deserialize};

use crate::domain::user::User;


#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserJson {
    pub username: String,
    pub first_name: String,
    pub last_name: String,
}

impl UserJson {
    pub fn from_user(user: User) -> Self {
        UserJson { 
            username: user.username, 
            first_name: user.first_name.unwrap_or("".to_string()), 
            last_name: user.last_name.unwrap_or("".to_string())
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewUserJson {
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub password: String,
}

impl NewUserJson {
    pub fn to_user(&self) -> User {
        User {
            id: None,
            username: self.username.clone(),
            first_name: Some(self.first_name.clone()),
            last_name: Some(self.last_name.clone()),
            password: self.password.clone(),
        }
    }
}