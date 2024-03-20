use auth::TokenData;
use crate::domain::profile::Profile;
use super::super::port::driven::user_repository::ProfileRepositoryTrait;


#[derive(Debug)]
pub enum FindError {
    Unknown(String),
    Unauthorized(String)
}

impl std::fmt::Display for FindError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            FindError::Unknown(msg) => write!(f, "Unknown error: {}", msg),
            FindError::Unauthorized(msg) => write!(f, "Unauthorized: {}", msg),
        }
    }
}

pub async fn execute<T>(
    conn: &T,
    repo: &impl ProfileRepositoryTrait<T>,
    secret: &[u8],
    token: &String
) -> Result<Profile, FindError> {
    let id = if let Ok(auth) = TokenData::from_token(token, secret) {
        auth.id
    } else {
        return Err(FindError::Unauthorized("Invalid token".to_string()));
    };
    match repo.find_by_id(conn, id).await {
        Ok(user) => Ok(user),
        Err(_) => Err(FindError::Unknown("user not found: ".to_string() + &id.to_string())),
    }
}