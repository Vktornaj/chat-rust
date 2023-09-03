use deadpool_redis::{Config, Runtime, Manager, Connection};
use deadpool::managed::Pool;


pub async fn create_pool() -> Pool<Manager, Connection> {
    let cfg = Config::from_url("redis://localhost:6379/");
    let pool = cfg.create_pool(Some(Runtime::Tokio1)).unwrap();
    pool
}


// test connection
#[cfg(test)]
mod tests {
    use deadpool_redis::redis::AsyncCommands;
    use rocket::tokio;

    use super::*;

    #[tokio::test]
    async fn test_redis() {
        let pool = create_pool().await;
        let mut conn = pool.get().await.unwrap();
        let _: () = conn.set("deadpool", "redis").await.unwrap();
        let res: String = conn.get("deadpool").await.unwrap();
        assert_eq!(res, "redis");
    }
}
