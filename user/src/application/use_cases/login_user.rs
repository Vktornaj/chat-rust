use crate::{
    domain::types::{
        email::Email, phone_number::PhoneNumber, password::{Password, self}
    }, 
    application::port::driven::user_repository::FindUser
};

use super::super::port::driven::user_repository::UserRepositoryTrait;
use auth::domain::auth::Auth;


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
    repo: &impl UserRepositoryTrait<T>, 
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
    let find_user = FindUser { 
        email, 
        phone_number, 
        ..Default::default() 
    };
    if let Ok(mut users) = repo.find_by_criteria(conn, find_user, 0, 1).await {
        if users.len() < 1 {
            return Err(LoginError::NotFound);
        }
        let user = users.swap_remove(0);
        if password.verify_password(&user.hashed_password).is_ok() {
            Ok(Auth::new(&user.id.into()).token(secret))
        } else  {
            Err(LoginError::Unauthorized)
        }
    } else {
        Err(LoginError::NotFound)
    }
}

