use rocket::{Request, catch, catchers, options, get};
use cors::CORS;
use rocket::{launch, routes};

mod cors;

use sqlx::PgPool;
use user::routes as user_routes;
use user::MIGRATOR as USER_MIGRATOR;
use todo::routes as todo_routes;
use common::{config, db};


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
    USER_MIGRATOR.run(pool).await.expect("USER_MIGRATOR failed");
}

#[launch]
pub async fn rocket() -> _ {
    let sqlx_pool = db::create_pool().await;
    run_migrations(&sqlx_pool).await;

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
                user_routes::user::create_user,
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
        .attach(config::AppState::manage())
        .register("/", catchers![not_found])
}