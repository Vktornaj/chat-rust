use auth::domain::auth::Auth;
use crate::domain::todo::Todo;

use super::super::port::driven::todo_repository::TodoRepositoryTrait;


#[derive(Debug)]
pub enum DeleteError {
    InvalidData(String),
    Unknown(String),
    Conflict(String),
    NotFound(String),
    Unautorized(String),
}

pub async fn execute<T>(
    conn: &T,
    repo: &impl TodoRepositoryTrait<T>,
    secret: &[u8],
    token: &String, 
    id: i32
) -> Result<Todo, DeleteError> {
    let user_id = if let Ok(auth) = Auth::from_token(token, secret) {
        auth.id
    } else {
        return Err(DeleteError::Unautorized("Invalid token".to_string()));
    };
    let todo = if let Ok(todo) = repo.find_by_id(conn, id).await {
        todo
    } else {
        return Err(DeleteError::NotFound(format!("")));
    };
    if todo.user_id != Some(user_id) {
        return Err(DeleteError::NotFound(format!("")));
    }
    match repo.delete(conn, id).await {
        Ok(todo) => Ok(todo),
        Err(err) => Err(DeleteError::Unknown(format!("Unknown error: {:?}", err))),
    }
}