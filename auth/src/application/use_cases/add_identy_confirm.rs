use crate::{
    application::port::driven::{
        auth_cache::{AuthCacheTrait, AddIdentificationRequest}, 
        auth_repository::{AuthRepositoryTrait, UpdateIdentify},
    }, 
    domain::{types::{token_data::TokenData, identification::NewIdentification, code::Code}, auth::Auth},
};


#[derive(Debug)]
pub enum UpdateError {
    Unauthorized(String),
    Unknown(String),
    InvalidData(String),
}

pub struct Payload {
    pub transaction_id: String,
    pub confirmation_code: Code,
}

pub async fn execute<T, U>(
    conn: &T, 
    cache_conn: &U, 
    repo: &impl AuthRepositoryTrait<T>,
    repo_cache: &impl AuthCacheTrait<U>,
    secret: &[u8],
    token: &String,
    payload: Payload,
) -> Result<Auth, UpdateError> {
    // verify token is valid
    if TokenData::from_token(token, &secret).is_err() {
        return Err(UpdateError::Unauthorized("Invalid token".to_string()));
    };
    // validate confirmation code
    let add_identify: AddIdentificationRequest = match repo_cache
        .find_by_id::<AddIdentificationRequest>(cache_conn, payload.transaction_id.clone()).await
    {
        Ok(update) => match update {
            Some(update) => {
                if update.confirmation_code == payload.confirmation_code {
                    update
                } else {
                    return Err(UpdateError::InvalidData("invalid confirmation code".to_string()));
                }
            },
            None => return Err(UpdateError::InvalidData("invalid transaction id".to_string())),
        },
        Err(error) => return Err(UpdateError::Unknown(format!("Unknown error: {:?}", error))),
    };
    // delete cache
    match repo_cache.delete(cache_conn, payload.transaction_id).await {
        Ok(_) => (),
        Err(error) => return Err(UpdateError::Unknown(format!("Unknown error: {:?}", error))),
    };
    // add identification to user
    let identification_operation = UpdateIdentify::Add(NewIdentification {
        user_id: add_identify.user_id,
        identification_value: add_identify.identity,
    });
    match repo.update_identifications(conn, identification_operation).await {
        Ok(user) => Ok(user),
        Err(error) => Err(UpdateError::Unknown(format!("Unknown error: {:?}", error.to_string()))),
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
