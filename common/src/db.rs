use sqlx::postgres::PgPoolOptions;

// Function to create the database pool.
pub async fn create_pool() -> sqlx::Pool<sqlx::Postgres> {
    let db_url = "postgres://username:password@localhost:5432/dbname"; // Replace with your PostgreSQL URL
    PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("Failed to create database pool.")
}
