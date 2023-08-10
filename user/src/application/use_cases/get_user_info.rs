use auth::domain::auth::Auth;
use crate::domain::user::User;

use super::super::port::driven::user_repository::UserRepositoryTrait;


#[derive(Debug)]
pub enum FindError {
    Unknown(String),
    Unautorized(String)
}

pub async fn execute<T>(
    conn: &T,
    repo: &impl UserRepositoryTrait<T>,
    secret: &[u8],
    token: &String
) -> Result<User, FindError> {
    let id = if let Ok(auth) = Auth::from_token(token, secret) {
        auth.id
    } else {
        return Err(FindError::Unautorized("Invalid token".to_string()));
    };
    match repo.find_by_id(conn, id).await {
        Ok(user) => Ok(user),
        Err(_) => Err(FindError::Unknown("user not found".to_string())),
    }
}