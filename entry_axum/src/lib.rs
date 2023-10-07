use prometheus::Encoder;
use systemstat::{Platform, System};
use axum::{
    Router, http::{StatusCode, Uri}, 
    routing::{get, post}, 
    response::IntoResponse, middleware,
};
use sqlx::{PgPool, migrate::Migrator};
use common::config;
use user::handlers as user_handlers;

mod metrics;


pub async fn router() -> Router {
   
    let sys = System::new();
    let state = config::AppState::new().await;

    run_migrations(&state.db_sql_pool).await;

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
                        .route("create-user-request", post(user_handlers::handle_create_user_cache))
                )
        )
        .layer(middleware::from_fn(metrics::metrics_middleware))
        .fallback(handler_404)
        .with_state(state)
}

async fn run_migrations(pool: &PgPool) {
    static MIGRATOR: Migrator = sqlx::migrate!("./migrations");
    MIGRATOR.run(pool).await.expect("USER_MIGRATOR failed");
}

// handlers

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