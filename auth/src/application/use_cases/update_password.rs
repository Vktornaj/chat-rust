use crate::{
    application::port::driven::auth_repository::AuthRepositoryTrait, 
    domain::{auth::Auth, types::{password::Password, token_data::TokenData}},
};


#[derive(Debug)]
pub enum UpdateError {
    NotFound(String),
    Unauthorized(String),
    Unknown(String),
    Conflict(String),
    InvalidData(String),
}

pub struct Payload {
    pub password: Password,
    pub new_password: Password,
}

pub async fn execute<T>(
    conn: &T, 
    repo: &impl AuthRepositoryTrait<T>, 
    secret: &[u8],
    token: &String,
    payload: Payload,
) -> Result<Auth, UpdateError> {
    if payload.password == payload.new_password {
        return Err(UpdateError::Conflict("New password is the same as old password".to_string()));
    }
    // verify user exist and token is valid
    let user_id = if let Ok(auth) = TokenData::from_token(token, &secret) {
        auth.id
    } else {
        return Err(UpdateError::Unauthorized("Invalid token".to_string()));
    };
    // verify user exists and password match
    if let Ok(auth) = repo.find_by_id(conn, user_id.into()).await {
        if payload.password.verify_password(&auth.hashed_password).is_err() {
            return Err(UpdateError::Unauthorized("Invalid password".to_string()));
        }
    } else {
        return Err(UpdateError::NotFound("User not found".to_string()));
    };
    // hash new password
    let new_hashed_password = if let Ok(hashed_password) = payload.new_password.hash_password() {
        hashed_password
    } else {
        return Err(UpdateError::Unknown("Unknown error".to_string()));
    };
    // update password
    match repo.update_password(conn, user_id, new_hashed_password).await {
        Ok(user) => Ok(user),
        Err(e) => Err(UpdateError::Unknown(format!("{:?}", e.to_string()))),
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