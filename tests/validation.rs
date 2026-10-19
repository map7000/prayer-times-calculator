//! Validation and error handling tests for `PrayerCalculator::calculate`.

mod common;

use common::request;
use prayer_times_calculator::{calculator::PrayerCalculator, methods, models::CalcError};

#[test]
fn rejects_month_zero() {
    let err =
        PrayerCalculator::calculate(&request(2026, 0, 15, 21.4, 39.8, 0.0, 3.0, methods::mwl()))
            .unwrap_err();
    assert!(matches!(
        err,
        CalcError::InvalidDate {
            month: 0,
            day: 15,
            ..
        }
    ));
}

#[test]
fn rejects_month_thirteen() {
    let err =
        PrayerCalculator::calculate(&request(2026, 13, 15, 21.4, 39.8, 0.0, 3.0, methods::mwl()))
            .unwrap_err();
    assert!(matches!(err, CalcError::InvalidDate { month: 13, .. }));
}

#[test]
fn rejects_day_zero() {
    let err =
        PrayerCalculator::calculate(&request(2026, 6, 0, 21.4, 39.8, 0.0, 3.0, methods::mwl()))
            .unwrap_err();
    assert!(matches!(err, CalcError::InvalidDate { day: 0, .. }));
}

#[test]
fn rejects_day_thirty_two() {
    let err =
        PrayerCalculator::calculate(&request(2026, 6, 32, 21.4, 39.8, 0.0, 3.0, methods::mwl()))
            .unwrap_err();
    assert!(matches!(err, CalcError::InvalidDate { day: 32, .. }));
}

#[test]
fn rejects_upper_and_lower_bound_latitude() {
    let err =
        PrayerCalculator::calculate(&request(2026, 6, 15, 90.01, 0.0, 0.0, 0.0, methods::mwl()))
            .unwrap_err();
    assert!(matches!(err, CalcError::InvalidLatitude));

    let err =
        PrayerCalculator::calculate(&request(2026, 6, 15, -90.01, 0.0, 0.0, 0.0, methods::mwl()))
            .unwrap_err();
    assert!(matches!(err, CalcError::InvalidLatitude));
}

#[test]
fn rejects_upper_and_lower_bound_longitude() {
    let err =
        PrayerCalculator::calculate(&request(2026, 6, 15, 0.0, 180.01, 0.0, 0.0, methods::mwl()))
            .unwrap_err();
    assert!(matches!(err, CalcError::InvalidLongitude));

    let err = PrayerCalculator::calculate(&request(
        2026,
        6,
        15,
        0.0,
        -180.01,
        0.0,
        0.0,
        methods::mwl(),
    ))
    .unwrap_err();
    assert!(matches!(err, CalcError::InvalidLongitude));
}

#[test]
fn boundary_values_of_month_day_are_accepted() {
    // Only the numeric ranges are validated (12/31 etc. — as in PrayTimes;
    // no calendar-aware clamping of e.g. Feb 30).
    let req = request(2026, 1, 1, 21.4, 39.8, 0.0, 3.0, methods::makkah());
    assert!(PrayerCalculator::calculate(&req).is_ok());
    let req = request(2026, 12, 31, 21.4, 39.8, 0.0, 3.0, methods::makkah());
    assert!(PrayerCalculator::calculate(&req).is_ok());
}

#[test]
fn exact_polar_boundary_coordinates_are_accepted() {
    // ±90 lat / ±180 lng pass the range check; results may be degenerate,
    // but must not panic.
    let req = request(2026, 3, 20, 90.0, 0.0, 0.0, 0.0, methods::mwl());
    assert!(PrayerCalculator::calculate(&req).is_ok());
    let req = request(2026, 3, 20, 0.0, 180.0, 0.0, 0.0, methods::mwl());
    assert!(PrayerCalculator::calculate(&req).is_ok());
    let req = request(2026, 3, 20, 0.0, -180.0, 0.0, 0.0, methods::mwl());
    assert!(PrayerCalculator::calculate(&req).is_ok());
}

#[test]
fn negative_era_years_are_accepted() {
    let req = request(-100, 6, 21, 21.4, 39.8, 0.0, 3.0, methods::mwl());
    let t = PrayerCalculator::calculate(&req).unwrap();
    for v in [t.fajr, t.sunrise, t.dhuhr, t.asr, t.sunset] {
        assert!(v.is_finite());
    }
}

#[test]
fn invalid_date_error_message_contains_the_input() {
    let err =
        PrayerCalculator::calculate(&request(2026, 0, 15, 21.4, 39.8, 0.0, 3.0, methods::mwl()))
            .unwrap_err();
    assert_eq!(err.to_string(), "Invalid date: 2026-0-15");
}

#[test]
fn date_validation_happens_before_coordinate_validation() {
    // Both invalid: the date error should win
    let err = PrayerCalculator::calculate(&request(
        2026,
        13,
        15,
        99.0,
        200.0,
        0.0,
        0.0,
        methods::mwl(),
    ))
    .unwrap_err();
    assert!(matches!(err, CalcError::InvalidDate { .. }));
}
