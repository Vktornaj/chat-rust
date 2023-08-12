use crate::{application::port::driven::user_repository::FindUser, domain::user::{PhoneNumber, Email}};

use super::super::port::driven::user_repository::UserRepositoryTrait;


pub async fn execute<T>(
    conn: &T,
    repo: &impl UserRepositoryTrait<T>,
    email: &Option<Email>, 
    phone_number: &Option<PhoneNumber>
) -> bool {
    let find_user_email =  FindUser { 
        email: email.to_owned(),
        phone_number: None,
        birthday: None,
        nationality: None,
        languages: None,
        created_at: None
    };
    let find_user_phone =  FindUser { 
        email: None,
        phone_number: phone_number.to_owned(),
        birthday: None,
        nationality: None,
        languages: None,
        created_at: None
    };
    repo.find_by_criteria(conn, find_user_email, 0, 1).await
        .is_ok_and(|x| x.len() > 0)
    || repo.find_by_criteria(conn, find_user_phone, 0, 1).await
        .is_ok_and(|x| x.len() > 0)
}