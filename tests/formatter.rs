//! Tests for `formatter::format_time`.

use prayer_times_calculator::formatter::format_time;

#[test]
fn formats_24h_with_minute_padding() {
    assert_eq!(format_time(12.75, true), "12:45");
    assert_eq!(format_time(0.25, true), "00:15");
    assert_eq!(format_time(9.0, true), "09:00");
    assert_eq!(format_time(23.983_333, true), "23:59");
}

#[test]
fn formats_24h_wraps_days() {
    // 0:30 on the following day
    assert_eq!(format_time(24.5, true), "00:30");
    assert_eq!(format_time(25.0, true), "01:00");
}

#[test]
fn formats_12h_with_am_pm() {
    assert_eq!(format_time(0.0, false), "12:00 AM");
    assert_eq!(format_time(1.0, false), "01:00 AM");
    assert_eq!(format_time(11.75, false), "11:45 AM");
    assert_eq!(format_time(12.0, false), "12:00 PM");
    assert_eq!(format_time(13.5, false), "01:30 PM");
    assert_eq!(format_time(23.0, false), "11:00 PM");
}

#[test]
fn formats_12h_wraps_days() {
    assert_eq!(format_time(24.5, false), "12:30 AM");
    assert_eq!(format_time(25.0, false), "01:00 AM");
}

#[test]
fn rounds_half_minute_up_within_the_hour() {
    // 10:58:52 rounds to 10:59, 10:58:12 stays 10:58
    assert_eq!(format_time(10.0 + 58.8 / 60.0, true), "10:59");
    assert_eq!(format_time(10.0 + 58.2 / 60.0, true), "10:58");
}

#[test]
fn rounds_into_the_next_hour() {
    // 10:59:42 rounds to 11:00; 10:59:12 stays 10:59
    assert_eq!(format_time(10.0 + 59.7 / 60.0, true), "11:00");
    assert_eq!(format_time(10.0 + 59.2 / 60.0, true), "10:59");
}

#[test]
fn near_midnight_rounding_wraps_to_00_00() {
    assert_eq!(format_time(23.999, true), "00:00");
    assert_eq!(format_time(23.999, false), "12:00 AM");
}

#[test]
fn negative_times_are_wrapped() {
    // -0.5 h (previous day 23:30) wraps to 23:30
    assert_eq!(format_time(-0.5, true), "23:30");
    assert_eq!(format_time(-0.5, false), "11:30 PM");
}

#[test]
fn nan_becomes_invalid_time_marker() {
    assert_eq!(format_time(f64::NAN, true), "-----");
    assert_eq!(format_time(f64::NAN, false), "-----");
}
