use serde::{Serialize, Deserialize};

use crate::domain::user::User;


#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserJson {
    pub email: Option<String>,
    pub phone_number: Option<String>,
    pub first_name: String,
    pub last_name: String,
    pub nationality: String,
    pub languages: Vec<String>,
}

impl UserJson {
    pub fn from_user(user: User) -> Self {
        UserJson { 
            email: user.email.map(|x| x.into()),
            phone_number: user.phone_number.map(|x| x.into()),
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
    pub email: Option<String>,
    pub phone_number: Option<String>,
    pub password: String,
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

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserContactInfo {
    #[serde(
        default,                                    // <- important for deserialization
        skip_serializing_if = "Option::is_none",    // <- important for serialization
        with = "::serde_with::rust::double_option",
    )]
    pub email: Option<Option<String>>,
    #[serde(
        default,                                    // <- important for deserialization
        skip_serializing_if = "Option::is_none",    // <- important for serialization
        with = "::serde_with::rust::double_option",
    )]
    pub phone_number: Option<Option<String>>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Credentials {
    pub email: Option<String>,
    pub phone_number: Option<String>,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JsonToken {
    pub authorization_token: String,
    pub token_type: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Credentials2 {
    pub password: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Credentials3 {
    pub password: String,
    pub new_password: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ValidTransaction {
    pub transaction_id: String,
    pub confirmation_code: u32,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IdTransaction {
    pub id_transaction: String,
}