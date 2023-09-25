// use sqlx::migrate::Migrator;

mod domain;
mod application;
mod adapter;

pub use adapter::driving::web::routes;

// pub static MIGRATOR: Migrator = sqlx::migrate!("src/adapter/driven/persistence/sqlx/migrations");