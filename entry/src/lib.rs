use std::thread;

use systemstat::{Platform, System, Duration};

use rocket::{catch, catchers, options, get, Request};
use cors::CORS;
use rocket::{launch, routes};
use sqlx::migrate::Migrator;
use deadpool::managed::Pool;
use deadpool_redis::{Manager, Connection};
use prometheus::Encoder;

mod cors;
mod metrics;

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

#[get("/metrics")]
pub fn get_metrics() -> Result<std::string::String, ()> {
    let mut buffer = Vec::new();
    let encoder = prometheus::TextEncoder::new();

    // Gather the metrics.
    let metric_families = prometheus::gather();
    // Encode them to send.
    encoder.encode(&metric_families, &mut buffer).unwrap();

    let output = String::from_utf8(buffer.clone()).unwrap();

    Ok(output)
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

    let sys = System::new();

    thread::spawn(move || loop {
        #[cfg(target_os = "linux")]
        match sys.cpu_load_aggregate() {
            Ok(cpu) => {
                thread::sleep(Duration::from_secs(1));
                let cpu = cpu.done().unwrap();
                metrics::CPU_USAGE.set(f64::trunc(
                    ((cpu.system * 100.0) + (cpu.user * 100.0)).into(),
                ));
            }
            Err(x) => println!("\nCPU load: error: {}", x),
        }
        match sys.memory() {
            Ok(mem) => {
                let memory_used = mem.total.0 - mem.free.0;
                let pourcentage_used = (memory_used as f64 / mem.total.0 as f64) * 100.0;
                metrics::MEM_USAGE.set(f64::trunc(pourcentage_used));
            }
            Err(x) => println!("\nMemory: error: {}", x),
        }
    });

    rocket::custom(config::from_env())
        .attach(CORS)
        .mount(
            "/", 
            routes![
                get_root,
                get_metrics
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
                user_routes::user::password_reset,
                user_routes::user::password_reset_request,
                user_routes::user::update_user_info,
                // todo_routes::todo::post_todo,
                // todo_routes::todo::update_todo,
                // todo_routes::todo::delete_todo,
                // todo_routes::todo::get_todos,
                // todo_routes::todo::put_add_tag,
                // todo_routes::todo::put_remove_tag,
                all_options,
            ]
        )
        .manage::<PgPool>(sqlx_pool)
        .manage::<Pool<Manager, Connection>>(redis_pool)
        .attach(config::AppState::manage())
        .attach(metrics::PrometheusMetrics)
        .register("/", catchers![not_found])
}