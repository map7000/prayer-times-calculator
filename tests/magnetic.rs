//! Magnetic declination tests.
//!
//! External anchors come from the `world_magnetic_model` crate's documented
//! NOAA examples (the crate itself validates against the official NCEI
//! WMM2020/WMM2025 test values upstream). Geophysical sanity bounds and
//! error mappings are validated directly.

use chrono::NaiveDate;
use prayer_times_calculator::{magnetic, magnetic::MagneticError, qibla};

fn date(y: i32, m: u32, d: u32) -> NaiveDate {
    NaiveDate::from_ymd_opt(y, m, d).unwrap()
}

#[test]
fn noaa_documented_example_declination() {
    // docs.rs example: 37.03°N, -7.91°E, 100 m, 15th day of 2029 => D = -0.17°
    let d = magnetic::declination_degrees(37.03, -7.91, 100.0, date(2029, 1, 15)).unwrap();
    assert!(
        (d - (-0.17)).abs() < 0.01,
        "declination {d} should be -0.17 ± 0.01"
    );
}

#[test]
fn noaa_documented_example_high_latitude() {
    // docs.rs example: 80°N, 2.36 rad, 3000 ft (914.4 m), 15th day of 2029
    let d = magnetic::declination_degrees(80.0, 2.36_f64.to_degrees(), 914.4, date(2029, 1, 15))
        .unwrap();
    assert!(
        (d - (-26.41)).abs() < 0.02,
        "declination {d} should be -26.41 ± 0.02"
    );
}

#[test]
fn declination_is_easterly_in_western_europe() {
    // Istanbul (2026-10-19): easterly declination, ~+5..6 degrees under WMM2025
    let d = magnetic::declination_degrees(41.0082, 28.9784, 0.0, date(2026, 10, 19)).unwrap();
    assert!(
        (2.0..9.0).contains(&d),
        "Istanbul declination {d} out of the expected range (2..9)"
    );
}

#[test]
fn declination_is_westerly_on_the_us_east_coast() {
    // New York (2026-10-19): westerly declination, ~-12..-13 degrees
    let d = magnetic::declination_degrees(40.7128, -74.0060, 0.0, date(2026, 10, 19)).unwrap();
    assert!(
        (-17.0..-8.0).contains(&d),
        "New York declination {d} out of the expected range (-17..-8)"
    );
}

#[test]
fn declination_is_easterly_on_the_us_west_coast() {
    // Los Angeles (2026-10-19): easterly declination, ~+11..12 degrees
    let d = magnetic::declination_degrees(34.0522, -118.2437, 0.0, date(2026, 10, 19)).unwrap();
    assert!(
        (7.0..16.0).contains(&d),
        "Los Angeles declination {d} out of the expected range (7..16)"
    );
}

#[test]
fn declination_is_continuous_across_the_epoch_switch() {
    // WMM2020 -> WMM2025 boundary: values one day apart must be sane
    let before = magnetic::declination_degrees(41.0082, 28.9784, 0.0, date(2024, 12, 31)).unwrap();
    let after = magnetic::declination_degrees(41.0082, 28.9784, 0.0, date(2025, 1, 1)).unwrap();
    // A small step at the boundary is physically expected: WMM2020's final
    // secular extrapolation and WMM2025's fresh epoch differ slightly.
    assert!(
        (after - before).abs() < 0.5,
        "declination jumps {before} -> {after} at the epoch switch"
    );
}

#[test]
fn both_bundled_epochs_evaluate() {
    // First and last valid dates of both bundled models
    assert!(magnetic::declination_degrees(0.0, 0.0, 0.0, date(2020, 1, 1)).is_ok());
    assert!(magnetic::declination_degrees(0.0, 0.0, 0.0, date(2024, 12, 31)).is_ok());
    assert!(magnetic::declination_degrees(0.0, 0.0, 0.0, date(2025, 1, 1)).is_ok());
    assert!(magnetic::declination_degrees(0.0, 0.0, 0.0, date(2029, 12, 31)).is_ok());
}

#[test]
fn leap_day_is_accepted() {
    assert!(magnetic::declination_degrees(21.4, 39.8, 0.0, date(2028, 2, 29)).is_ok());
}

#[test]
fn dates_outside_validity_range_error() {
    let err = magnetic::declination_degrees(0.0, 0.0, 0.0, date(2019, 12, 31)).unwrap_err();
    assert!(
        matches!(err, MagneticError::DateOutsideOfValidityRange),
        "{err}"
    );
    let err = magnetic::declination_degrees(0.0, 0.0, 0.0, date(2030, 1, 1)).unwrap_err();
    assert!(
        matches!(err, MagneticError::DateOutsideOfValidityRange),
        "{err}"
    );
}

#[test]
fn altitude_outside_validity_range_errors() {
    let err = magnetic::declination_degrees(0.0, 0.0, -1500.0, date(2026, 10, 19)).unwrap_err();
    assert!(
        matches!(err, MagneticError::HeightOutsideOfValidityRange),
        "{err}"
    );
    let err = magnetic::declination_degrees(0.0, 0.0, 850_001.0, date(2026, 10, 19)).unwrap_err();
    assert!(
        matches!(err, MagneticError::HeightOutsideOfValidityRange),
        "{err}"
    );
}

#[test]
fn altitude_boundaries_are_accepted() {
    assert!(magnetic::declination_degrees(21.4, 39.8, -1000.0, date(2026, 10, 19)).is_ok());
    assert!(magnetic::declination_degrees(21.4, 39.8, 850_000.0, date(2026, 10, 19)).is_ok());
}

#[test]
fn easterly_declination_reduces_the_compass_qibla() {
    // Istanbul: true qibla is SSE; with easterly declination the compass
    // must read a smaller bearing.
    let date = date(2026, 10, 19);
    let bearing = qibla::bearing_degrees(41.0082, 28.9784);
    let declination = magnetic::declination_degrees(41.0082, 28.9784, 0.0, date).unwrap();
    let magnetic_bearing = magnetic::to_magnetic_bearing(bearing, declination);

    assert!(declination > 0.0);
    assert!((magnetic_bearing - (bearing - declination).rem_euclid(360.0)).abs() < 1e-9);
    assert!(magnetic_bearing < bearing);
}

#[test]
fn to_magnetic_bearing_wraps_under_zero() {
    // true bearing 5°, easterly declination +10° => wraps to 355°
    let m = magnetic::to_magnetic_bearing(5.0, 10.0);
    assert!((m - 355.0).abs() < 1e-12);
}

#[test]
fn to_magnetic_bearing_stays_plain_when_in_range() {
    let m = magnetic::to_magnetic_bearing(350.0, 20.0);
    assert!((m - 330.0).abs() < 1e-12);
}

#[test]
fn to_magnetic_bearing_wraps_over_360_for_westerly_declination() {
    // Westerly (negative) declination pushes the compass reading upward
    let m = magnetic::to_magnetic_bearing(350.0, -20.0);
    assert!((m - 10.0).abs() < 1e-12);
    let m = magnetic::to_magnetic_bearing(5.0, -20.0);
    assert!((m - 25.0).abs() < 1e-12);
}

#[test]
fn declination_is_deterministic() {
    let date = date(2026, 10, 19);
    let d1 = magnetic::declination_degrees(51.5074, -0.1278, 0.0, date).unwrap();
    let d2 = magnetic::declination_degrees(51.5074, -0.1278, 0.0, date).unwrap();
    assert_eq!(d1, d2);
}
