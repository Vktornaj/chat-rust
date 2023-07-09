use crate::domain::{todo::Todo, auth::Auth};

use super::super::port::driven::todo_repository::TodoRepository;


#[derive(Debug)]
pub enum UpdateError {
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
    todo_id: i32, 
    tag: &String
) -> Result<Todo, UpdateError> {
    let username = if let Ok(auth) = Auth::from_token(token, secret) {
        auth.username
    } else {
        return Err(UpdateError::Unautorized("Invalid token".to_string()));
    };
    let todo = if let Ok(todo) = repo.find_one(conn, todo_id).await {
        todo
    } else {
        return Err(UpdateError::Unknown(format!("Unknown error")));
    };

    if todo.tags.contains(tag) {
        return Err(UpdateError::Conflict(format!("Tag already exist")));
    }

    match repo.add_tag(conn, todo_id, &tag).await {
        Ok(todo) => Ok(todo),
        Err(error) => Err(UpdateError::Unknown(format!("Unknown error: {:?}", error))),
    }
}