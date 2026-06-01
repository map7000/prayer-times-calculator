use axum::response::IntoResponse;
use serde_json::json;

pub async fn health_check() -> impl IntoResponse {
    json!({
        "status": "healthy",
        "service": "prayer-times-api",
        "version": "0.1.0"
    })
}