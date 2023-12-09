use common::domain::types::id::Id;

use crate::{application::port::driven::{
    auth_repository::AuthRepositoryTrait, 
    auth_cache::{AuthCacheTrait, AddIdentificationRequest}, 
    email_service::EmailServiceTrait,
}, domain::types::{identification::IdentificationValue, token_data::TokenData, code::Code}};


#[derive(Debug)]
pub enum UpdateError {
    NotFound,
    Unautorized,
    Unknown(String),
    Conflict(String),
    InvalidData(String),
}

pub struct Payload {
    pub identify_value: String,
    pub identify_type: String,
}

pub async fn execute<T, U, ES>(
    conn: &T,
    cache_conn: &U,
    email_conn: &ES,
    repo: &impl AuthRepositoryTrait<T>,
    repo_cache: &impl AuthCacheTrait<U>,
    email_service: &impl EmailServiceTrait<ES>,
    secret: &[u8],
    token: &String,
    payload: Payload,
) -> Result<Option<String>, UpdateError> {
    // validate payload
    let identity = IdentificationValue::from_string(
        payload.identify_value.clone(), 
        payload.identify_type.clone(),
    ).map_err(|e| UpdateError::InvalidData(e))?;
    // verify token is valid
    let user_id = if let Ok(token) = TokenData::from_token(token, &secret) {
        token.id
    } else {
        return Err(UpdateError::Unautorized);
    };
    // verify no update request with same email or phone number in cache
    let transaction_id: String = identity.get_value();
    if let Ok(res) = repo_cache
        .find_by_id::<AddIdentificationRequest>(cache_conn, transaction_id.clone()).await {
        if res.is_some() {
            return Err(UpdateError::Conflict(format!("update request already in progress")));
        }
    } else {
        return Err(UpdateError::Unknown("unknown error".to_string()));
    }
    // verify user exists, data is not the same, user contact data integrity and password match
    if let Ok(auth) = repo.find_by_id(conn, user_id.into()).await {
        if auth.identifications.iter()
            .map(|i| i.identification_value).collect::<Vec<&IdentificationValue>>()
            .contains(identity) 
        {
            return Err(UpdateError::Conflict("Phone number is the same".to_string()));
        }
    } else {
        return Err(UpdateError::NotFound);
    };
    // verify email is not in use
    if let Ok(users) = repo.find_by_identification(conn, identity).await {
        return Err(UpdateError::Conflict("Email already in use".to_string()));
    }
    // create request to update sensitive info
    let confirmation_code = Code::new(6);
    let update_aurh_request = AddIdentificationRequest {
        user_id: Id::try_from(user_id).unwrap(),
        identity,
        confirmation_code: confirmation_code.clone(),
    };
    let res = match repo_cache
        .add_request::<AddIdentificationRequest>(
            cache_conn, 
            transaction_id, 
            update_aurh_request, 
            60
        ).await {
        Ok(transaction_id) => Ok(Some(transaction_id)),
        Err(e) => Err(UpdateError::Unknown(format!("{:?}", e)))
    };
    // Send confirmation email
    if let IdentificationValue::Email(email) = identity {
        if email_service.send_confirmation_email(
            email_conn, 
            email.into(),
            confirmation_code.into()
        ).await.is_err() {
            return Err(UpdateError::Unknown("Email invalid".to_string()));
        }
    }
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