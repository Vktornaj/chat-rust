use prometheus::Encoder;
use systemstat::{Platform, System};
use axum::{
    Router, http::{StatusCode, Uri}, 
    routing::{get, post, put, delete}, 
    response::IntoResponse, middleware,
};
use sqlx::{PgPool, migrate::Migrator};
use common::config;
use user::handlers as user_handlers;

mod metrics;


pub async fn router() -> Router {
   
    let sys = System::new();
    let app_state = config::AppState::new().await;

    run_migrations(&app_state.db_sql_pool).await;

    tokio::spawn(async move { 
        loop {
            // sleep for 1 second
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
            #[cfg(target_os = "linux")]
            match sys.cpu_load_aggregate() {
                Ok(cpu) => {
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
        }
    });

    Router::new()
        .route("/", get(handler_get_root))
        .route("/metrics", get(handler_metrics))
        .nest(
            "/api",
            Router::new()
                .nest(
                    "/user", 
                    Router::new()
                        .route("/create-user-request", post(user_handlers::handle_create_user_cache))
                        .route("/create-use-confirmation", post(user_handlers::handle_create_user_confirmation))
                        .route("/get-user", get(user_handlers::handle_get_user_info))
                        .route("/update-user", put(user_handlers::handle_update_user_info))
                        .route("/delete-user", delete(user_handlers::handle_delete_account))
                        .route("/email-available/:email", get(user_handlers::handle_email_available))
                        .route("/phone-number-available/:phone", get(user_handlers::handle_phone_number_available))
                        .route("/login", post(user_handlers::handle_login))
                        .route("/update-password", put(user_handlers::handle_update_password))
                        .route("/update-user-contact-info-cache", put(user_handlers::handle_update_user_contact_info_cache))
                        .route("/update-user-contact-info-confirmation", put(user_handlers::handle_update_user_contact_info_confirmation))
                        .route("/password-recovery-request", post(user_handlers::handle_password_recovery_request))
                        .route("/password-reset-confirmation/:token", put(user_handlers::handle_password_reset_confirmation))
                )
        )
        .layer(middleware::from_fn(metrics::metrics_middleware))
        .fallback(handler_404)
        .with_state(app_state)
}

async fn run_migrations(pool: &PgPool) {
    static MIGRATOR: Migrator = sqlx::migrate!("./migrations");
    MIGRATOR.run(pool).await.expect("USER_MIGRATOR failed");
}

// root handlers

async fn handler_404(uri: Uri) -> impl IntoResponse {
    (StatusCode::NOT_FOUND, format!("No route for {}", uri))
}

async fn handler_get_root() -> &'static str {
    "ok"
}

async fn handler_metrics() -> std::string::String {
    let mut buffer = Vec::new();
    let encoder = prometheus::TextEncoder::new();

    // Gather the metrics.
    let metric_families = prometheus::gather();
    // Encode them to send.
    encoder.encode(&metric_families, &mut buffer).unwrap();

    String::from_utf8(buffer.clone()).unwrap()
}