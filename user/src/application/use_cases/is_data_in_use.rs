use crate::{
    application::port::driven::user_repository::FindUser, domain::types::{
        email::Email, error::ErrorMsg, phone_number::PhoneNumber
    }
};

use super::super::port::driven::user_repository::UserRepositoryTrait;


pub struct Payload {
    pub email: Option<String>,
    pub phone_number: Option<String>
}

pub async fn execute<T>(
    conn: &T,
    repo: &impl UserRepositoryTrait<T>,
    payload: Payload,
) -> Result<bool, ErrorMsg> {
    if let Some(email) = payload.email {
        let find_user_email =  FindUser { 
            email: Some(Email::try_from(email.to_owned())?),
            phone_number: None,
            birthday: None,
            nationality: None,
            languages: None,
            created_at: None
        };
        if repo.find_by_criteria(conn, find_user_email, 0, 1).await
            .is_ok_and(|x| x.len() > 0) {
            return Ok(true);
        }
    }
    if let Some(phone_number) = payload.phone_number {
        let find_user_phone =  FindUser { 
            email: None,
            phone_number: Some(PhoneNumber::try_from(phone_number.to_owned())?),
            birthday: None,
            nationality: None,
            languages: None,
            created_at: None
        };
        if repo.find_by_criteria(conn, find_user_phone, 0, 1).await
            .is_ok_and(|x| x.len() > 0) {
            return Ok(true);
        }
    }
    Ok(false)
}