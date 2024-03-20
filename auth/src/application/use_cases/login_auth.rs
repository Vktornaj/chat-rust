use crate::domain::types::{
    password::Password, 
    identification::IdentificationValue,
    token_data::TokenData,
};

use super::super::port::driven::auth_repository::AuthRepositoryTrait;


#[derive(Debug)]
pub enum LoginError {
    NotFound,
    Unauthorized,
}

pub struct Payload {
    pub identifier: String,
    pub password: Password,
}

// TODO: improve when criteria will implemented onto the traid
pub async fn execute<T>(
    conn: &T,
    repo: &impl AuthRepositoryTrait<T>, 
    secret: &[u8],
    payload: Payload,
) -> Result<String, LoginError> {
    
    let identifier: IdentificationValue = match IdentificationValue::try_from(payload.identifier) {
        Ok(identifier) => identifier,
        Err(_) => return Err(LoginError::NotFound)
    };
    if let Ok(Some(auth)) = repo.find_by_identification(conn, identifier).await {
        if payload.password.verify_password(&auth.hashed_password).is_ok() {
            Ok(TokenData::new(&auth.user_id.into()).token(secret))
        } else  {
            Err(LoginError::Unauthorized)
        }
    } else {
        Err(LoginError::NotFound)
    }
}

#[cfg(test)]
mod test {
    use super::execute;
    use common::adapter::db::create_pool;
    use crate::{
        adapter::driven::persistence::sqlx::auth_repository::AuthRepository, 
        application::use_cases::login_auth::Payload,
    };
    use common::adapter::config::Config;

    #[tokio::test]
    pub async fn test_login_auth() {

        let pool = create_pool().await;
        let config = Config::new();

        let identifier = "vktornajpro@gmail.com";
        let password = "Password123!";

        let res = execute(
            &pool, 
            &AuthRepository {}, 
            &config.secret, 
            Payload {
                identifier: identifier.to_string(),
                password: password.to_string().try_into().unwrap(),
            }
        ).await;

        println!("{:?}", res);

        assert!(res.is_ok());
    }
        
}