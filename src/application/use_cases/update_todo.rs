use chrono::Utc;

use super::super::port::driven::todo_repository::{TodoRepository};
use crate::{domain::{{todo::Todo, todo::Status}, auth::Auth}, application::port::driven::todo_repository::UpdateTodo};


#[derive(Debug)]
pub enum UpdateError {
    InvalidData(String),
    Unknown(String),
    Unautorized(String),
}

pub async fn execute<T>(
    conn: &T,
    repo: &impl TodoRepository<T>,
    secret: &[u8],
    token: &String,
    mut update_todo: UpdateTodo
) -> Result<Todo, UpdateError> {
    if let Ok(auth) = Auth::from_token(token, secret) {
        auth.username
    } else {
        return Err(UpdateError::Unautorized("Invalid token".to_string()));
    };
    if update_todo.status.is_some() {
        if &update_todo.status == &Some(Status::DONE)  {
            update_todo.done_date = Some(Some(Utc::now()));
        } else {
            update_todo.done_date = Some(None);
        }
    }
    match repo.update(conn,  update_todo).await {
        Ok(todo) => Ok(todo),
        Err(error) => Err(UpdateError::Unknown(format!("Unknown error: {:?}", error))),
    }
}