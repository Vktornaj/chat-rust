use auth::domain::auth::Auth;

use super::super::port::driven::user_repository::UserRepositoryTrait;
use crate::{
    domain::types::{
        password::Password, 
        email::Email, 
        phone_number::{PhoneNumber, self}, 
        id::Id, code::Code
    }, 
    application::port::driven::{user_cache::{UserCacheTrait, UpdateUserCDCache}, user_repository::{FindUser, UpdateUser}}
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
    pub email: Option<Option<String>>,
    pub phone_number: Option<Option<String>>,
}

pub async fn execute<T>(
    conn: &T, 
    repo: &impl UserRepositoryTrait<T>,
    repo_cache: &impl UserCacheTrait<T>,
    secret: &[u8],
    token: &String,
    payload: Payload,
) -> Result<Option<String>, UpdateError> {
    // validate payload
    let password = if let Ok(password) = Password::try_from(payload.password) {
        password
    } else {
        return Err(UpdateError::Unautorized);
    };
    let email = if let Some(email) = payload.email {
        if let Ok(email) = email.map(|x| Email::try_from(x)).transpose() {
           Some(email)
        } else {
            return Err(UpdateError::InvalidData("Invalid email".to_string()));
        }
    } else {
        None
    };
    let phone_number = if let Some(phone_number) = payload.phone_number {
        if let Ok(phone_number) = phone_number.map(|x| PhoneNumber::try_from(x)).transpose() {
            Some(phone_number)
        } else {
            return Err(UpdateError::InvalidData("Invalid phone number".to_string()));
        }
    } else {
        None
    };
    // verify token is valid
    let id = if let Ok(auth) = Auth::from_token(token, &secret) {
        auth.id
    } else {
        return Err(UpdateError::Unautorized);
    };
    // verify no update request with same email or phone number in cache
    let transaction_id: String = if let Some(email) = email.as_ref() {
        email.clone().unwrap().into()
    } else {
        phone_number.clone().unwrap().unwrap().into()
    };
    if let Ok(res) = repo_cache.find_update(conn, transaction_id.clone()).await {
        if let Some(res) = res {
            return Err(UpdateError::Conflict(format!("update request already in progress")));
        }
    } else {
        return Err(UpdateError::Unknown("unknown error".to_string()));
    }
    // verify user exists, data is not the same, user contact data integrity and password match
    if let Ok(user) = repo.find_by_id(conn, id.into()).await {
        if let Some(email) = email.as_ref() {          
            if user.phone_number.is_none() && email.is_none() {
                return Err(UpdateError::Conflict("contact data can't be null".to_string()));
            }
            if &user.email == email {
                return Err(UpdateError::Conflict("Email is the same".to_string()));
            }
        }
        if let Some(phone_number) = phone_number.as_ref() {
            if user.email.is_none() && phone_number.is_none() {
                return Err(UpdateError::Conflict("contact data can't be null".to_string()));
            }
            if &user.phone_number == phone_number {
                return Err(UpdateError::Conflict("Phone number is the same".to_string()));
            }
        }
        if password.verify_password(&user.hashed_password).is_err() {
            return Err(UpdateError::Unautorized);
        }
    } else {
        return Err(UpdateError::NotFound);
    };
    // if none just delete
    if let Some(email) = email.as_ref() {
        if email.is_none() {
            let user = UpdateUser {
                id,
                email: Some(None),
                ..Default::default()
            };
            match repo.update(conn, user).await {
                Ok(_) => return Ok(None),
                Err(e) => return Err(UpdateError::Unknown(format!("{:?}", e)))
            }
        }
    }
    if let Some(phone_number) = phone_number.as_ref() {
        if phone_number.is_none() {
            let user = UpdateUser {
                id,
                phone_number: Some(None),
                ..Default::default()
            };
            match repo.update(conn, user).await {
                Ok(_) => return Ok(None),
                Err(e) => return Err(UpdateError::Unknown(format!("{:?}", e)))
            }
        }
    }
    // verify email is not in use
    if let Some(email) = &email {
        let find_user = FindUser {
            email: email.clone(),
            ..Default::default()
        };
        if let Ok(users) = repo.find_by_criteria(conn, find_user, 0, 1).await {
            if users.len() > 0 {
                return Err(UpdateError::Conflict("Email already in use".to_string()));
            }
        }
    }
    // verify phone number is not in use
    if let Some(phone_number) = &phone_number {
        let find_user = FindUser {
            phone_number: phone_number.clone(),
            ..Default::default()
        };
        if let Ok(users) = repo.find_by_criteria(conn, find_user, 0, 1).await {
            if users.len() > 0 {
                return Err(UpdateError::Conflict("Phone number already in use".to_string()));
            }
        }
    }
    // create request to update sensitive info
    let user_update_cache = UpdateUserCDCache {
        id: Id::try_from(id).unwrap(),
        email,
        phone_number,
        confirmation_code: Code::new(6)
    };
    let res = match repo_cache
        .add_update_user(
            conn, 
            transaction_id, 
            user_update_cache, 
            3600
        ).await {
        Ok(transaction_id) => Ok(Some(transaction_id)),
        Err(e) => Err(UpdateError::Unknown(format!("{:?}", e)))
    };
    // TODO: send email
    // TODO: send sms
    res
}

#[cfg(test)]
mod tests {
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

    
}