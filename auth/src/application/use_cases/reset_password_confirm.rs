use crate::{
    application::port::driven::auth_repository::AuthRepositoryTrait, 
    domain::{auth::Auth, types::{password::Password, token_data::TokenData}}
};


#[derive(Debug)]
pub enum ResetError {
    InvalidData(String),
    Unknown(String),
    Unauthorized(String),
    NotFound(String),
}

pub struct Payload {
    pub token: String,
    pub password: Password,
}

pub async fn execute<T>(
    conn: &T,
    repo: &impl AuthRepositoryTrait<T>,
    secret: &[u8],
    payload: Payload
) -> Result<Auth, ResetError> {
    // Get user id
    let user_id = if let Ok(auth) = TokenData::from_token(&payload.token, secret) {
        auth.id
    } else {
        return Err(ResetError::Unauthorized("Invalid token".to_string()));
    };
    // update password
    let new_hash_password = payload.password.hash_password().map_err(|_| {
        ResetError::InvalidData("Invalid password".to_string())
    })?;
    match repo.update_password(conn, user_id, new_hash_password).await {
        Ok(auth) => Ok(auth),
        Err(_) => return Err(ResetError::NotFound("Unknown error".to_string())),
    }
}

#[cfg(test)]
mod tests {}