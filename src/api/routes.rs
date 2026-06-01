use axum::{routing::{get, post}, Router};
use crate::api::handlers::{health, methods, prayer_times};
use crate::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/health", get(health::health_check))
        .route("/api/methods", get(methods::get_methods))
        .route("/api/prayer-times", post(prayer_times::post_prayer_times))
        .route("/api/prayer-times", get(prayer_times::get_prayer_times))
        .route("/api/prayer-times/range", post(prayer_times::post_prayer_times_range))
}