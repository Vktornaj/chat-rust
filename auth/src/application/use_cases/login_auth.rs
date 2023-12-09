use crate::domain::types::{
    password::Password, 
    identification::IdentificationValue,
    token_data::TokenData,
};

use super::super::port::driven::auth_repository::AuthRepositoryTrait;
use common::domain::types::{email::Email, phone_number::PhoneNumber};


#[derive(Debug)]
pub enum LoginError {
    NotFound,
    Unauthorized,
}

pub struct Payload {
    pub email: Option<String>,
    pub phone_number: Option<String>,
    pub password: String
}

// TODO: improve when criteria will implemented onto the traid
pub async fn execute<T>(
    conn: &T,
    repo: &impl AuthRepositoryTrait<T>, 
    secret: &[u8],
    payload: Payload,
) -> Result<String, LoginError> {
    if payload.email.is_none() && payload.phone_number.is_none() {
        return Err(LoginError::NotFound);
    }
    let email = if let Some(email) = payload.email {
        match Email::try_from(email) {
            Ok(email) => Some(email),
            Err(_) => return Err(LoginError::NotFound)
        }
    } else {
        None
    };
    let phone_number = if let Some(phone_number) = payload.phone_number {
        match PhoneNumber::try_from(phone_number) {
            Ok(phone_number) => Some(phone_number),
            Err(_) => return Err(LoginError::NotFound)
        }
    } else {
        None
    };
    let password = match Password::try_from(payload.password) {
        Ok(password) => password,
        Err(_) => return Err(LoginError::NotFound)
    };
    let find_auth = if let Some(email) = email {
        IdentificationValue::Email(email)
    } else if let Some(phone_number) = phone_number {
        IdentificationValue::PhoneNumber(phone_number)
    } else {
        return Err(LoginError::NotFound);

    };
    if let Ok(user) = repo.find_by_identification(conn, find_auth).await {
        if password.verify_password(&user.hashed_password).is_ok() {
            Ok(TokenData::new(&user.user_id.into()).token(secret))
        } else  {
            Err(LoginError::Unauthorized)
        }
    } else {
        Err(LoginError::NotFound)
    }
}