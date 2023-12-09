use crate::{
    application::port::driven::auth_repository::{AuthRepositoryTrait, UpdateAuth}, 
    domain::{auth::Auth, types::{password::Password, token_data::TokenData}},
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
    pub password: String,
    pub new_password: String,
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
    // validate payload
    let password = if let Ok(password) = Password::try_from(payload.password) {
        password
    } else {
        return Err(UpdateError::Unautorized);
    };
    let new_password = if let Ok(new_password) = Password::try_from(payload.new_password) {
        new_password
    } else {
        return Err(UpdateError::InvalidData("Invalid new password".to_string()));
    };
    // verify user exist and token is valid
    let id = if let Ok(auth) = TokenData::from_token(token, &secret) {
        auth.id
    } else {
        return Err(UpdateError::Unautorized);
    };
    // verify user exists and password match
    if let Ok(auth) = repo.find_by_id(conn, id.into()).await {
        if password.verify_password(&auth.hashed_password).is_err() {
            return Err(UpdateError::Unautorized);
        }
    } else {
        return Err(UpdateError::NotFound);
    };
    // hash new password
    let new_hashed_password = if let Ok(hashed_password) = new_password.hash_password() {
        Some(hashed_password)
    } else {
        return Err(UpdateError::Unknown("Unknown error".to_string()));
    };
    // update password
    let user_update = UpdateAuth { id: id.into(), new_hashed_password, ..Default::default() };
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