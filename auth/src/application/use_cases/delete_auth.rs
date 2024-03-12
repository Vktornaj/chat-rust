use std::fmt::Display;

use crate::{
    application::port::driven::auth_repository::AuthRepositoryTrait, 
    domain::{types::{token_data::TokenData, password::Password}, 
    auth::Auth}
};


#[derive(Debug)]
pub enum DeleteError {
    NotFound,
    Unauthorized,
    Unknown(String),
}

impl Display for DeleteError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DeleteError::NotFound => write!(f, "Not found"),
            DeleteError::Unauthorized => write!(f, "Unauthorized"),
            DeleteError::Unknown(e) => write!(f, "Unknown error: {}", e),
        }
    }
}

pub struct Payload {
    pub password: Password,
}

pub async fn execute<T>(
    conn: &T, 
    repo: &impl AuthRepositoryTrait<T>, 
    secret: &[u8],
    token: &String,
    payload: Payload,
) -> Result<Auth, DeleteError> {
    // verify user exist and token is valid
    let user_id = if let Ok(token) = TokenData::from_token(token, &secret) {
        token.id
    } else {
        return Err(DeleteError::Unauthorized);
    };
    // get auth
    let auth = if let Ok(auth) = repo.find_by_id(conn, user_id.into()).await {
        auth
    } else {
        return Err(DeleteError::NotFound);
    };
    // verify password
    if payload.password.verify_password(&auth.hashed_password).is_err() {
        return Err(DeleteError::Unauthorized);
    }
    // TODO: delete user articles
    // delete auth
    match repo.delete(conn, user_id.into()).await {
        Ok(auth) => Ok(auth),
        Err(error) => Err(DeleteError::Unknown(format!("Unknown error: {:?}", error.to_string()))),
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
    //         Err(DeleteError::Unauthorized) => true,
    //         _ => false,
    //     });
    // }
}