use rocket::{Request, catch, catchers, options, get};
use cors::CORS;
use rocket::{launch, routes};
use sqlx::migrate::Migrator;
use deadpool::managed::Pool;
use deadpool_redis::{Manager, Connection};

mod cors;

use sqlx::PgPool;
use user::routes as user_routes;
use todo::routes as todo_routes;
use common::{config, db, cache};


#[catch(404)]
fn not_found(req: &Request) -> String {
    format!("Sorry, '{}' is not a valid path.", req.uri())
}

/// Catches all OPTION requests in order to get the CORS related Fairing triggered.
#[options("/<_..>")]
fn all_options() {
    /* Intentionally left empty */
}

#[get("/")]
pub fn get_root() -> &'static str {
    "{ \"msg\": \"ok\" }"
}

async fn run_migrations(pool: &PgPool) {
    static MIGRATOR: Migrator = sqlx::migrate!("./migrations");
    MIGRATOR.run(pool).await.expect("USER_MIGRATOR failed");
}

#[launch]
pub async fn rocket() -> _ {
    let sqlx_pool = db::create_pool().await;
    run_migrations(&sqlx_pool).await;
    let redis_pool = cache::create_pool().await;

    rocket::custom(config::from_env())
        .attach(CORS)
        .mount(
            "/", 
            routes![
                get_root
            ]
        )
        .mount(
            "/api", 
            routes![
                user_routes::user::email_available,
                user_routes::user::phone_number_available,
                user_routes::user::create_user_cache,
                user_routes::user::create_user_confirmation,
                user_routes::user::update_user_contact_info_cache,
                user_routes::user::update_user_contact_info_confirmation,
                user_routes::user::login,
                user_routes::user::get_user_info,
                todo_routes::todo::post_todo,
                todo_routes::todo::update_todo,
                todo_routes::todo::delete_todo,
                todo_routes::todo::get_todos,
                todo_routes::todo::put_add_tag,
                todo_routes::todo::put_remove_tag,
                all_options,
            ]
        )
        .manage::<PgPool>(sqlx_pool)
        .manage::<Pool<Manager, Connection>>(redis_pool)
        .attach(config::AppState::manage())
        .register("/", catchers![not_found])
}