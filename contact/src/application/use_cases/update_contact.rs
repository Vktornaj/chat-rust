use auth::TokenData;
use common::domain::types::id::Id;
use crate::{
    application::port::driven::contact_repository::{
        ContactRepositoryTrait, 
        Error as ContactRepositoryError, 
        UpdateContact
    }, 
    domain::{contact::Contact, types::alias::Alias},
};


pub enum Error {
    NotFound,
    DatabaseError,
    Unauthorized,
}

pub struct Payload {
    pub id: Id,
    pub alias: Option<Option<Alias>>,
    pub blocked: Option<bool>,
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

    let update_contact = UpdateContact {
        id: payload.id,
        user_id: id.try_into().unwrap(),
        alias: payload.alias.map(|x| x.map(|x| x.into())),
        is_blocked: payload.blocked,
    };

    match repo.update(conn, update_contact).await {
        Ok(contact) => Ok(contact),
        Err(e) => match e {
            ContactRepositoryError::DatabaseError => Err(Error::DatabaseError),
            ContactRepositoryError::NotFound => Err(Error::NotFound),
        },
    }
}