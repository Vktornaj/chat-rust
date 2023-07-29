use super::super::port::driven::user_repository::UserRepositoryTrait;
use auth::domain::auth::Auth;


#[derive(Debug)]
pub enum LoginError {
    InvalidData(String),
    Unknown(String),
    Conflict(String)
}

// TODO: improve when criteria will implemented onto the traid
pub async fn execute<T>(
    conn: &T,
    repo: &impl UserRepositoryTrait<T>, 
    secret: &[u8],
    email: &Option<String>,
    phone_number: &Option<String>,
    password: &String
) -> Result<String, LoginError> {
    let user = {
        if let Some(email) = email {
            if let Ok(user) = repo.find_one_by_email(conn, email).await {
                user
            } else {
                return Err(LoginError::InvalidData("User not found".to_string()));
            }
        } else if let Some(phone_number) = phone_number {
            if let Ok(user) = repo.find_one_by_phone_number(conn, phone_number).await {
                user
            } else {
                return Err(LoginError::InvalidData("User not found".to_string()));
            }
        } else {
            return Err(LoginError::InvalidData(
                "A email or phone number must be specified".to_string())
            )
        }
    };

    if user.verify_password(password).is_ok() {
        Ok(Auth::new(&user.id.unwrap()).token(secret))
    } else  {
        Err(LoginError::InvalidData("Invalid password".to_string()))
    }
}