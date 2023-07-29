use super::super::port::driven::user_repository::UserRepositoryTrait;


pub async fn execute<T>(
    conn: &T,
    repo: &impl UserRepositoryTrait<T>,
    email: &Option<String>, 
    phone_number: &Option<String>
) -> bool {
    if let Some(email) = email {
        if repo.find_one_by_email(conn, email).await.is_ok() {
            return true;
        }
    }
    if let Some(phone_number) = phone_number {
        if repo.find_one_by_phone_number(conn, phone_number).await.is_ok() {
            return true;
        }
    }
    false
}