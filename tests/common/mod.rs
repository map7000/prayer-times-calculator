//! Shared helpers for the integration test binaries.

#![allow(dead_code)]

use prayer_times_calculator::models::{CalculationRequest, Params, Settings, TimeOffsets};

/// Assert that two times (hours) are within `tol_minutes` of each other.
#[track_caller]
pub fn assert_minutes_close(actual: f64, expected: f64, tol_minutes: f64) {
    let diff = (actual - expected).abs() * 60.0;
    assert!(
        diff <= tol_minutes,
        "times differ by {diff:.3} min (tolerance {tol_minutes}): actual {actual}, expected {expected}"
    );
}

/// Build a calculation request with default settings/offsets.
#[allow(clippy::too_many_arguments)]
pub fn request(
    year: i32,
    month: u32,
    day: u32,
    lat: f64,
    lng: f64,
    elv: f64,
    timezone: f64,
    params: Params,
) -> CalculationRequest {
    request_full(
        year,
        month,
        day,
        lat,
        lng,
        elv,
        timezone,
        params,
        Settings::default(),
        TimeOffsets::default(),
    )
}

/// Build a calculation request with explicit settings.
#[allow(clippy::too_many_arguments)]
pub fn request_with_settings(
    year: i32,
    month: u32,
    day: u32,
    lat: f64,
    lng: f64,
    elv: f64,
    timezone: f64,
    params: Params,
    settings: Settings,
) -> CalculationRequest {
    request_full(
        year,
        month,
        day,
        lat,
        lng,
        elv,
        timezone,
        params,
        settings,
        TimeOffsets::default(),
    )
}

/// Build a calculation request with explicit settings and offsets.
#[allow(clippy::too_many_arguments)]
pub fn request_full(
    year: i32,
    month: u32,
    day: u32,
    lat: f64,
    lng: f64,
    elv: f64,
    timezone: f64,
    params: Params,
    settings: Settings,
    offsets: TimeOffsets,
) -> CalculationRequest {
    CalculationRequest {
        year,
        month,
        day,
        lat,
        lng,
        elv,
        timezone,
        params,
        settings,
        offsets,
    }
}
