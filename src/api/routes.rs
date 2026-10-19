use crate::api::handlers::{health, methods, prayer_times, qibla, range};
use crate::AppState;
use axum::{routing::get, Router};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/health", get(health::health_check))
        .route("/api/methods", get(methods::get_methods))
        .route("/api/prayer-times", get(prayer_times::get_prayer_times))
        .route(
            "/api/prayer-times/range",
            axum::routing::get(range::get_prayer_times_range),
        )
        .route("/api/qibla", get(qibla::get_qibla))
}
