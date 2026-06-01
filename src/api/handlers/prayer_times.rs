use axum::{extract::{Query, State, Json}, response::IntoResponse};
use prayer_times_calculator::CalculationRequest;
use crate::api::models::request::{PrayerQueryParams, RangeRequest};
use crate::api::models::response::{PrayerTimesResponse, RequestMeta, RangeResponse, RangeItem};
use crate::api::error::AppError;
use crate::services::prayer_service;
use crate::AppState;
use chrono::Datelike;

pub async fn post_prayer_times(
    State(_): State<AppState>,
    Json(req): Json<CalculationRequest>,
) -> Result<impl IntoResponse, AppError> {
    let times = prayer_service::calculate_from_request(&req)?;
    Ok(Json(PrayerTimesResponse {
        status: "success".into(),
        data: times,
        request: RequestMeta {
            date: format!("{}-{:02}-{:02}", req.year, req.month, req.day),
            coordinates: (req.lat, req.lng),
            method: None,
            timezone: None,
        },
    }))
}

pub async fn get_prayer_times(
    State(_): State<AppState>,
    Query(params): Query<PrayerQueryParams>,
) -> Result<impl IntoResponse, AppError> {
    let (times, method, coords, tz) = prayer_service::calculate_from_query(params)?;
    Ok(Json(PrayerTimesResponse {
        status: "success".into(),
        data: times,
        request: RequestMeta {
            date: format!("{}-{:02}-{:02}", 
                chrono::Utc::now().year(), chrono::Utc::now().month(), chrono::Utc::now().day()),
            coordinates: coords,
            method: Some(method),
            timezone: Some(tz),
        },
    }))
}

pub async fn post_prayer_times_range(
    State(_): State<AppState>,
    Json(req): Json<RangeRequest>,
) -> Result<impl IntoResponse, AppError> {
    let results = prayer_service::calculate_range(req)?;
    let data: Vec<RangeItem> = results.into_iter().map(|(date, times)| RangeItem {
        date: date.to_string(),
        times: times.into(),
    }).collect();

    Ok(Json(RangeResponse {
        status: "success".into(),
        count: data.len(),
        data,
    }))
}