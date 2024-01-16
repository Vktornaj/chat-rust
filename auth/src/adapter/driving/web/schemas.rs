use serde::{Serialize, Deserialize};

use crate::domain::types::identification::{Identification, IdentificationValue};


#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IdentificationJson {
    pub value: String,
    pub id_type: String,
}

impl From<Identification> for IdentificationJson {
    fn from(identification: Identification) -> Self {
        let value = match identification.identification_value.clone() {
            IdentificationValue::Email(email) => email.to_string(),
            IdentificationValue::PhoneNumber(phone_number) => phone_number.to_string(),
        };
        let id_type = match identification.identification_value {
            IdentificationValue::Email(_) => "email",
            IdentificationValue::PhoneNumber(_) => "phone_number",
        };
        IdentificationJson {
            value,
            id_type: id_type.to_string(),
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthJson {
    pub identifications: Vec<IdentificationJson>,
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
pub struct UpdatePassword {
    pub password: String,
    pub new_password: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ValidateTransaction {
    pub transaction_id: String,
    pub confirmation_code: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Credentials {
    pub identifier: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UuidWrapper {
    pub uuid: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PasswordJson {
    pub password: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JsonBool {
    pub value: bool,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResOk {
    pub ok: bool,
}