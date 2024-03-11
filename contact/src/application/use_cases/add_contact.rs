use auth::TokenData;
use common::domain::types::id::Id;
use crate::{
    application::port::driven::contact_repository::{ContactRepositoryTrait, Error as ContactRepositoryError}, 
    domain::{contact::{Contact, NewContact}, types::alias::Alias},
};


pub enum Error {
    NotFound,
    DatabaseError,
    Unauthorized,
}

pub struct Payload {
    pub id: Id,
    pub alias: Option<Alias>,
    pub is_blocked: bool,
}

pub async fn execute<T>(
    conn: &T,
    repo: &impl ContactRepositoryTrait<T>,
    secret: &[u8],
    token: &String,
    payload: Payload,
) -> Result<Contact, Error> {
    let id = if let Ok(auth) = TokenData::from_token(token, secret) {
        auth.id
    } else {
        return Err(Error::Unauthorized);
    };

    let new_contact = NewContact {
        id: payload.id,
        user_id: id.try_into().unwrap(),
        alias: payload.alias,
        is_blocked: payload.is_blocked,
    };

    match repo.create(conn, new_contact).await {
        Ok(contact) => Ok(contact),
        Err(e) => match e {
            ContactRepositoryError::DatabaseError => Err(Error::DatabaseError),
            ContactRepositoryError::NotFound => Err(Error::NotFound),
        },
    }
}

// Tests
#[cfg(test)]
mod test {
    use common::adapter::state;
    use crate::adapter::driven::persistence::sqlx::contact_repository::ContactRepository;
    use super::*;


    static USER_TOKEN: &'static str = "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJleHAiOjE3MTUzNjM5NjQsImlkIjoiOWEwODkzNWItMmM5Yy00ZTY2LWExYmEtNGQxNzQ3YzAzNzg0IiwidGtuX2lkIjoiOWE5YmYzMzEtODhkNi00NjY1LTk3ZWItZTJlNjEyNzUyNGZiIn0.u8qF29U2P309muZ5gmp4B6DZg5MgV4Bexi1fsNP70sk";

    #[tokio::test]
    async fn create_ok() {
        let state = state::AppState::new().await;
        let res = execute(
            &state.db_sql_pool,
            &ContactRepository(),
            &state.config.secret,
            &USER_TOKEN.to_string(),
            Payload {
                id: String::from("f8772297-b06c-48e5-9602-92c9013f00c7").try_into().unwrap(),
                alias: Some("Pepe".to_string().try_into().unwrap()),
                is_blocked: false,
            },
        ).await;

        assert!(res.is_ok());
    }
}