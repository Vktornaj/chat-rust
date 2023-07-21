use super::super::port::driven::todo_repository::{TodoRepositoryTrait, FindTodo};
use crate::domain::todo::Status;
use auth::domain::auth::Auth;


#[derive(Debug)]
pub enum DeleteError {
    InvalidData(String),
    Unknown(String),
    Conflict(String),
    Unautorized(String),
}

async fn execute<T>(
    conn: &T,
    repo: &impl TodoRepositoryTrait<T>,
    secret: &[u8], 
    token: &String
) -> Result<(), DeleteError> {
    let username = if let Ok(auth) = Auth::from_token(token, secret) {
        auth.username
    } else {
        return Err(DeleteError::Unautorized("Invalid token".to_string()));
    };
    let find_todo = FindTodo {
        username: (&username).to_owned(),
        title: None, 
        description: None, 
        status: Some(Status::DONE), 
        tags: None,
    };
    match repo.delete_all_criteria(conn, find_todo).await {
        Ok(_) => Ok(()),
        Err(error) => Err(DeleteError::Unknown(format!("Unknown error: {:?}", error))),
    }
}