use super::super::port::driven::user_repository::UserRepository;
use crate::domain::user::User;
use super::is_user_exist;


#[derive(Debug)]
pub enum CreateError {
    InvalidData(String),
    Unknown(String),
    Conflict(String)
}

pub async fn execute<T>(conn: &T, repo: &impl UserRepository<T>, mut user: User) -> Result<User, CreateError> {
    if is_user_exist::execute(conn, repo, &user.username).await {
        return Err(CreateError::Conflict("Username is already in use".to_string()))
    }
    if user.hash_password_mut().is_err() {
        return Err(CreateError::InvalidData("Invalid password".to_string()));
    }
    match repo.create(conn, user).await {
        Ok(user) => Ok(user),
        Err(error) => Err(CreateError::Unknown(format!("Unknown error: {:?}", error))),
    }
}