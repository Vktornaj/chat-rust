use auth::domain::auth::Auth;

use crate::{
    application::port::driven::{
        user_repository::{UserRepositoryTrait, FindUser}, 
        email_service::EmailServiceTrait
    }, 
    domain::types::{email::Email, phone_number::PhoneNumber}
};


#[derive(Debug)]
pub enum ResetError {
    InvalidData(String),
    Unknown(String),
    NotFound(String),
}

pub struct Payload {
    pub email: Option<String>, 
    pub phone_number: Option<String>,
    pub domain: String,
}

pub async fn execute<T, U>(
    conn: &T,
    email_conn: &U,
    repo: &impl UserRepositoryTrait<T>,
    email_service: &impl EmailServiceTrait<U>,
    secret: &[u8],
    payload: Payload
) -> Result<(), ResetError> {
    // Get user id
    let find_user = if let Some(email) = payload.email.clone() {
        FindUser {
            email: Some(Email::try_from(email).unwrap()),
            ..Default::default()
        }
    } else if let Some(phone_number) = payload.phone_number.clone() {
        FindUser {
            phone_number: Some(PhoneNumber::try_from(phone_number).unwrap()),
            ..Default::default()
        }
    } else {
        return Err(ResetError::InvalidData("Email or phone number required".to_string()));
    };

    let user = match repo
        .find_by_criteria(conn, find_user, 0, 1).await {
        Ok(users) => {
            if users.len() == 0 {
                return Err(ResetError::InvalidData("Email or phone number not found".to_string()));
            }
            users[0].clone()
        },
        Err(_) => return Err(ResetError::NotFound("Unknown error".to_string())),
    };

    // Generate link
    let token = Auth::new_reset_password_token(&user.id.into());
    let link = format!("http://{}/api/password-reset/{}", payload.domain, token.token(secret));

    // Send reset password email
    if let Some(email) = payload.email.clone() {
        match email_service.send_reset_password_email(email_conn, email, link).await {
            Ok(_) => (),
            Err(_) => return Err(ResetError::Unknown("Unknown error".to_string())),
        };
    }
    // TODO: send reset password sms
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::sync::Mutex;

    // use crate::repositories::pokemon::InMemoryRepository;
    use chrono::{Utc, NaiveDate, DateTime};
    use rocket::tokio;
    use uuid::Uuid;
    use super::*;
    use crate::{
        adapter::driven::persistence::in_memory_repository::InMemoryRepository, 
        domain::types::id::Id
    };
    
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