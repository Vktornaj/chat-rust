use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHasher, SaltString, Error
    },
    Argon2
};

use super::super::port::driven::user_repository::UserRepositoryTrait;
use crate::{domain::user::User, application::port::driven::user_repository::NewUser};
use super::is_user_exist;


#[derive(Debug)]
pub enum CreateError {
    InvalidData(String),
    Unknown(String),
    Conflict(String)
}

pub async fn execute<T>(conn: &T, repo: &impl UserRepositoryTrait<T>, new_user: NewUser) -> Result<User, CreateError> {
    let mut new_user = if let Ok(new_user) = User::new(
        None, 
        new_user.email, 
        new_user.phone_number, 
        new_user.password, 
        Some(new_user.first_name), 
        Some(new_user.last_name), 
        Some(new_user.birthday), 
        new_user.nationality, 
        Some(new_user.languages), 
        None, 
        None
    ) {
        NewUser {
            email: new_user.email,
            phone_number: new_user.phone_number,
            password: new_user.password,
            first_name: new_user.first_name.unwrap(),
            last_name: new_user.last_name.unwrap(),
            birthday: new_user.birthday.unwrap(),
            nationality: new_user.nationality,
            languages: new_user.languages.unwrap(),
        }
    } else {
        return Err(CreateError::InvalidData("".to_string()))
    };
    if is_user_exist::execute(conn, repo, &new_user.email, &new_user.phone_number).await {
        return Err(CreateError::Conflict("email or phone already in use".to_string()))
    }
    if let Ok(hashed_password) = hash_password(new_user.password) {
        new_user.password = hashed_password;
    } else {
        return Err(CreateError::InvalidData("Invalid password".to_string()));
    }
    match repo.create(conn, new_user).await {
        Ok(user) => Ok(user),
        Err(error) => Err(CreateError::Unknown(format!("Unknown error: {:?}", error))),
    }
}

// TODO: Reduce the runtime; 1.3 seconds
pub fn hash_password(password: String) -> Result<String, Error>{
    let salt = SaltString::generate(&mut OsRng);
    
    let argon2 = Argon2::default();
    Ok(argon2.hash_password(password.as_bytes(), &salt)?.to_string())
}