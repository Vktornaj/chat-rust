use tokio::{runtime::Builder, net::TcpListener};
use tracing_subscriber;


fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    Builder::new_multi_thread()
        .worker_threads(4)
        .thread_name("worker")
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            // build our application with a single route
            let app = entry::router().await; 

            let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
            // run it with hyper on localhost:3000
            axum::serve(listener, app.into_make_service())
                .await
                .unwrap();
        });
}