use crate::{
    application::port::driven::{
        auth_cache::{AuthCacheTrait, AddIdentificationRequest}, 
        auth_repository::{AuthRepositoryTrait, UpdateIdentify},
    }, 
    domain::{types::{token_data::TokenData, identification::NewIdentification}, auth::Auth},
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
    pub transaction_id: String,
    pub confirmation_code: String,
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
        return Err(UpdateError::Unautorized);
    };
    // validate confirmation code
    let add_identify: AddIdentificationRequest = match repo_cache
        .find_by_id::<AddIdentificationRequest>(cache_conn, payload.transaction_id.clone()).await {
        Ok(update) => match update {
            Some(update) => {
                if Into::<String>::into(update.confirmation_code.clone()) == payload.confirmation_code {
                    update.to_update_user()
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
        Err(error) => Err(UpdateError::Unknown(format!("Unknown error: {:?}", error))),
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