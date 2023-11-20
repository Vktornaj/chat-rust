use sqlx::postgres::PgPoolOptions;
use std::env;

// Function to create the database pool.
pub async fn create_pool() -> sqlx::Pool<sqlx::Postgres> {
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("Failed to create database pool.")
}