use auth::domain::auth::Auth;
use chrono::{DateTime, Utc};
use common::domain::types::error::ErrorMsg;

use super::super::port::driven::user_repository::UserRepositoryTrait;
use crate::{
    domain::{
        user::User, types::{
            first_name::FirstName, 
            last_name::LastName, 
            birthday::Birthday, 
            nationality::Nationality, 
            language::Language,
        }
    }, 
    application::port::driven::user_repository::UpdateUser
};


#[derive(Debug)]
pub enum UpdateError {
    NotFound,
    Unautorized,
    InvalidData(String),
    Unknown(String),
    Conflict(String),
}

pub struct Payload {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub birthday: Option<DateTime<Utc>>,
    pub nationality: Option<String>,
    pub languages: Option<Vec<String>>,
}

pub async fn execute<T>(
    conn: &T, 
    repo: &impl UserRepositoryTrait<T>,
    secret: &[u8],
    token: &String,
    payload: Payload,
) -> Result<User, UpdateError> {
    // validate data
    let first_name = if let Some(first_name) = payload.first_name {
        match FirstName::try_from(first_name) {
            Ok(first_name) => Some(first_name),
            Err(e) => return Err(UpdateError::InvalidData(e.to_string())),
        }
    } else {
        None
    };
    let last_name = if let Some(last_name) = payload.last_name {
        match LastName::try_from(last_name) {
            Ok(last_name) => Some(last_name),
            Err(e) => return Err(UpdateError::InvalidData(e.to_string())),
        }
    } else {
        None
    };
    let birthday = if let Some(birthday) = payload.birthday {
        match Birthday::try_from(birthday) {
            Ok(birthday) => Some(birthday),
            Err(e) => return Err(UpdateError::InvalidData(e.to_string())),
        }
    } else {
        None
    };
    let nationality = if let Some(nationality) = payload.nationality {
        match Nationality::try_from(nationality) {
            Ok(nationality) => Some(nationality),
            Err(e) => return Err(UpdateError::InvalidData(e.to_string())),
        }
    } else {
        None
    };
    let languages = if let Some(languages) = payload.languages {
        let languages: Result<Vec<Language>, ErrorMsg> = languages.into_iter()
            .map(|x| Language::try_from(x)).collect();
        match languages {
            Ok(languages) => Some(languages),
            Err(e) => return Err(UpdateError::InvalidData(e.to_string())),
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
    // verify user exists and data is not the same
    if let Ok(user) = repo.find_by_id(conn, id.into()).await {
        if Some(user.first_name) == first_name {
            return Err(UpdateError::Conflict("first_name is the same".into()));
        }
        if Some(user.last_name) == last_name {
            return Err(UpdateError::Conflict("last_name is the same".into()));
        }
        if Some(user.birthday) == birthday {
            return Err(UpdateError::Conflict("birthday is the same".into()));
        }
        if Some(user.nationality) == nationality {
            return Err(UpdateError::Conflict("nationality is the same".into()));
        }
        if Some(user.languages) == languages {
            return Err(UpdateError::Conflict("languages is the same".into()));
        }
    } else {
        return Err(UpdateError::NotFound);
    };
    // update only user not sensitive info
    let user_update = UpdateUser {
        id: id.into(),
        first_name,
        last_name,
        birthday,
        nationality,
        languages,
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