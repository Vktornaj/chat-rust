use serde::Serialize;
use utoipa::ToSchema;
use axum::http::StatusCode;


#[derive(ToSchema)]
pub struct Response<T>(JsonResponse<T>, StatusCode);

#[derive(Serialize, ToSchema, Default)]
pub struct JsonResponse<T> {
    pub message: Option<String>,
    pub data: Option<T>,
    pub error: Option<String>,
    pub details: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_response() {
        let json_response = JsonResponse {
            message: Some("message".to_string()),
            data: Some("data".to_string()),
            ..Default::default()
        };
        let expected = json!({
            "message": Some("message"),
            "data": Some("data"),
        });
        assert_eq!(serde_json::to_value(json_response).unwrap(), expected);
    }
}
