use crate::domain::{todo::Todo, auth::Auth};

use super::super::port::driven::todo_repository::TodoRepository;


#[derive(Debug)]
pub enum FindError {
    Unknown(String),
    Unauthorized(String)
}

async fn execute<T>(
    conn: &T,
    repo: &impl TodoRepository<T>,
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