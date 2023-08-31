use deadpool_redis::{Config, Runtime, Manager, Connection};
use deadpool::managed::Pool;


pub async fn create_pool() -> Pool<Manager, Connection> {
    let cfg = Config::from_url("redis://127.0.0.1/");
    let pool = cfg.create_pool(Some(Runtime::Tokio1)).unwrap();
    pool
}
