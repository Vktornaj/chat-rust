use common::adapter::config::Environment;

use crate::{application::port::driven::{
    auth_cache::{AuthCacheTrait, CreateAuthRequest}, 
    auth_repository::AuthRepositoryTrait, 
    email_service::EmailServiceTrait
}, domain::types::{code::Code, identification::IdentificationValue}};
use super::is_data_in_use;


#[derive(Debug)]
pub enum CreateError {
    InvalidData(String),
    Unknown(String),
    Conflict(String)
}

pub struct Payload {
    pub password: String, 
    pub identification_value: String,
    pub identification_type: String,
}

pub async fn execute<T, U, ES>(
    conn: &T,
    cache_conn: &U,
    email_conn: &ES,
    repo: &impl AuthRepositoryTrait<T>, 
    repo_cache: &impl AuthCacheTrait<U>,
    email_service: &impl EmailServiceTrait<ES>,
    environment: &Environment,
    payload: Payload
) -> Result<String, CreateError> {
    // TODO: improve environment handling
    let confirmation_code = match environment {
        Environment::Development => Code::new_0s(6),
        Environment::Production => Code::new(6),
    };
    let identity = match IdentificationValue::from_string(
        payload.identification_value,
        payload.identification_type
    ) {
        Ok(identification) => identification,
        Err(error) => return Err(CreateError::InvalidData(error.to_string())),
    };
    let auth_request = match CreateAuthRequest::new(
        payload.password,
        confirmation_code, 
        identity,
    ) {
        Ok(new_user) => new_user,
        Err(error) => return Err(CreateError::InvalidData(error.to_string())),
    };
    // verify no identity in cache
    match repo_cache.find_by_id::<CreateAuthRequest>(cache_conn, auth_request.identity.get_value().clone()).await {
        Ok(user) => {
            if user.is_some() {
                return Err(CreateError::Conflict("identification id unavailable".to_string()));
            }
        },
        Err(error) => return Err(CreateError::Unknown(format!("Unknown error: {:?}", error))),
    }
    // verify no user with same email or phone number
    if let Ok(res) = is_data_in_use::execute(
        conn, 
        repo, 
        is_data_in_use::Payload {
            identify_value: auth_request.identity.get_value().clone(),
            identify_type: auth_request.identity.get_type().clone(),
        }
    ).await {
        if res {
            return Err(CreateError::Conflict("identification value is already in use".to_string()));
        }
    } else {
        return Err(CreateError::Unknown("unknown error".to_string()));
    }
    // create auth request 
    let res = match repo_cache
        .add_request::<CreateAuthRequest>(
            cache_conn,
            identity.get_value().clone(),
            auth_request.clone(),
            60
        ).await {
        Ok(transaction_id) => Ok(transaction_id),
        Err(error) => Err(CreateError::Unknown(format!("Unknown error: {:?}", error))),
    };
    // Send confirmation email
    if let IdentificationValue::Email(email) = auth_request.identity {
        if email_service.send_confirmation_email(
            email_conn, 
            email.into(), 
            auth_request.confirmation_code.into()
        ).await.is_err() {
            return Err(CreateError::Unknown("Email invalid".to_string()));
        }
    }
    // TODO: send sms
    res
}

// #[cfg(test)]
// mod tests {
//     use std::sync::Mutex;

//     // use crate::repositories::pokemon::InMemoryRepository;
//     use chrono::{Utc, NaiveDate, DateTime};
//     use rocket::tokio;
//     use uuid::Uuid;
//     use super::*;
//     use crate::{
//         adapter::driven::persistence::in_memory_repository::InMemoryRepository, 
//         domain::types::id::Id
//     };
    
    // #[tokio::test]
    // async fn it_should_return_the_user_otherwise() {
    //     let conn = Mutex::new(vec![]);
    //     let repo = InMemoryRepository {};
    //     let repo_cache = InMemoryRepository {};
    //     let payload = Payload {
    //         email: Some("some_2@some.some".to_string()),
    //         phone_number: Some("+528331114146".to_string()),
    //         password: "Password123!".to_string(),
    //         first_name: "Victor".to_string(),
    //         last_name: "Najera".to_string(),
    //         birthday: NaiveDate::from_ymd_opt(1990, 1, 1)
    //             .unwrap()
    //             .and_hms_opt(0, 0, 0)
    //             .unwrap()
    //             .and_local_timezone(Utc)
    //             .unwrap(),
    //         nationality: "MEX".to_string(),
    //         languages: vec!["ES".to_string(), "EN".to_string()]
    //     };

    //     let res = execute(&conn, &repo, &repo_cache, payload).await;
        
    //     match res {
    //         Ok(user_cache) => {
    //             assert_eq!(Into::<String>::into(user_cache.email.unwrap()), "some_2@some.some".to_string());
    //             assert_eq!(Into::<String>::into(user_cache.phone_number.unwrap()), "+528331114146".to_string());
    //             assert_eq!(Into::<String>::into(user_cache.first_name), "Victor".to_string());
    //             assert_eq!(Into::<String>::into(user_cache.last_name), "Najera".to_string());
    //             assert_eq!(Into::<DateTime<Utc>>::into(user_cache.birthday), NaiveDate::from_ymd_opt(1990, 1, 1)
    //                 .unwrap()
    //                 .and_hms_opt(0, 0, 0)
    //                 .unwrap()
    //                 .and_local_timezone(Utc)
    //                 .unwrap());
    //             assert_eq!(Into::<String>::into(user_cache.nationality), "MEX".to_string());
    //             assert_eq!(
    //                 user_cache.languages.into_iter().map(|x| Into::<String>::into(x)).collect::<Vec<String>>(), 
    //                 vec!["ES".to_string(), "EN".to_string()]
    //             );
    //         }   
    //         _ => unreachable!(),
    //     };
    // }
// }
