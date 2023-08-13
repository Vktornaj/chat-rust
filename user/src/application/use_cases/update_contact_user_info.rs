use auth::domain::auth::Auth;

use super::super::port::driven::user_repository::UserRepositoryTrait;
use crate::{
    domain::{
        user::User, types::{
            password::Password, email::Email, phone_number::PhoneNumber
        }
    }, 
    application::port::driven::user_repository::UpdateUser
};


#[derive(Debug)]
pub enum UpdateError {
    NotFound,
    Unautorized,
    Unknown(String),
    Conflict(String),
    InvalidData(String),
}

pub struct Payload {
    password: String,
    email: Option<String>,
    phone_number: Option<String>,
}

pub async fn execute<T>(
    conn: &T, 
    repo: &impl UserRepositoryTrait<T>, 
    secret: &[u8],
    token: &String,
    payload: Payload,
) -> Result<User, UpdateError> {
    let password = if let Ok(password) = Password::try_from(payload.password) {
        password
    } else {
        return Err(UpdateError::Unautorized);
    };
    let email = if let Some(email) = payload.email {
        if let Ok(email) = Email::try_from(email) {
            Some(Some(email))
        } else {
            return Err(UpdateError::InvalidData("Invalid email".to_string()));
        }
    } else {
        None
    };
    let phone_number = if let Some(phone_number) = payload.phone_number {
        if let Ok(phone_number) = PhoneNumber::try_from(phone_number) {
            Some(Some(phone_number))
        } else {
            return Err(UpdateError::InvalidData("Invalid phone number".to_string()));
        }
    } else {
        None
    };
    // verify user exist and token is valid
    let id = if let Ok(auth) = Auth::from_token(token, &secret) {
        auth.id
    } else {
        return Err(UpdateError::Unautorized);
    };
    // verify user exists and password match
    if let Ok(user) = repo.find_by_id(conn, id.into()).await {
        if password.verify_password(&user.hashed_password).is_err() {
            return Err(UpdateError::Unautorized);
        }
    } else {
        return Err(UpdateError::NotFound);
    };
    // update sensitive info
    let user_update = UpdateUser {
        id: id.into(),
        email,
        phone_number,
        ..Default::default()
    };
    match repo.update(conn, user_update).await {
        Ok(user) => Ok(user),
        Err(e) => Err(UpdateError::Unknown(format!("{:?}", e))),
    }
}

// #[cfg(test)]
// mod tests {
//     use std::sync::Mutex;

//     // use crate::repositories::pokemon::InMemoryRepository;
//     use chrono::{Utc, NaiveDate};
//     use rocket::tokio;
//     use uuid::Uuid;
//     use super::*;
//     use crate::{
//         application::{
//             port::driven::{
//                 user_repository::NewUser, 
//                 errors::RepoSelectError
//             }, 
//             use_cases::create_user
//         }, 
//         domain::user::Id
//     };
//     use crate::adapter::driven::persistence::in_memory_repository::InMemoryRepository;

    
// }