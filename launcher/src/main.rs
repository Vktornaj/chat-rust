use diesel::{PgConnection};
use diesel_migrations::{MigrationHarness, EmbeddedMigrations};
// use todo_rust;
use common::config::establish_connection_pg;
// use rocket::{self};
use user::MIGRATION as user_migration;
use todo::MIGRATION as todo_migration;


fn run_migrations(conn: &mut PgConnection, migrations: Vec<EmbeddedMigrations>) {
    for migration in migrations {
        conn.run_pending_migrations(migration).unwrap();
    }
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let conn = &mut establish_connection_pg();
    run_migrations(conn, vec![user_migration, todo_migration]);
    let _rocket = launcher::rocket()
        .ignite().await?
        .launch().await?;
    Ok(())
}