use crate::{
    application::port::driven::{auth_repository::AuthRepositoryTrait, email_service::EmailServiceTrait}, 
    domain::types::{identification::IdentificationValue, token_data::TokenData},
};

#[derive(Debug)]
pub enum ResetError {
    InvalidData(String),
    Unknown(String),
    NotFound(String),
}

pub struct Payload {
    pub identifier_value: String,
    pub identifier_type: String,
    pub domain: String,
}

pub async fn execute<T, U>(
    conn: &T,
    email_conn: &U,
    repo: &impl AuthRepositoryTrait<T>,
    email_service: &impl EmailServiceTrait<U>,
    secret: &[u8],
    payload: Payload
) -> Result<(), ResetError> {
    // Get user id
    let identifier = IdentificationValue::from_string(
        payload.identifier_value.clone(),
        payload.identifier_type.clone()
    ).map_err(|_| {
        ResetError::InvalidData("Invalid identifier".to_string())
    })?;
    let auth = match repo.find_by_identification(conn, identifier.clone()).await {
        Ok(auth) => auth,
        Err(_) => return Err(ResetError::NotFound("Unknown error".to_string())),
    };
    // Generate link
    let token = TokenData::new_reset_password_token(&auth.user_id.into());
    let link = format!("http://{}/api/password-reset/{}", payload.domain, token.token(secret));

    // Send reset password email
    if let IdentificationValue::Email(email) = identifier {
        match email_service.send_reset_password_email(email_conn, email.into(), link).await {
            Ok(_) => (),
            Err(_) => return Err(ResetError::Unknown("Unknown error".to_string())),
        };
    }
    // TODO: send reset password sms
    Ok(())
}

#[cfg(test)]
mod tests {
    // use std::sync::Mutex;

    // // use crate::repositories::pokemon::InMemoryRepository;
    // use chrono::{Utc, NaiveDate, DateTime};
    // use rocket::tokio;
    // use uuid::Uuid;
    // use super::*;
    // use crate::{
    //     adapter::driven::persistence::in_memory_repository::InMemoryRepository, 
    //     domain::types::id::Id
    // };
    
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
}