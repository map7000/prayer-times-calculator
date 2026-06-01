use axum::{
    extract::{Query, State},
    Json,
};
use chrono::Local;

use crate::api::error::AppError;
use crate::api::models::request::PrayerQueryParams;
use crate::api::models::response::{PrayerTimesResponse, RequestMeta};
use crate::services::prayer_service;
use crate::AppState;

pub async fn get_prayer_times(
    State(_state): State<AppState>,
    Query(params): Query<PrayerQueryParams>,
) -> Result<Json<PrayerTimesResponse>, AppError> {
    let requested_date = params.date.unwrap_or_else(|| Local::now().date_naive());

    let (times, method, coords, tz) =
        prayer_service::calculate_from_query(params, requested_date)?;

    Ok(Json(PrayerTimesResponse {
        status: "success".into(),
        data: times.into(),
        request: RequestMeta {
            date: requested_date.to_string(),
            coordinates: coords,
            method: Some(method),
            timezone: Some(tz),
        },
    }))
}