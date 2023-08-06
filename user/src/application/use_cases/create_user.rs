use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHasher, SaltString, Error
    },
    Argon2
};
use chrono::{DateTime, Utc};

use super::super::port::driven::user_repository::UserRepositoryTrait;
use crate::{
    domain::user::{
        User, Email, PhoneNumber, Password, FirstName, LastName, Birthday, Nationality, Language
    }, 
    application::port::driven::user_repository::NewUser
};
use super::is_user_exist;


#[derive(Debug)]
pub enum CreateError {
    InvalidData(String),
    Unknown(String),
    Conflict(String)
}

pub async fn execute<T>(conn: &T, repo: &impl UserRepositoryTrait<T>, mut new_user: NewUser) -> Result<User, CreateError> {
    // validate data
    new_user = validate_data(new_user)?;  
    // verify no user with same email or phone number
    if is_user_exist::execute(conn, repo, &new_user.email, &new_user.phone_number).await {
        return Err(CreateError::Conflict("email or phone already in use".to_string()))
    }
    // hash password
    new_user.password = if let Ok(hashed_password) = hash_password(new_user.password) {
        hashed_password
    } else {
        return Err(CreateError::InvalidData("Invalid password".to_string()));
    };
    // create user
    match repo.create(conn, new_user).await {
        Ok(user) => Ok(user),
        Err(error) => Err(CreateError::Unknown(format!("Unknown error: {:?}", error))),
    }
}

// TODO: Reduce the runtime; 1.3 seconds
fn hash_password(password: String) -> Result<String, Error>{
    let salt = SaltString::generate(&mut OsRng);
    
    let argon2 = Argon2::default();
    Ok(argon2.hash_password(password.as_bytes(), &salt)?.to_string())
}

fn validate_data(mut new_user: NewUser) -> Result<NewUser, CreateError> {
    new_user.email = evaluate::<Email, String>(new_user.email)?;
    new_user.phone_number = evaluate::<PhoneNumber, String>(new_user.phone_number)?;
    new_user.password = evaluate::<Password, String>(Some(new_user.password))?.unwrap();
    new_user.first_name = evaluate::<FirstName, String>(Some(new_user.first_name))?.unwrap();
    new_user.last_name = evaluate::<LastName, String>(Some(new_user.last_name))?.unwrap();
    new_user.birthday = evaluate::<Birthday, DateTime<Utc>>(Some(new_user.birthday))?.unwrap();
    new_user.nationality = evaluate::<Nationality, String>(Some(new_user.nationality))?.unwrap();
    let mut temp_languages: Vec<String> = Vec::new();
    for language in new_user.languages {
        if let Ok(language) = evaluate::<Language, String>(Some(language)) {
            temp_languages.push(language.unwrap());
        } else {
            return Err(CreateError::InvalidData("Invalid language".to_string()));
        }
    }
    new_user.languages = temp_languages;   
    Ok(new_user)
}

fn evaluate<T,E>(item: Option<E>) -> Result<Option<E>, CreateError> 
where 
    T: std::convert::TryFrom<E>,
    E: std::convert::From<T>,
    <T as TryFrom<E>>::Error: std::fmt::Display
{

    if let Some(some) = item {
        match T::try_from(some) {
            Ok(some) => Ok(Some(some.into())),
            Err(some) => Err(CreateError::InvalidData(some.to_string()))
        }
    } else {
        Ok(None)
    }
}

