use rocket::{Request, catch, catchers, options, get};
use cors::CORS;
use rocket::{launch, routes};
use dotenv::dotenv;

mod cors;

// use adapter::driving::web::routes;
use user::routes as user_routes;
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

#[launch]
pub fn rocket() -> _ {
    dotenv().ok();
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
                user_routes::user::username_available,
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
        .attach(db::Db::fairing())
        .attach(config::AppState::manage())
        .register("/", catchers![not_found])
}