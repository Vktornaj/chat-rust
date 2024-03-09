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
        blocked: false,
    };

    match repo.create(conn, new_contact).await {
        Ok(contact) => Ok(contact),
        Err(e) => match e {
            ContactRepositoryError::DatabaseError => Err(Error::DatabaseError),
            ContactRepositoryError::NotFound => Err(Error::NotFound),
        },
    }
}
