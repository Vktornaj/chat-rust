use prometheus::Encoder;
use systemstat::{Platform, System};
use axum::{
    Router, http::{StatusCode, Uri}, 
    routing::get, 
    response::IntoResponse, middleware,
};
use common::{db, cache};
use sqlx::{PgPool, migrate::Migrator};
use deadpool_redis::{Manager, Connection};
use deadpool::managed::Pool;

mod metrics;


#[derive(Clone)]
struct MyAppState {
    db_sql_pool: PgPool,
    cache_pool: Pool<Manager, Connection>,
}

pub async fn router() -> Router {
    let sqlx_pool = db::create_pool().await;
    let redis_pool = cache::create_pool().await;
    let sys = System::new();

    run_migrations(&sqlx_pool).await;

    let state = MyAppState { 
        db_sql_pool: sqlx_pool,
        cache_pool: redis_pool,
    };

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
        .route("/", get(handle_get_root))
        .route("/metrics", get(get_metrics))
        .layer(middleware::from_fn(metrics::metrics_middleware))
        .fallback(handler_404)
        .with_state(state)
}

async fn run_migrations(pool: &PgPool) {
    static MIGRATOR: Migrator = sqlx::migrate!("./migrations");
    MIGRATOR.run(pool).await.expect("USER_MIGRATOR failed");
}

async fn handler_404(uri: Uri) -> impl IntoResponse {
    (StatusCode::NOT_FOUND, format!("No route for {}", uri))
}

async fn handle_get_root() -> &'static str {
    "ok"
}

async fn get_metrics() -> std::string::String {
    let mut buffer = Vec::new();
    let encoder = prometheus::TextEncoder::new();

    // Gather the metrics.
    let metric_families = prometheus::gather();
    // Encode them to send.
    encoder.encode(&metric_families, &mut buffer).unwrap();

    String::from_utf8(buffer.clone()).unwrap()
}