use auth::TokenData;
use common::domain::types::id::Id;
use crate::application::port::driven::contact_repository::{ContactRepositoryTrait, Error as ContactRepositoryError};


pub enum Error {
    NotFound,
    DatabaseError,
    Unauthorized,
}

pub struct Payload {
    pub id: Id,
}

pub async fn execute<T>(
    conn: &T,
    repo: &impl ContactRepositoryTrait<T>,
    secret: &[u8],
    token: &String,
    payload: Payload,
) -> Result<(), Error> {
    let id = if let Ok(auth) = TokenData::from_token(token, secret) {
        auth.id
    } else {
        return Err(Error::Unauthorized);
    };

    match repo.delete(conn, id.try_into().unwrap(), payload.id).await {
        Ok(_) => Ok(()),
        Err(e) => match e {
            ContactRepositoryError::DatabaseError => Err(Error::DatabaseError),
            ContactRepositoryError::NotFound => Err(Error::NotFound),
        },
    }
}