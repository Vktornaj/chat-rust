use std::time::Instant;
use axum::{
    http::Request,
    response::Response,
    middleware::Next, body::Body,
};

use prometheus::{
    HistogramVec, 
    IntCounter, 
    // IntCounterVec, 
    // IntGauge,
    // Opts, 
    register_int_counter, 
    // register_int_gauge, 
    // register_int_counter_vec, 
    register_histogram_vec, 
    Gauge, 
    register_gauge,
};
use lazy_static::lazy_static;


lazy_static! {
    pub static ref INCOMING_REQUESTS: IntCounter = register_int_counter!(
        "http_requests_total", 
        "Total number of HTTP requests"
    ).unwrap();

    // pub static ref CONNECTED_CLIENTS: IntGauge = register_int_gauge!(
    //     "connected_clients", 
    //     "Connected Clients"
    // ).unwrap();

    // pub static ref RESPONSE_CODE_COLLECTOR: IntCounterVec = register_int_counter_vec!(
    //     Opts::new("response_code", "Response Codes"),
    //     &["env", "statuscode", "type"]
    // ).unwrap();

    pub static ref RESPONSE_TIME_COLLECTOR: HistogramVec = register_histogram_vec!(
        "response_time", 
        "Response Time", 
        &["is_ok"]
    ).unwrap();

    pub static ref CPU_USAGE: Gauge = register_gauge!(
        "cpu_usage", 
        "Current CPU usage in percent"
    ).unwrap();

    pub static ref MEM_USAGE: Gauge = register_gauge!(
        "mem_usage", 
        "Current memory usage in percent"
    ).unwrap();
}

pub async fn metrics_middleware(
    req: Request<Body>,
    next: Next,
) -> Response {
    // do something with `request`...
    let start = Instant::now();
    INCOMING_REQUESTS.inc();

    let response = next.run(req).await;

    // do something with `response`...
    let elapsed = start.elapsed();
    let status_result = if response.status()
        .is_success() { "ok" } else { "err" };
    RESPONSE_TIME_COLLECTOR.with_label_values(&[status_result])
        .observe(elapsed.as_secs_f64());

    response
}