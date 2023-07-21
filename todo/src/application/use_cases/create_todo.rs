use super::super::port::driven::todo_repository::{TodoRepositoryTrait, FindTodo};
use crate::{domain::todo::Todo, application::port::driven::errors::RepoSelectError};
use auth::domain::auth::Auth;


#[derive(Debug)]
pub enum CreateError {
    InvalidData(String),
    Unknown(String),
    Conflict(String),
    Unautorized(String),
}

pub async fn execute<T>(
    conn: &T,
    repo: &impl TodoRepositoryTrait<T>,
    secret: &[u8],
    token: &String,
    todo: Todo
) -> Result<Todo, CreateError> {
    let username = if let Ok(auth) = Auth::from_token(token, secret) {
        auth.username
    } else {
        return Err(CreateError::Unautorized("Invalid token".to_string()));
    };
    let find_todo = FindTodo {
        username: (&username).to_owned(),
        title: Some(todo.title.to_owned()),
        description: None,
        status: None,
        tags: None,
    };
    println!("Begining");
    println!("user: {}", &username);
    let res = repo.find_one_criteria(conn, &username, find_todo).await;
    println!("res: {:?}", res);
    if res.is_ok() {
        return Err(CreateError::Conflict("Title already exist".to_string()));
    }
    println!("Title does not exist");
    if let Err(RepoSelectError::Unknown(msg)) = res {
        return Err(CreateError::Unknown(msg));
    }
    println!("RepoSelectError");
    match repo.create(conn, &username, todo).await {
        Ok(todo) => Ok(todo),
        Err(error) => Err(CreateError::Unknown(format!("Unknown error: {:?}", error))),
    }
}