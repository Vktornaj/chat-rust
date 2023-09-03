use super::super::port::driven::user_repository::UserRepositoryTrait;
use crate::{
    application::port::driven::user_cache::{UserCacheTrait, UpdateUserCDCache}, 
    domain::user::User
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
    pub confirmation_code: u32,
}

pub async fn execute<T, U>(
    conn: &T, 
    cache_conn: &U, 
    repo: &impl UserRepositoryTrait<T>,
    repo_cache: &impl UserCacheTrait<U>,
    secret: &[u8],
    token: &String,
    payload: Payload,
) -> Result<User, UpdateError> {
    // validate confirmation code
    let update_user = match repo_cache.find_by_id::<UpdateUserCDCache>(cache_conn, payload.transaction_id).await {
        Ok(update) => match update {
            Some(update) => {
                if Into::<u32>::into(update.confirmation_code.clone()) == payload.confirmation_code {
                    update.to_update_user()
                } else {
                    return Err(UpdateError::InvalidData("invalid confirmation code".to_string()));
                }
            },
            None => return Err(UpdateError::InvalidData("invalid transaction id".to_string())),
        },
        Err(error) => return Err(UpdateError::Unknown(format!("Unknown error: {:?}", error))),
    };
    // create user
    match repo.update(conn, update_user).await {
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