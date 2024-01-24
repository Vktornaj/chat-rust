use crate::{application::port::driven::auth_repository::{AuthRepositoryTrait, RepoSelectError}, domain::types::identification::IdentificationValue};


pub struct Payload {
    pub identify_value: String,
    pub identify_type: String
}

pub async fn execute<T>(
    conn: &T,
    repo: &impl AuthRepositoryTrait<T>,
    payload: Payload,
) -> Result<bool, String> {
    let identification = IdentificationValue::from_string(
        payload.identify_value,
        payload.identify_type
    )?;

    match repo.find_by_identification(conn, identification).await{
        Ok(res) => Ok(false),
        Err(err) => match err {
            RepoSelectError::NotFound(_) => Ok(true),
            RepoSelectError::Unknown(err) => Err(err)
        } 
    }
}
