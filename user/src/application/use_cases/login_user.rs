use crate::{application::port::driven::user_repository::FindUser, domain::user::{Email, PhoneNumber, Password}};

use super::{super::port::driven::user_repository::UserRepositoryTrait, utils};
use auth::domain::auth::Auth;


#[derive(Debug)]
pub enum LoginError {
    InvalidData(String),
    Unknown(String),
    Conflict(String)
}

// TODO: improve when criteria will implemented onto the traid
pub async fn execute<T>(
    conn: &T,
    repo: &impl UserRepositoryTrait<T>, 
    secret: &[u8],
    email: Option<Email>,
    phone_number: Option<PhoneNumber>,
    password: Password
) -> Result<String, LoginError> {
    let find_user = FindUser {
        email: email.to_owned(),
        phone_number: phone_number.to_owned(),
        birthday: None,
        nationality: None,
        languages: None,
        created_at: None,
    };
    if let Ok(mut users) = repo.find_by_criteria(conn, find_user, 0, 1).await {
        if users.len() < 1 {
            return Err(LoginError::InvalidData("User not found".to_string()));
        }
        let user = users.swap_remove(0);
        if utils::verify_password(&user.hashed_password.unwrap(), &password.into()).is_ok() {
            Ok(Auth::new(&user.id.unwrap().into()).token(secret))
        } else  {
            Err(LoginError::InvalidData("Invalid password".to_string()))
        }
    } else {
        Err(LoginError::InvalidData("User not found".to_string()))
    }
}

