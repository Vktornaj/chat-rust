use diesel_migrations::{embed_migrations, EmbeddedMigrations};

mod domain;
mod application;
mod adapter;

pub use adapter::driving::web::routes;

pub const MIGRATION: EmbeddedMigrations = embed_migrations!("src/adapter/driven/persistence/pgsql/migrations");
