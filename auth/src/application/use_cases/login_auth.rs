use crate::domain::types::{
    password::Password, 
    identification::IdentificationValue,
    token_data::TokenData,
};

use super::super::port::driven::auth_repository::AuthRepositoryTrait;


#[derive(Debug)]
pub enum LoginError {
    NotFound,
    Unauthorized,
}

pub struct Payload {
    pub identifier: String,
    pub password: Password,
}

// TODO: improve when criteria will implemented onto the traid
pub async fn execute<T>(
    conn: &T,
    repo: &impl AuthRepositoryTrait<T>, 
    secret: &[u8],
    payload: Payload,
) -> Result<String, LoginError> {
    
    let identifier: IdentificationValue = match IdentificationValue::try_from(payload.identifier) {
        Ok(identifier) => identifier,
        Err(_) => return Err(LoginError::NotFound)
    };
    if let Ok(user) = repo.find_by_identification(conn, identifier).await {
        if payload.password.verify_password(&user.hashed_password).is_ok() {
            Ok(TokenData::new(&user.user_id.into()).token(secret))
        } else  {
            Err(LoginError::Unauthorized)
        }
    } else {
        Err(LoginError::NotFound)
    }
}