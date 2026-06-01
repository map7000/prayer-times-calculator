use axum::{
    extract::{Query, State},
    Json,
};
use chrono::NaiveDate;

use crate::api::error::AppError;
use crate::api::models::request::RangeRequest;
use crate::api::models::response::{RangeItem, RangeResponse, RequestMeta};
use crate::services::prayer_service;
use crate::AppState;
use crate::api::models::request::PrayerQueryParams;

pub async fn get_prayer_times_range(
    State(_state): State<AppState>,
    Query(params): Query<RangeRequest>,
) -> Result<Json<RangeResponse>, AppError> {
    if params.end_date < params.start_date {
        return Err(AppError::InvalidDate(
            "Invalid date range".into(),
            "end_date must be after or equal to start_date".into(),
        ));
    }

    let mut data = Vec::new();
    let mut current = params.start_date;

    while current <= params.end_date {
        let day_params = PrayerQueryParams {
            lat: params.lat,
            lng: params.lng,
            date: Some(current),
            method: params.method.clone(),
            timezone: params.timezone,
        };

        let (times, _, _, _) =
            prayer_service::calculate_from_query(day_params, current)?;

        data.push(RangeItem {
            date: current.to_string(),
            fajr: crate::formatter::format_time(times.fajr, true),
            sunrise: crate::formatter::format_time(times.sunrise, true),
            dhuhr: crate::formatter::format_time(times.dhuhr, true),
            asr: crate::formatter::format_time(times.asr, true),
            maghrib: crate::formatter::format_time(times.maghrib, true),
            isha: crate::formatter::format_time(times.isha, true),
        });

        current = current.succ_opt().unwrap_or(current);
    }

    Ok(Json(RangeResponse {
        status: "success".into(),
        data,
        request: RequestMeta {
            date: format!("{} to {}", params.start_date, params.end_date),
            coordinates: (params.lat, params.lng),
            method: params.method,
            timezone: params.timezone,
        },
    }))
}