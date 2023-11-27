use auth::domain::token_data::TokenData;

use crate::{
    application::port::driven::user_repository::{UserRepositoryTrait, UpdateUser}, 
    domain::{user::User, types::password::Password}
};


#[derive(Debug)]
pub enum ResetError {
    InvalidData(String),
    Unknown(String),
    NotFound(String),
}

pub struct Payload {
    pub token: String,
    pub password: String,
}

pub async fn execute<T>(
    conn: &T,
    repo: &impl UserRepositoryTrait<T>,
    secret: &[u8],
    payload: Payload
) -> Result<User, ResetError> {
    // validate password
    let password = Password::try_from(payload.password).map_err(|_| {
        ResetError::InvalidData("Invalid password".to_string())
    })?;
    // Get user id
    let id = if let Ok(auth) = TokenData::from_token(&payload.token, secret) {
        auth.id
    } else {
        return Err(ResetError::InvalidData("Invalid token".to_string()));
    };
    // update password
    let hash_password = password.hash_password().map_err(|_| {
        ResetError::InvalidData("Invalid password".to_string())
    })?;
    let user_update = UpdateUser {
        id,
        hashed_password: Some(hash_password),
        ..Default::default()
    };
    match repo.update(conn, user_update).await {
        Ok(user) => Ok(user),
        Err(_) => return Err(ResetError::NotFound("Unknown error".to_string())),
    }
}

#[cfg(test)]
mod tests {}