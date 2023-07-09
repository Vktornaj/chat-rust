use super::super::port::driven::user_repository::UserRepository;


pub async fn execute<T>(conn: &T, repo: &impl UserRepository<T>, username: &String) -> bool {
    repo.find_one(conn, username).await.is_ok()
}