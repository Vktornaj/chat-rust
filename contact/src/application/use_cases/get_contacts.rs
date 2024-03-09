use crate::{application::port::driven::contact_repository::{
    ContactRepositoryTrait, Error as ContactRepositoryError,
}, domain::contact::Contact};
use auth::TokenData;


pub enum Error {
    NotFound,
    DatabaseError,
    Unauthorized,
}

pub async fn execute<T>(
    conn: &T,
    repo: &impl ContactRepositoryTrait<T>,
    secret: &[u8],
    token: &String,
) -> Result<Vec<Contact>, Error> {
    let id = if let Ok(auth) = TokenData::from_token(token, secret) {
        auth.id
    } else {
        return Err(Error::Unauthorized);
    };

    match repo.get_by_user_id(conn, id.try_into().unwrap()).await {
        Ok(contact) => Ok(contact),
        Err(e) => match e {
            ContactRepositoryError::DatabaseError => Err(Error::DatabaseError),
            ContactRepositoryError::NotFound => Err(Error::NotFound),
        },
    }
}
