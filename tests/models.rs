//! Tests for the data models: enums, serde round-trips, defaults and errors.

use prayer_times_calculator::methods;
use prayer_times_calculator::models::*;

#[test]
fn angle_or_minutes_reports_value_and_kind() {
    let angle = AngleOrMinutes::Angle(18.0);
    assert_eq!(angle.value(), 18.0);
    assert!(!angle.is_minutes());

    let minutes = AngleOrMinutes::Minutes(90.0);
    assert_eq!(minutes.value(), 90.0);
    assert!(minutes.is_minutes());
}

#[test]
fn settings_defaults_match_praytimes_defaults() {
    let s = Settings::default();
    assert_eq!(s.imsak, AngleOrMinutes::Minutes(10.0));
    assert_eq!(s.dhuhr, AngleOrMinutes::Minutes(0.0));
    assert_eq!(s.asr, AsrJuristic::Standard);
    assert_eq!(s.high_lats, HighLatMethod::NightMiddle);
}

#[test]
fn offsets_default_to_zero() {
    let o = TimeOffsets::default();
    for v in [
        o.imsak, o.fajr, o.sunrise, o.dhuhr, o.asr, o.sunset, o.maghrib, o.isha, o.midnight,
    ] {
        assert_eq!(v, 0.0);
    }
}

#[test]
fn params_and_settings_serde_roundtrip() {
    let params = methods::makkah();
    let json = serde_json::to_string(&params).unwrap();
    let back: Params = serde_json::from_str(&json).unwrap();
    assert_eq!(params, back);

    let settings = Settings {
        asr: AsrJuristic::Custom(1.75),
        ..Settings::default()
    };
    let json = serde_json::to_string(&settings).unwrap();
    let back: Settings = serde_json::from_str(&json).unwrap();
    assert_eq!(settings, back);
}

#[test]
fn calculation_request_is_deserializable_with_defaults() {
    // `settings` and `offsets` are optional and fall back to defaults
    let json = r#"{
        "year": 2026, "month": 6, "day": 14,
        "lat": 21.4225, "lng": 39.8262, "elv": 0.0, "timezone": 3.0,
        "params": {
            "fajr": { "Angle": 18.5 },
            "isha": { "Minutes": 90.0 },
            "maghrib": { "Minutes": 0.0 },
            "midnight": "Standard"
        }
    }"#;
    let req: CalculationRequest = serde_json::from_str(json).unwrap();
    assert_eq!(req.lat, 21.4225);
    assert_eq!(req.settings, Settings::default());
    assert_eq!(req.offsets, TimeOffsets::default());
    assert!(req.params.isha.is_minutes());
}

#[test]
fn times_serde_roundtrip() {
    let times = Times {
        imsak: 4.5,
        fajr: 5.0,
        sunrise: 6.0,
        dhuhr: 12.0,
        asr: 15.0,
        sunset: 18.0,
        maghrib: 18.5,
        isha: 20.0,
        midnight: 24.0,
    };
    let json = serde_json::to_string(&times).unwrap();
    let back: Times = serde_json::from_str(&json).unwrap();
    assert_eq!(times, back);
}

#[test]
fn calc_error_display_messages() {
    let e = CalcError::InvalidDate {
        year: 2026,
        month: 13,
        day: 0,
    };
    assert_eq!(e.to_string(), "Invalid date: 2026-13-0");
    assert_eq!(
        CalcError::InvalidLatitude.to_string(),
        "Latitude must be between -90 and 90"
    );
    assert_eq!(
        CalcError::InvalidLongitude.to_string(),
        "Longitude must be between -180 and 180"
    );
}
