use crate::domain::todo::Todo;
use auth::domain::auth::Auth;

use super::super::port::driven::todo_repository::TodoRepositoryTrait;


#[derive(Debug)]
pub enum FindError {
    Unknown(String),
    Unauthorized(String)
}

async fn execute<T>(
    conn: &T,
    repo: &impl TodoRepositoryTrait<T>,
    secret: &[u8],
    token: &String,
    id: i32
) -> Result<Todo, FindError> {
    if Auth::from_token(token, secret).is_err() {
        return Err(FindError::Unauthorized("Invalid token".to_string()));
    };
    match repo.find_one(conn, id).await.ok() {
        Some(todo) => Ok(todo),
        None => Err(FindError::Unknown("not found".to_string())),
    }
}