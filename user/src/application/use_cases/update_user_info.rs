use auth::domain::auth::Auth;

use super::{super::port::driven::user_repository::UserRepositoryTrait, utils};
use crate::{domain::user::User, application::port::driven::user_repository::UpdateUser};


#[derive(Debug)]
pub enum UpdateError {
    NotFound,
    Unautorized,
    Unknown(String),
    Conflict(String),
}

pub async fn execute<T>(
    conn: &T, 
    repo: &impl UserRepositoryTrait<T>,
    update_user: UpdateUser,
    secret: &[u8],
    token: &String
) -> Result<User, UpdateError> {
    // verify user exist and token is valid
    let id = if let Ok(auth) = Auth::from_token(token, &secret) {
        auth.id
    } else {
        return Err(UpdateError::Unautorized);
    };
    // verify user exists
    if repo.find_by_id(conn, id.into()).await.is_err() {
        return Err(UpdateError::NotFound);
    };
    // update only user not sensitive info
    let user_update = UpdateUser {
        id: id.into(),
        first_name: update_user.first_name,
        last_name: update_user.last_name,
        birthday: update_user.birthday,
        nationality: update_user.nationality,
        languages: update_user.languages,
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