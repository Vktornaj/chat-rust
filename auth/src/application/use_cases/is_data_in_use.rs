use common::domain::types::error::ErrorMsg;

use crate::{application::port::driven::auth_repository::AuthRepositoryTrait, domain::types::identification::IdentificationValue};


pub struct Payload {
    pub identify_value: String,
    pub identify_type: String
}

pub async fn execute<T>(
    conn: &T,
    repo: &impl AuthRepositoryTrait<T>,
    payload: Payload,
) -> Result<bool, ErrorMsg> {
    let identification = IdentificationValue::from_string(
        payload.identify_value,
        payload.identify_type
    )?;

    let auth = repo.find_by_identification(conn, identification).await?;

    Ok(auth.is_some())
}