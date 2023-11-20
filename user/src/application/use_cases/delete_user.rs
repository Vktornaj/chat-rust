use auth::domain::auth::Auth;

use super::super::port::driven::user_repository::UserRepositoryTrait;
use crate::domain::{user::User, types::password::Password};


#[derive(Debug)]
pub enum DeleteError {
    NotFound,
    Unautorized,
    Unknown(String),
}

pub struct Payload {
    pub password: String,
}

pub async fn execute<T>(
    conn: &T, 
    repo: &impl UserRepositoryTrait<T>, 
    secret: &[u8],
    token: &String,
    payload: Payload,
) -> Result<User, DeleteError> {
    // verify user exist and token is valid
    let id = if let Ok(auth) = Auth::from_token(token, &secret) {
        auth.id
    } else {
        return Err(DeleteError::Unautorized);
    };
    // verify password is a valid password
    let password = if let Ok(password) = Password::try_from(payload.password) {
        password
    } else {
        return Err(DeleteError::Unautorized);
    };
    // get user
    let user = if let Ok(user) = repo.find_by_id(conn, id.into()).await {
        user
    } else {
        return Err(DeleteError::NotFound);
    };
    // verify password
    if password.verify_password(&user.hashed_password).is_err() {
        return Err(DeleteError::Unautorized);
    }
    // TODO: delete user articles
    // delete user
    match repo.delete(conn, id.into()).await {
        Ok(user) => Ok(user),
        Err(error) => Err(DeleteError::Unknown(format!("Unknown error: {:?}", error))),
    }
}

#[cfg(test)]
mod tests {
    // use std::sync::Mutex;

    // use crate::repositories::pokemon::InMemoryRepository;
    // use chrono::{Utc, NaiveDate};
    // use rocket::tokio;
    // use uuid::Uuid;
    // use super::*;
    // use crate::{adapter::driven::persistence::in_memory_repository::InMemoryRepository, domain::types::id::Id, application::port::driven::errors::RepoSelectError};
    // use super::super::create_user_cache;

    // #[tokio::test]
    // async fn delete_user_successful() {
    //     let secret: Vec<u8> = "8Xui8SN4mI+7egV/9dlfYYLGQJeEx4+DwmSQLwDVXJg=".to_string().into_bytes();
    //     let conn = Mutex::new(vec![]);
    //     let repo = InMemoryRepository {};
    //     let payload = Payload {
    //         password: "Password123!".to_string(),
    //     };
    //     let create_user_payload = create_user_cache::Payload {
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

    //     let res = create_user_cache::execute(&conn, &repo, create_user_payload).await;
        
    //     let user = match res {
    //         Ok(user) => {
    //             assert!(Id::try_from(Into::<Uuid>::into(user.id.clone())).is_ok());
    //             user
    //         }   
    //         _ => unreachable!(),
    //     };

    //     let token = Auth::new(&user.id.into()).token(&secret);

    //     // delete user
    //     let res = execute(&conn, &repo, &secret, &token, payload).await;

    //     let user = match res {
    //         Ok(user) => {
    //             assert!(Id::try_from(Into::<Uuid>::into(user.id.clone())).is_ok());
    //             user
    //         }   
    //         Err(e) => return assert!(false, "{}", format!("Error: {:?}", e)),
    //     };

    //     let res = repo.find_by_id(&conn, user.id.into()).await;

    //     assert!(match res {
    //         Ok(_) => false,
    //         Err(RepoSelectError::NotFound) => true,
    //         _ => false,
    //     });

    // }
    
    // #[tokio::test]
    // async fn delete_user_bad_password() {
    //     // prepare for test
    //     let secret: Vec<u8> = "8Xui8SN4mI+7egV/9dlfYYLGQJeEx4+DwmSQLwDVXJg=".to_string().into_bytes();
    //     let conn = Mutex::new(vec![]);
    //     let repo = InMemoryRepository {};
    //     let payload = Payload {
    //         password: "".to_string(),
    //     };
    //     let create_user_payload = create_user_cache::Payload {
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

    //     let res = create_user_cache::execute(&conn, &repo, create_user_payload).await;
        
    //     let user = match res {
    //         Ok(user) => {
    //             assert!(Id::try_from(Into::<Uuid>::into(user.id.clone())).is_ok());
    //             user
    //         }   
    //         _ => unreachable!(),
    //     };

    //     let token = Auth::new(&user.id.into()).token(&secret);

    //     // delete user
    //     let res = execute(&conn, &repo, &secret, &token, payload).await;

    //     assert!(match res {
    //         Ok(_) => false,
    //         Err(DeleteError::Unautorized) => true,
    //         _ => false,
    //     });
    // }
}