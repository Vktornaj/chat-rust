use common::domain::types::id::Id;

use crate::{
    application::port::driven::auth_repository::{self, AuthRepositoryTrait},
    domain::types::identification::IdentificationValue,
    TokenData,
};

pub enum Error {
    NotFound,
    Unknown(String),
    Unauthorized,
}

pub struct Payload {
    pub identify_type: String,
    pub identify_value: String,
}

pub async fn execute<T>(
    secret: &[u8],
    conn: &T,
    repository: &impl AuthRepositoryTrait<T>,
    token: String,
    payload: Payload,
) -> Result<Id, Error> {
    TokenData::from_token(&token, secret).map_err(|_| Error::Unauthorized)?;

    let identifier =
        IdentificationValue::from_string(payload.identify_value, payload.identify_type)
            .map_err(|err| Error::Unknown(err))?;

    let user = match repository.find_by_identification(&conn, identifier).await {
        Ok(user) => user,
        Err(auth_repository::Error::NotFound) => return Err(Error::NotFound),
        Err(err) => return Err(Error::Unknown(err.to_string())),
    };

    let user = user.ok_or(Error::NotFound)?;

    Ok(user.user_id)
}
