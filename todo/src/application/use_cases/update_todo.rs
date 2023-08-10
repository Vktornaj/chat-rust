use chrono::Utc;

use super::super::port::driven::todo_repository::TodoRepositoryTrait;
use crate::{
    domain::{todo::Todo, todo::Status}, 
    application::port::driven::todo_repository::UpdateTodo
};
use auth::domain::auth::Auth;


#[derive(Debug)]
pub enum UpdateError {
    InvalidData(String),
    Unknown(String),
    Unautorized(String),
    NotFound(String),
}

pub async fn execute<T>(
    conn: &T,
    repo: &impl TodoRepositoryTrait<T>,
    secret: &[u8],
    token: &String,
    mut update_todo: UpdateTodo
) -> Result<Todo, UpdateError> {
    let user_id = if let Ok(auth) = Auth::from_token(token, secret) {
        auth.id
    } else {
        return Err(UpdateError::Unautorized("Invalid token".to_string()));
    };
    let todo = if let Ok(todo) = repo.find_by_id(conn, update_todo.id).await {
        todo
    } else {
        return Err(UpdateError::NotFound(format!("")));
    };
    if todo.user_id != Some(user_id) {
        return Err(UpdateError::NotFound(format!("")));
    }
    if update_todo.status.is_some() {
        if &update_todo.status == &Some(Status::DONE)  {
            update_todo.done_date = Some(Some(Utc::now()));
        } else {
            update_todo.done_date = Some(None);
        }
    }
    match repo.update(conn,  update_todo).await {
        Ok(todo) => Ok(todo),
        Err(error) => {
            Err(UpdateError::Unknown(format!("Unknown error: {:?}", error)))
        },
    }
}