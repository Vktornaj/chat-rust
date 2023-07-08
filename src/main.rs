mod config;
mod domain;
mod application;
mod adapter;

use diesel::{PgConnection};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
// use todo_rust;
use config::establish_connection_pg;
// use rocket::{self};


pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("src/adapter/driven/persistence/pgsql/migrations");

fn run_migration(conn: &mut PgConnection) {
    conn.run_pending_migrations(MIGRATIONS).unwrap();
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let conn = &mut establish_connection_pg();
    run_migration(conn);
    let _rocket = todo_rust::rocket()
        .ignite().await?
        .launch().await?;
    Ok(())
}