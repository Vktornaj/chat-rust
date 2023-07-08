use crate::domain::{todo::Todo, auth::Auth};

use super::super::port::driven::todo_repository::TodoRepository;


#[derive(Debug)]
pub enum FindAllError {
    Unknown(String),
    Unautorized(String),
}

pub async fn execute<T>(
    conn: &T,
    repo: &impl TodoRepository<T>, 
    secret: &[u8],
    token: &String, 
    from: i32, 
    to: i32
) -> Result<Vec<Todo>, FindAllError> {
    let username = if let Ok(auth) = Auth::from_token(token, secret) {
        auth.username
    } else {
        return Err(FindAllError::Unautorized("Invalid token".to_string()));
    };
    match repo.find_all(conn, &username, from, from + to).await.ok() {
        Some(todo) => Ok(todo),
        None => Err(FindAllError::Unknown("not found".to_string())),
    }
}