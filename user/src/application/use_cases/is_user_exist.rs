use super::super::port::driven::user_repository::UserRepositoryTrait;


pub async fn execute<T>(conn: &T, repo: &impl UserRepositoryTrait<T>, username: &String) -> bool {
    repo.find_one(conn, username).await.is_ok()
}