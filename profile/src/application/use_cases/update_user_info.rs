use std::fmt::Display;

use auth::TokenData;

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
    Unauthorized,
    Unknown(String),
    Conflict(String),
}

impl Display for UpdateError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            UpdateError::NotFound => write!(f, "User not found"),
            UpdateError::Unauthorized => write!(f, "Unauthorized"),
            UpdateError::Unknown(msg) => write!(f, "Unknown error: {}", msg),
            UpdateError::Conflict(msg) => write!(f, "Conflict: {}", msg),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Payload {
    pub first_name: Option<FirstName>,
    pub last_name: Option<LastName>,
    pub birthday: Option<Birthday>,
    pub nationality: Option<Nationality>,
    pub languages: Option<Vec<Language>>,
}

pub async fn execute<T>(
    conn: &T, 
    repo: &impl UserRepositoryTrait<T>,
    secret: &[u8],
    token: &String,
    payload: Payload,
) -> Result<User, UpdateError> {
    // verify user exist and token is valid
    let id = if let Ok(auth) = TokenData::from_token(token, &secret) {
        auth.id
    } else {
        return Err(UpdateError::Unauthorized);
    };
    // verify user exists and data is not the same
    if let Ok(user) = repo.find_by_id(conn, id.into()).await {
        if any_equal(&payload, user.clone()) {
            return Err(UpdateError::Conflict("At least one of the fields is the same".to_string()));
        }
    } else {
        return Err(UpdateError::NotFound);
    };
    // update only user not sensitive info
    let user_update = UpdateUser {
        id: id.into(),
        first_name: payload.first_name,
        last_name: payload.last_name,
        birthday: payload.birthday,
        nationality: payload.nationality,
        languages: payload.languages,
    };
    match repo.update(conn, user_update).await {
        Ok(user) => Ok(user),
        Err(e) => Err(UpdateError::Unknown(format!("{:?}", e))),
    }
}

fn any_equal(payload: &Payload, user: User) -> bool {
    if Some(user.first_name) == payload.first_name {
        return true;
    }
    if Some(user.last_name) == payload.last_name {
        return true;
    }
    if Some(user.birthday) == payload.birthday {
        return true;
    }
    if Some(user.nationality) == payload.nationality {
        return true;
    }
    if Some(user.languages) == payload.languages {
        return true;
    }
    false
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