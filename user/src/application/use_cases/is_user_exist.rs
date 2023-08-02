use crate::application::port::driven::user_repository::FindUser;

use super::super::port::driven::user_repository::UserRepositoryTrait;


pub async fn execute<T>(
    conn: &T,
    repo: &impl UserRepositoryTrait<T>,
    email: &Option<String>, 
    phone_number: &Option<String>
) -> bool {
    let find_user =  FindUser { 
        email: email.to_owned(),
        phone_number: phone_number.to_owned(),
        birthday: None,
        nationality: None,
        languages: None,
        created_at: None
    };
    repo.find_by_criteria(conn, &find_user, 0, 1).await
        .is_ok_and(|x| x.len() > 0)
}