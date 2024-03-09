use crate::application::port::driven::contact_repository::{
    ContactRepositoryTrait, Error as ContactRepositoryError,
};
use common::domain::types::id::Id;

pub enum Error {
    NotFound,
    DatabaseError,
    Unauthorized,
}

pub struct Payload {
    user_id: Id,
    contact_id: Id,
}

pub async fn execute<T>(
    conn: &T,
    repo: &impl ContactRepositoryTrait<T>,
    payload: Payload,
) -> Result<bool, Error> {
    match repo
        .get_by_id(
            conn,
            payload.user_id.try_into().unwrap(),
            payload.contact_id.try_into().unwrap(),
        )
        .await
    {
        Ok(contact) => {
            if contact.is_blocked {
                return Ok(true);
            }
            return Ok(false);
        }
        Err(e) => match e {
            ContactRepositoryError::DatabaseError => Err(Error::DatabaseError),
            ContactRepositoryError::NotFound => Err(Error::NotFound),
        },
    }
}
