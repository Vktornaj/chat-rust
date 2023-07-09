use crate::domain::auth::Auth;
use crate::domain::todo::Todo;

use super::{super::port::driven::todo_repository::TodoRepository};


#[derive(Debug)]
pub enum DeleteError {
    InvalidData(String),
    Unknown(String),
    Conflict(String),
    Unautorized(String),
}

pub async fn execute<T>(
    conn: &T,
    repo: &impl TodoRepository<T>,
    secret: &[u8],
    token: &String, 
    id: i32
) -> Result<Todo, DeleteError> {
    if Auth::from_token(token, secret).is_err() {
        return Err(DeleteError::Unautorized("Invalid token".to_string()));
    };
    if repo.find_one(conn, id).await.is_ok() {
        match repo.delete(conn, id).await {
            Ok(todo) => Ok(todo),
            Err(error) => Err(DeleteError::Unknown(format!("Unknown error: {:?}", error))),
        }
    } else {
        return Err(DeleteError::Conflict("Todo does not exist".to_string()));
    }
}