use auth::domain::auth::Auth;
use axum::{
    extract::{
        ws::{Message, WebSocket},
        State, WebSocketUpgrade,
    },
    headers::{authorization::Bearer, Authorization},
    http::{StatusCode, Uri, Method},
    middleware,
    response::{IntoResponse, Response},
    routing::{delete, get, post, put},
    Router, 
    TypedHeader,
    http::HeaderValue,
};
use futures_util::stream::SplitSink;
use prometheus::Encoder;
use sqlx::{migrate::Migrator, PgPool};
use systemstat::{Platform, System};
use tower::ServiceBuilder;
use tower_http::{
    trace::{DefaultMakeSpan, TraceLayer}, 
    cors::{CorsLayer, Any},
};

use common::adapter::state::AppState;
use common::domain::models::{
    client::Clients,
    event::EventQueue,
    message::Message as MessageDomain,
};

mod metrics;
mod ws;
use user::handlers as user_handlers;


pub async fn router() -> Router {
    let sys: System = System::new();
    let app_state = AppState::new().await;

    // run migrations
    run_migrations(&app_state.db_sql_pool).await;

    // new thread to listen to event queue
    run_consumer_event_queue(app_state.event_queue.clone(), app_state.clients.clone()).await;

    // new thread to get metrics
    run_geting_metricts(sys);

    Router::new()
        .route("/", get(handler_get_root))
        .route("/metrics", get(handler_metrics))
        .nest(
            "/api",
            Router::new()
                .nest(
                    "/user",
                    Router::new()
                        .route(
                            "/create-user-request",
                            post(user_handlers::handle_create_user_cache),
                        )
                        .route(
                            "/create-user-confirmation",
                            post(user_handlers::handle_create_user_confirmation),
                        )
                        .route("/get-user", get(user_handlers::handle_get_user_info))
                        .route("/update-user", put(user_handlers::handle_update_user_info))
                        .route("/delete-user", delete(user_handlers::handle_delete_account))
                        .route(
                            "/email-available/:email",
                            get(user_handlers::handle_email_available),
                        )
                        .route(
                            "/phone-number-available/:phone",
                            get(user_handlers::handle_phone_number_available),
                        )
                        .route("/login", post(user_handlers::handle_login))
                        .route(
                            "/update-password",
                            put(user_handlers::handle_update_password),
                        )
                        .route(
                            "/update-user-contact-info-cache",
                            put(user_handlers::handle_update_user_contact_info_cache),
                        )
                        .route(
                            "/update-user-contact-info-confirmation",
                            put(user_handlers::handle_update_user_contact_info_confirmation),
                        )
                        .route(
                            "/password-recovery-request",
                            post(user_handlers::handle_password_recovery_request),
                        )
                        .route(
                            "/password-reset-confirmation/:token",
                            put(user_handlers::handle_password_reset_confirmation),
                        ),
                )
                .nest("/message", Router::new().route("/ws", get(ws_handler))),
        )
        .layer(
            ServiceBuilder::new()
                .layer(middleware::from_fn(metrics::metrics_middleware))
                .layer(
                    TraceLayer::new_for_http()
                        .make_span_with(DefaultMakeSpan::default().include_headers(true)),
                )
                .layer(
                    CorsLayer::new()
                        .allow_methods([ Method::GET, Method::POST, Method::PUT, Method::DELETE])
                        .allow_headers(Any)
                        .allow_origin([
                            "http://localhost:5173".parse::<HeaderValue>().unwrap(),
                            "http://192.168.1.120:5173".parse::<HeaderValue>().unwrap(),
                            "http://192.168.1.120".parse::<HeaderValue>().unwrap(),
                        ])
                    )
        )
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

// Websocket handlers
pub async fn ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
    TypedHeader(token): TypedHeader<Authorization<Bearer>>,
) -> Response {
    let user_id = if let Ok(auth) = Auth::from_token(
        &token.token().to_string(), 
        &state.config.secret
    ) {
        auth.id
    } else {
        return StatusCode::UNAUTHORIZED.into_response();
    };
    ws.on_upgrade(move |socket| {
        ws::client_connect::execute(
            state.clients,
            state.event_queue,
            user_id,
            socket,
        )
    })
}

fn run_geting_metricts(sys: System) {
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
}

async fn run_consumer_event_queue(
    event_queue: EventQueue<MessageDomain>, 
    clients: Clients<SplitSink<WebSocket, Message>>,
) {
    ws::consume_event::execute(clients, event_queue).await;
}

// fn run_producer_event_queue(
//     event_queue: EventQueue,
//     clients: Clients<SplitSink<WebSocket, Message>, SplitStream<WebSocket>>,
// ) {
//     tokio::spawn(async move {
//         let (tx, mut rx) = mpsc::channel(32);

//         tokio::spawn(async move {
//             let mut clients_cloned = clients.write().await;
//             let extract_receiver = |(_, client): (&Uuid, &mut Client<SplitSink<WebSocket, Message>, SplitStream<WebSocket>>)| {
//                 client.receiver.take()
//             };
//             let mut receivers = vec![clients_cloned
//                 .iter_mut()
//                 .filter_map(extract_receiver)];
//             let clients_cloned_2 = clients.clone();
//             loop {
//                 let mut clients_c_c_r = clients_cloned_2.write().await;
//                 let new_receivers = clients_c_c_r
//                     .iter_mut()
//                     .filter_map(extract_receiver);

//                 receivers.push(new_receivers);

//                 let receivers_cloned = receivers
//                     .iter_mut()
//                     .map(|receiver| receiver);

//                 let merged = futures::stream::select_all(&mut receivers_cloned.flatten()).fuse();

//                 let _ = tx
//                     .send(merged)
//                     .await
//                     .map_err(|err| println!("Error sending merged stream: {:?}", err.to_string()));

//                 // sleep for 1 seconds
//                 sleep(Duration::from_secs(1)).await;
//             }
//         });

//         let mut task: Option<tokio::task::JoinHandle<()>> = None;
//         while let Some(mut merged) = rx.recv().await {
//             println!("Received a merged stream");
//             let event_queue_cloned = event_queue.clone();
//             if let Some(task) = task.take() {
//                 task.abort();
//             }
//             // Spawn a task to produce events
//             task = Some(tokio::spawn(async move {
//                 while let Some(msg) = merged.next().await {
//                     let msg = if let Ok(msg) = msg {
//                         msg
//                     } else {
//                         continue;
//                     };

//                     println!("Received a message: {:?}", &msg);

//                     let my_message = if let Ok(my_message) = MyMessage::try_from(msg.clone()) {
//                         my_message
//                     } else {
//                         // invalid message
//                         continue;
//                     };
//                     let event = Event {
//                         target_user_id: my_message.recipient.clone().into(),
//                         content: EventContent::Message(my_message),
//                     };
//                     event_queue_cloned.write().await.push_back(event);
//                 }
//             }));
//         }
//     });
// }
