use argon2::{
    password_hash::{
        PasswordHash, PasswordVerifier, Error
    },
    Argon2
};

use crate::application::port::driven::user_repository::FindUser;

use super::super::port::driven::user_repository::UserRepositoryTrait;
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
    email: &Option<String>,
    phone_number: &Option<String>,
    password: &String
) -> Result<String, LoginError> {
    let find_user = FindUser {
        email: email.to_owned(),
        phone_number: phone_number.to_owned(),
        birthday: None,
        nationality: None,
        languages: None,
        created_at: None,
    };
    if let Ok(mut users) = repo.find_by_criteria(conn, &find_user, 0, 1).await {
        if users.len() < 1 {
            return Err(LoginError::InvalidData("User not found".to_string()));
        }
        if verify_password(&users.swap_remove(0).hashed_password.unwrap(), password).is_ok() {
            Ok(Auth::new(&users.swap_remove(0).id.unwrap().into()).token(secret))
        } else  {
            Err(LoginError::InvalidData("Invalid password".to_string()))
        }
    } else {
        Err(LoginError::InvalidData("User not found".to_string()))
    }
}

// TODO: Reduce the runtime; 1.2 seconds
pub fn verify_password(user_password: &String, password: &String) -> Result<(), Error> {
    let parsed_hash = PasswordHash::new(&user_password)?;
    Argon2::default().verify_password(
        password.as_bytes(), 
        &parsed_hash
    )
}