use axum::{http::StatusCode, response::{IntoResponse, Json}};
use serde::Serialize;

#[derive(Serialize)]
pub struct ApiErrorBody {
    pub status: String,
    pub error: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
}

pub enum AppError {
    BadRequest(String),
    InvalidDate(String, String),
    Calculation(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let (status, error, details) = match self {
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg, None),
            AppError::InvalidDate(msg, det) => (StatusCode::BAD_REQUEST, msg, Some(det)),
            AppError::Calculation(msg) => (StatusCode::BAD_REQUEST, msg, None),
        };
        (status, Json(ApiErrorBody {
            status: "error".into(),
            error,
            details,
        })).into_response()
    }
}