use axum::{response::IntoResponse, Json};
use serde_json::json;

pub async fn health_check() -> impl IntoResponse {
    Json(json!({
        "status": "healthy",
        "service": "prayer-times-api",
        "version": env!("CARGO_PKG_VERSION")
    }))
}
