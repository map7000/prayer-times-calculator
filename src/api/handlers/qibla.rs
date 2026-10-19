use axum::{
    extract::{Query, State},
    Json,
};
use chrono::Local;

use crate::api::error::AppError;
use crate::api::models::request::QiblaQueryParams;
use crate::api::models::response::{QiblaData, QiblaResponse, RequestMeta};
use crate::services::qibla_service;
use crate::AppState;

pub async fn get_qibla(
    State(_state): State<AppState>,
    Query(params): Query<QiblaQueryParams>,
) -> Result<Json<QiblaResponse>, AppError> {
    let requested_date = params.date.unwrap_or_else(|| Local::now().date_naive());

    let computed = qibla_service::calculate_from_query(params, requested_date)?;

    Ok(Json(QiblaResponse {
        status: "success".into(),
        data: QiblaData {
            true_bearing_degrees: computed.qibla.bearing_degrees,
            magnetic_bearing_degrees: computed.magnetic_bearing_degrees,
            magnetic_declination_degrees: computed.declination_degrees,
            true_cardinal_direction: computed.qibla.cardinal_direction,
            distance_km: computed.qibla.distance_km,
        },
        request: RequestMeta {
            date: computed.date_used.to_string(),
            coordinates: (computed.qibla.latitude, computed.qibla.longitude),
            method: None,
            timezone: None,
        },
    }))
}
