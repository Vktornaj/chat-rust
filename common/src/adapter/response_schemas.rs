use serde::Serialize;
use axum::{
    http::{StatusCode, header, HeaderValue},
    response::{IntoResponse, Response}, body::Body,
};


#[derive(Serialize)]
pub struct JsonError {
    pub message: String,
    pub details: String,
    pub code: u32,
}

#[derive(Serialize)]
pub struct JsonResponse<T> {
    #[serde(with = "status_code")]
    pub status: StatusCode,
    pub data: Option<T>,
    pub error: Option<JsonError>,
}

impl<T> JsonResponse<T> {

    pub fn new_ok(data: T) -> Self {
        JsonResponse {
            status: StatusCode::OK,
            data: Some(data),
            error: None,
        }
    }

    pub fn new_int_ser_err(code: u32, message: &str, details: String) -> Self {
        JsonResponse {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            data: None,
            error: Some(JsonError {
                code,
                message: message.to_string(),
                details,
            }),
        }
    }
}

mod status_code {
    use serde::{self, Serializer};
    use axum::http::StatusCode;

    pub fn serialize<S>(status: &StatusCode, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u16(status.as_u16())
    }
}

impl<T> IntoResponse for JsonResponse<T>
where
    T: Serialize,
{
    fn into_response(self) -> Response {
        Response::builder()
            .status(self.status)
            .header(header::CONTENT_TYPE, HeaderValue::from_static("application/json"))
            .body(Body::from(serde_json::to_vec(&self).unwrap()))
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    // use super::*;
    // use serde_json::json;

    #[test]
    fn test_response() {
        // let json_response = JsonResponse {
        //     data: Some("data".to_string()),
        //     error: None,
        //     status: StatusCode::OK,
        // };
        // let expected = json!({
        //     "status": 200,
        //     "data": Some("data"),
        //     "error": {
        //         "message": None::<String>,
        //         "details": None::<String>,
        //     },
        // });
        // assert_eq!(serde_json::to_value(json_response).unwrap(), expected);
    }
}
