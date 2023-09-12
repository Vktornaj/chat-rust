use rocket::{Request, Data};
use rocket::fairing::{Fairing, Info, Kind};
use std::time::Instant;
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
pub struct PrometheusMetrics;

#[rocket::async_trait]
impl Fairing for PrometheusMetrics {
    fn info(&self) -> Info {
        Info {
            name: "Prometheus Metrics",
            kind: Kind::Request | Kind::Response,
        }
    }

    async fn on_request(&self, request: &mut Request<'_>, _: &mut Data<'_>) {
        request.local_cache(|| Instant::now());
        INCOMING_REQUESTS.inc();
    }

    async fn on_response<'r>(&self, request: &'r Request<'_>, response: &mut rocket::Response<'r>) {
        let start = request.local_cache(|| Instant::now());
        let elapsed = start.elapsed();
        let code = response.status().code;
        let is_ok = if code >= 200 && code < 300 { "true" } else { "false" };
        RESPONSE_TIME_COLLECTOR.with_label_values(&[is_ok]).observe(elapsed.as_secs_f64());
    }
}
