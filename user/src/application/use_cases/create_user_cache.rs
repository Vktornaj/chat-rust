use crate::{application::port::driven::{
    user_cache::{UserCacheTrait, CreateUserCache}, 
    user_repository::UserRepositoryTrait, 
    email_service::EmailServiceTrait
}, domain::types::code::Code};
use super::is_data_in_use;


#[derive(Debug)]
pub enum CreateError {
    InvalidData(String),
    Unknown(String),
    Conflict(String)
}

pub struct Payload {
    pub email: Option<String>, 
    pub phone_number: Option<String>, 
    pub password: String, 
    pub first_name: String, 
    pub last_name: String, 
    pub birthday: chrono::DateTime<chrono::Utc>, 
    pub nationality: String, 
    pub languages: Vec<String>
}

pub async fn execute<T, U, ES>(
    conn: &T,
    cache_conn: &U,
    email_conn: &ES,
    repo: &impl UserRepositoryTrait<T>, 
    repo_cache: &impl UserCacheTrait<U>,
    email_service: &impl EmailServiceTrait<ES>,
    environment: &String,
    payload: Payload
) -> Result<String, CreateError> {
    // TODO: improve environment handling
    let confirmation_code = if environment == "production" {
        Code::new(6)
    } else {
        Code::new_0s(6)
    };
    let cache_user = match CreateUserCache::new(
        payload.email,
        payload.phone_number, 
        payload.password, 
        payload.first_name, 
        payload.last_name, 
        payload.birthday, 
        payload.nationality, 
        payload.languages,
        confirmation_code
    ) {
        Ok(new_user) => new_user,
        Err(error) => return Err(CreateError::InvalidData(error.to_string())),
    };
    // verify no email in cache
    if let Some(email) = cache_user.email.clone() {
        match repo_cache.find_by_id::<CreateUserCache>(cache_conn, email.into()).await {
            Ok(user) => {
                if user.is_some() {
                    return Err(CreateError::Conflict("email unavailable".to_string()));
                }
            },
            Err(error) => return Err(CreateError::Unknown(format!("Unknown error: {:?}", error))),
        }
    }
    // verify no phone number in cache
    if let Some(phone_number) = cache_user.phone_number.clone() {
        match repo_cache.find_by_id::<CreateUserCache>(cache_conn, phone_number.into()).await {
            Ok(user) => {
                if user.is_some() {
                    return Err(CreateError::Conflict("phone number unavailable".to_string()));
                }
            },
            Err(error) => return Err(CreateError::Unknown(format!("Unknown error: {:?}", error))),
        }
    }
    // verify no user with same email or phone number
    if let Ok(res) = is_data_in_use::execute(
        conn, 
        repo, 
        is_data_in_use::Payload {
            email: cache_user.email.as_ref().map(|x| x.to_owned().into()), 
            phone_number: cache_user.phone_number.as_ref().map(|x| x.to_owned().into())
        }
    ).await {
        if res {
            return Err(CreateError::Conflict("email or phone already in use".to_string()));
        }
    } else {
        return Err(CreateError::Unknown("unknown error".to_string()));
    }
    // create cache user
    let id: String = if let Some(email) = cache_user.email.clone() {
        email.into()
    } else {
        cache_user.phone_number.clone().unwrap().into()
    };
    let res = match repo_cache
        .add_request::<CreateUserCache>(
            cache_conn,
            id,
            cache_user.clone(),
            60
        ).await {
        Ok(transaction_id) => Ok(transaction_id),
        Err(error) => Err(CreateError::Unknown(format!("Unknown error: {:?}", error))),
    };
    // Send confirmation email
    if let Some(email) = cache_user.email.clone() {
        if email_service.send_confirmation_email(
            email_conn, 
            email.into(), 
            cache_user.confirmation_code.into()
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
