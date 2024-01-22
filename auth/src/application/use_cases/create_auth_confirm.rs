use crate::{application::port::driven::{
    auth_repository::AuthRepositoryTrait, auth_cache::{AuthCacheTrait, CreateAuthRequest}}, 
    domain::{auth::Auth, types::code::Code}
};



#[derive(Debug)]
pub enum CreateError {
    InvalidData(String),
    Unknown(String),
    Conflict(String)
}

pub struct Payload {
    pub transaction_id: String,
    pub confirmation_code: Code,
}

// TODO: add attempt limit
pub async fn execute<T, U>(
    conn: &T,
    cache_conn: &U,
    repo: &impl AuthRepositoryTrait<T>, 
    repo_cache: &impl AuthCacheTrait<U>,
    payload: Payload,
) -> Result<Auth, CreateError> {
    // validate confirmation code
    let new_auth = match repo_cache
        .find_by_id::<CreateAuthRequest>(cache_conn, payload.transaction_id.clone()).await 
    {
        Ok(auth) => match auth {
            Some(auth) => {
                if auth.confirmation_code == payload.confirmation_code {
                    auth.to_new_auth()
                } else {
                    return Err(CreateError::InvalidData("invalid confirmation code".to_string()));
                }
            },
            None => return Err(CreateError::InvalidData("invalid transaction id".to_string())),
        },
        Err(error) => return Err(CreateError::Unknown(format!("Unknown error: {:?}", error))),
    };
    // delete cache
    match repo_cache.delete(cache_conn, payload.transaction_id).await {
        Ok(_) => (),
        Err(error) => return Err(CreateError::Unknown(format!("Unknown error: {:?}", error))),
    };
    // create auth
    match repo.create(conn, new_auth).await {
        Ok(user) => Ok(user),
        Err(error) => Err(CreateError::Unknown(format!("Unknown error: {:?}", error))),
    }
}

#[cfg(test)]
mod tests {
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

//     #[tokio::test]
//     async fn it_should_return_the_user_otherwise() {
//         let conn = Mutex::new(vec![]);
//         let repo = InMemoryRepository {};
//         let payload = Payload {
//             email: Some("some_2@some.some".to_string()),
//             phone_number: Some("+528331114146".to_string()),
//             password: "Password123!".to_string(),
//             first_name: "Victor".to_string(),
//             last_name: "Najera".to_string(),
//             birthday: NaiveDate::from_ymd_opt(1990, 1, 1)
//                 .unwrap()
//                 .and_hms_opt(0, 0, 0)
//                 .unwrap()
//                 .and_local_timezone(Utc)
//                 .unwrap(),
//             nationality: "MEX".to_string(),
//             languages: vec!["ES".to_string(), "EN".to_string()]
//         };

//         let res = execute(&conn, &repo, payload).await;
        
//         match res {
//             Ok(user) => {
//                 assert!(Id::try_from(Into::<Uuid>::into(user.id)).is_ok());
//                 assert_eq!(Into::<String>::into(user.email.unwrap()), "some_2@some.some".to_string());
//                 assert_eq!(Into::<String>::into(user.phone_number.unwrap()), "+528331114146".to_string());
//                 assert_eq!(Into::<String>::into(user.first_name), "Victor".to_string());
//                 assert_eq!(Into::<String>::into(user.last_name), "Najera".to_string());
//                 assert_eq!(Into::<DateTime<Utc>>::into(user.birthday), NaiveDate::from_ymd_opt(1990, 1, 1)
//                     .unwrap()
//                     .and_hms_opt(0, 0, 0)
//                     .unwrap()
//                     .and_local_timezone(Utc)
//                     .unwrap());
//                 assert_eq!(Into::<String>::into(user.nationality), "MEX".to_string());
//                 assert_eq!(
//                     user.languages.into_iter().map(|x| Into::<String>::into(x)).collect::<Vec<String>>(), 
//                     vec!["ES".to_string(), "EN".to_string()]
//                 );
//             }   
//             _ => unreachable!(),
//         };
//     }
}