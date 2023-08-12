use auth::domain::auth::Auth;

use super::{super::port::driven::user_repository::UserRepositoryTrait, utils};
use crate::domain::user::User;


#[derive(Debug)]
pub enum DeleteError {
    NotFound,
    Unautorized,
    Unknown(String),
    Conflict(String),
}

pub async fn execute<T>(
    conn: &T, 
    repo: &impl UserRepositoryTrait<T>, 
    password: String,
    secret: &[u8],
    token: &String
) -> Result<User, DeleteError> {
    // verify user exist and token is valid
    let id = if let Ok(auth) = Auth::from_token(token, &secret) {
        auth.id
    } else {
        return Err(DeleteError::Unautorized);
    };
    // verify password
    if let Ok(user) = repo.find_by_id(conn, id.into()).await {
        if utils::verify_password(&user.hashed_password.unwrap(), &password).is_err() {
            return Err(DeleteError::Unautorized);
        }
    } else {
        return Err(DeleteError::NotFound);
    };
    // TODO: delete user articles
    // delete user
    match repo.delete(conn, id.into()).await {
        Ok(user) => Ok(user),
        Err(error) => Err(DeleteError::Unknown(format!("Unknown error: {:?}", error))),
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Mutex;

    // use crate::repositories::pokemon::InMemoryRepository;
    use chrono::{Utc, NaiveDate};
    use rocket::tokio;
    use uuid::Uuid;
    use super::*;
    use crate::{
        application::{
            port::driven::{
                user_repository::NewUser, 
                errors::RepoSelectError
            }, 
            use_cases::create_user
        }, 
        domain::user::{Id, Email, PhoneNumber, Password, FirstName, LastName, Birthday, Nationality, Language}
    };
    use crate::adapter::driven::persistence::in_memory_repository::InMemoryRepository;

    #[tokio::test]
    async fn delete_user_successful() {
        let secret: Vec<u8> = "8Xui8SN4mI+7egV/9dlfYYLGQJeEx4+DwmSQLwDVXJg=".to_string().into_bytes();
        let conn = Mutex::new(vec![]);
        let repo = InMemoryRepository {};
        let password = "Password123!".to_string();
        let new_user = NewUser {
            email: Some(Email::try_from("some_2@some.some".to_string()).unwrap()),
            phone_number: Some(PhoneNumber::try_from("+528331114146".to_string()).unwrap()),
            password: Some(Password::try_from("Password123!".to_string()).unwrap()),
            hashed_password: None,
            first_name: FirstName::try_from("Victor Eduardo".to_string()).unwrap(),
            last_name: LastName::try_from("Garcia Najera".to_string()).unwrap(),
            birthday: Birthday::try_from(
                NaiveDate::from_ymd_opt(1990, 1, 1)
                    .unwrap()
                    .and_hms_opt(0, 0, 0)
                    .unwrap()
                    .and_local_timezone(Utc)
                    .unwrap()
            ).unwrap(),
            nationality: Nationality::try_from("MEX".to_string()).unwrap(),
            languages: vec![
                Language::try_from("ES".to_string()).unwrap(), 
                Language::try_from("EN".to_string()).unwrap(), 
            ],
        };

        let res = create_user::execute(&conn, &repo, new_user).await;
        
        let user = match res {
            Ok(user) => {
                assert!(Id::try_from(Into::<Uuid>::into(user.id.clone().unwrap())).is_ok());
                user
            }   
            _ => unreachable!(),
        };

        let token = Auth::new(&user.id.unwrap().into()).token(&secret);

        // delete user
        let res = execute(&conn, &repo, password, &secret, &token).await;

        let user = match res {
            Ok(user) => {
                assert!(Id::try_from(Into::<Uuid>::into(user.id.clone().unwrap())).is_ok());
                user
            }   
            _ => unreachable!(),
        };

        let res = repo.find_by_id(&conn, user.id.unwrap().into()).await;

        assert!(match res {
            Ok(_) => false,
            Err(RepoSelectError::NotFound) => true,
            _ => false,
        });

    }
    
    #[tokio::test]
    async fn delete_user_bad_password() {
        // prepare for test
        let secret: Vec<u8> = "8Xui8SN4mI+7egV/9dlfYYLGQJeEx4+DwmSQLwDVXJg=".to_string().into_bytes();
        let conn = Mutex::new(vec![]);
        let repo = InMemoryRepository {};
        let password = "Password123!".to_string();
        let new_user = NewUser {
            email: Some(Email::try_from("some_2@some.some".to_string()).unwrap()),
            phone_number: Some(PhoneNumber::try_from("+528331114146".to_string()).unwrap()),
            password: Some(Password::try_from("Password123!".to_string()).unwrap()),
            hashed_password: None,
            first_name: FirstName::try_from("Victor Eduardo".to_string()).unwrap(),
            last_name: LastName::try_from("Garcia Najera".to_string()).unwrap(),
            birthday: Birthday::try_from(
                NaiveDate::from_ymd_opt(1990, 1, 1)
                    .unwrap()
                    .and_hms_opt(0, 0, 0)
                    .unwrap()
                    .and_local_timezone(Utc)
                    .unwrap()
            ).unwrap(),
            nationality: Nationality::try_from("MEX".to_string()).unwrap(),
            languages: vec![
                Language::try_from("ES".to_string()).unwrap(), 
                Language::try_from("EN".to_string()).unwrap(), 
            ],
        };

        let res = create_user::execute(&conn, &repo, new_user).await;
        
        let user = match res {
            Ok(user) => {
                assert!(Id::try_from(Into::<Uuid>::into(user.id.clone().unwrap())).is_ok());
                user
            }   
            _ => unreachable!(),
        };

        let token = Auth::new(&user.id.unwrap().into()).token(&secret);

        // delete user
        let res = execute(&conn, &repo, "".to_string(), &secret, &token).await;

        assert!(match res {
            Ok(_) => false,
            Err(DeleteError::Unautorized) => true,
            _ => false,
        });
    }
}