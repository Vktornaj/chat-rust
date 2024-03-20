use axum::body::{to_bytes, Bytes, Body};
use axum::http::{Request, Response};
use axum::middleware::Next;
use axum::Error;
// TODO: remove hyper from cargo.toml
// use hyper::body::Body;
use log::info;
use colored::*;


pub async fn log_request_response(
    req: Request<Body>,
    next: Next,
) -> Response<Body> {
    let path = &req.uri().path().to_string();

    let (req_parts, req_body) = req.into_parts();

    // Print request
    let bytes = buffer_and_print("request", path, req_body).await.unwrap();
    let req: Request<Body> = Request::from_parts(req_parts, Body::from(bytes));

    let res = next.run(req).await;
    

    let (mut res_parts, res_body) = res.into_parts();

    // Print response
    let bytes = buffer_and_print("response", path, res_body).await.unwrap();
    
    // When your encoding is chunked there can be problems without removing the header
    res_parts.headers.remove("transfer-encoding");
    
    let res = Response::from_parts(res_parts, Body::from(bytes));
     
    res
}

// Consumes body and prints
async fn buffer_and_print(direction: &str, path: &str, body: Body) -> Result<Bytes, Error> {
    let bytes = match to_bytes(body, usize::MAX).await {
        Ok(bytes) => bytes,
        Err(err) => return Err(err),
    };

    if let Ok(str_body) = std::str::from_utf8(&bytes) {
        if !str_body.is_empty() {
            let direction = match direction {
                "request" => "request".green(),
                "response" => "response".red(),
                _ => "unknown".white(),
            };
            let path = path.blue();
            let str_body = str_body.yellow();
            if str_body.len() > 2000 {
                info!("{}", format!("{} -> path: {} body: {}...", direction, path, &str_body[0..2000]));
            }
            else {
                info!("{}", format!("{} -> path: {} body: {}", direction, path, str_body));
            }
        }
    }

    Ok(bytes)
}
