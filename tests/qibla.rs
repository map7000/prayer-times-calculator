//! Qibla direction tests.
//!
//! Expected bearings and distances were computed independently with the
//! standard spherical great-circle formulas (initial-bearing formula and
//! Haversine distance) for the Kaaba location
//! (21.422487°, 39.826206°), Earth radius 6371 km.

use prayer_times_calculator::{
    models::CalcError,
    qibla::{self, Cardinal},
};

const CITY_TOL_DEG: f64 = 0.02; // bearing tolerance in degrees
const CITY_TOL_KM: f64 = 0.02; // distance tolerance in km

fn close_bearing(actual: f64, expected: f64) {
    let diff = (actual - expected).abs();
    assert!(
        diff <= CITY_TOL_DEG,
        "bearing differs by {diff} degrees: actual {actual}, expected {expected}"
    );
}

fn close_distance(actual: f64, expected: f64) {
    let diff = (actual - expected).abs();
    assert!(
        diff <= CITY_TOL_KM,
        "distance differs by {diff} km: actual {actual}, expected {expected}"
    );
}

#[test]
fn istanbul_reference_values() {
    let q = qibla::calculate(41.0082, 28.9784).unwrap();
    close_bearing(q.bearing_degrees, 151.6206); // SSE
    close_distance(q.distance_km, 2405.0682);
    assert_eq!(q.cardinal_direction, Cardinal::SouthSouthEast);
}

#[test]
fn london_reference_values() {
    let q = qibla::calculate(51.5074, -0.1278).unwrap();
    close_bearing(q.bearing_degrees, 118.9872); // ESE
    close_distance(q.distance_km, 4793.7767);
    assert_eq!(q.cardinal_direction, Cardinal::EastSouthEast);
}

#[test]
fn new_york_reference_values() {
    let q = qibla::calculate(40.7128, -74.0060).unwrap();
    close_bearing(q.bearing_degrees, 58.4817); // ENE
    close_distance(q.distance_km, 10306.3079);
    assert_eq!(q.cardinal_direction, Cardinal::EastNorthEast);
}

#[test]
fn cairo_reference_values() {
    let q = qibla::calculate(30.0444, 31.2357).unwrap();
    close_bearing(q.bearing_degrees, 136.1373);
    close_distance(q.distance_km, 1287.2177);
    assert_eq!(q.cardinal_direction, Cardinal::SouthEast);
}

#[test]
fn jakarta_reference_values() {
    let q = qibla::calculate(-6.2088, 106.8456).unwrap();
    close_bearing(q.bearing_degrees, 295.1517); // WNW (west across the Indian Ocean)
    close_distance(q.distance_km, 7920.1259);
    assert_eq!(q.cardinal_direction, Cardinal::WestNorthWest);
}

#[test]
fn sydney_reference_values() {
    let q = qibla::calculate(-33.8688, 151.2093).unwrap();
    close_bearing(q.bearing_degrees, 277.4996);
    close_distance(q.distance_km, 13236.2543);
    assert_eq!(q.cardinal_direction, Cardinal::West);
}

#[test]
fn los_angeles_reference_values() {
    let q = qibla::calculate(34.0522, -118.2437).unwrap();
    close_bearing(q.bearing_degrees, 23.8571);
    close_distance(q.distance_km, 13424.3194);
    assert_eq!(q.cardinal_direction, Cardinal::NorthNorthEast);
}

#[test]
fn murmansk_reference_values() {
    // High-latitude sanity check
    let q = qibla::calculate(68.9585, 33.0827).unwrap();
    close_bearing(q.bearing_degrees, 171.5030);
    close_distance(q.distance_km, 5305.7050);
    assert_eq!(q.cardinal_direction, Cardinal::South);
}

#[test]
fn at_the_kaaba_bearing_is_zero_and_distance_zero() {
    let q = qibla::calculate(21.422487, 39.826206).unwrap();
    assert!(q.bearing_degrees.abs() < 1e-9);
    assert!(q.distance_km.abs() < 1e-9);
    assert_eq!(q.cardinal_direction, Cardinal::North);
}

#[test]
fn antipodal_point_is_half_the_circumference_away() {
    let q = qibla::calculate(-21.422487, 39.826206 - 180.0).unwrap();
    let half_circumference = std::f64::consts::PI * qibla::EARTH_RADIUS_KM;
    close_distance(q.distance_km, half_circumference);
    // The bearing is degenerate here: the formula feeds atan2 values around
    // ±1e-16, so only the range can be asserted.
    assert!((0.0..360.0).contains(&q.bearing_degrees));
}

#[test]
fn north_pole_reference_values() {
    // Arc from the North Pole to the Kaaba spans 90 - 21.42 = 68.58 degrees
    let q = qibla::calculate(90.0, 5.0).unwrap();
    close_distance(q.distance_km, 7625.4715);
    close_bearing(q.bearing_degrees, 145.1738);
}

#[test]
fn south_pole_reference_values() {
    // Arc from the South Pole to the Kaaba spans 90 + 21.42 = 111.42 degrees
    let q = qibla::calculate(-90.0, 5.0).unwrap();
    close_distance(q.distance_km, 12389.6153);
    close_bearing(q.bearing_degrees, 34.8262);
}

#[test]
fn raw_helpers_are_consistent_with_calculate() {
    let q = qibla::calculate(51.5074, -0.1278).unwrap();
    close_bearing(qibla::bearing_degrees(51.5074, -0.1278), q.bearing_degrees);
    close_distance(qibla::distance_to_kaaba_km(51.5074, -0.1278), q.distance_km);
    assert_eq!(
        qibla::cardinal_direction(q.bearing_degrees).unwrap(),
        q.cardinal_direction
    );
}

/// The boundaries of each 22.5°-wide sector resolve exactly per
/// `floor((bearing + 11.25) / 22.5)`, with the `11.25 + k*22.5` angles
/// belonging to the next point clockwise.
#[test]
fn cardinal_direction_sixteen_points() {
    assert_eq!(qibla::cardinal_direction(0.0), Some(Cardinal::North));
    assert_eq!(qibla::cardinal_direction(11.24), Some(Cardinal::North));
    assert_eq!(
        qibla::cardinal_direction(11.25),
        Some(Cardinal::NorthNorthEast)
    );
    assert_eq!(
        qibla::cardinal_direction(11.26),
        Some(Cardinal::NorthNorthEast)
    );
    assert_eq!(
        qibla::cardinal_direction(22.5),
        Some(Cardinal::NorthNorthEast)
    );
    assert_eq!(qibla::cardinal_direction(33.75), Some(Cardinal::NorthEast));
    assert_eq!(qibla::cardinal_direction(45.0), Some(Cardinal::NorthEast));
    assert_eq!(qibla::cardinal_direction(90.0), Some(Cardinal::East));
    assert_eq!(
        qibla::cardinal_direction(101.25),
        Some(Cardinal::EastSouthEast)
    );
    assert_eq!(qibla::cardinal_direction(123.75), Some(Cardinal::SouthEast));
    assert_eq!(
        qibla::cardinal_direction(146.25),
        Some(Cardinal::SouthSouthEast)
    );
    assert_eq!(qibla::cardinal_direction(168.75), Some(Cardinal::South));
    assert_eq!(
        qibla::cardinal_direction(191.25),
        Some(Cardinal::SouthSouthWest)
    );
    assert_eq!(qibla::cardinal_direction(213.75), Some(Cardinal::SouthWest));
    assert_eq!(
        qibla::cardinal_direction(236.25),
        Some(Cardinal::WestSouthWest)
    );
    assert_eq!(qibla::cardinal_direction(258.75), Some(Cardinal::West));
    assert_eq!(
        qibla::cardinal_direction(281.25),
        Some(Cardinal::WestNorthWest)
    );
    assert_eq!(qibla::cardinal_direction(303.75), Some(Cardinal::NorthWest));
    assert_eq!(
        qibla::cardinal_direction(326.25),
        Some(Cardinal::NorthNorthWest)
    );
    assert_eq!(
        qibla::cardinal_direction(348.74),
        Some(Cardinal::NorthNorthWest)
    );
    assert_eq!(qibla::cardinal_direction(348.75), Some(Cardinal::North));
}

#[test]
fn cardinal_direction_wraps_out_of_range_input() {
    assert_eq!(qibla::cardinal_direction(360.0), Some(Cardinal::North));
    assert_eq!(qibla::cardinal_direction(370.0), Some(Cardinal::North));
    assert_eq!(
        qibla::cardinal_direction(-90.0),
        Some(Cardinal::West) // -90 + 360 = 270
    );
    assert_eq!(
        qibla::cardinal_direction(735.0),
        Some(Cardinal::NorthNorthEast) // 735 - 720 = 15
    );
}

#[test]
fn cardinal_direction_rejects_non_finite() {
    assert!(qibla::cardinal_direction(f64::NAN).is_none());
    assert!(qibla::cardinal_direction(f64::INFINITY).is_none());
    assert!(qibla::cardinal_direction(f64::NEG_INFINITY).is_none());
}

#[test]
fn cardinal_is_serialized_compactly() {
    // Wire format stays "N", "NNE", ... , "NNW"
    assert_eq!(serde_json::to_string(&Cardinal::North).unwrap(), "\"N\"");
    assert_eq!(
        serde_json::to_string(&Cardinal::NorthNorthWest).unwrap(),
        "\"NNW\""
    );
    assert_eq!(Cardinal::SouthSouthEast.as_str(), "SSE");
    assert_eq!(Cardinal::SouthSouthEast.to_string(), "SSE");
}

#[test]
fn bearing_is_always_non_negative_when_input_valid() {
    // Same point, sweep longitudes across the antimeridian
    for lng in [-180.0, -179.9, 0.0, 179.9, 180.0] {
        for lat in [-89.9, -45.0, 0.0, 45.0, 89.9] {
            let q = qibla::calculate(lat, lng).unwrap();
            assert!(
                (0.0..360.0).contains(&q.bearing_degrees),
                "bearing {q:?} not in [0, 360)"
            );
            assert!(q.distance_km.is_finite());
            assert!(q.distance_km >= 0.0);
        }
    }
}

#[test]
fn bearing_is_symmetric_across_the_sign_of_longitude() {
    // Mirroring longitude about the Kaaba's meridian must mirror the bearing
    let west = qibla::calculate(30.0, 20.0).unwrap();
    let east = qibla::calculate(30.0, 59.652412).unwrap(); // 39.826206 + 19.826206
    let mirrored = 360.0 - west.bearing_degrees;
    assert!((east.bearing_degrees - mirrored).abs() < 1e-6);
}

#[test]
fn qibla_struct_is_serde_roundtrip() {
    let q = qibla::calculate(51.5074, -0.1278).unwrap();
    let json = serde_json::to_string(&q).unwrap();
    assert!(json.contains("\"ESE\""));
    let back: qibla::Qibla = serde_json::from_str(&json).unwrap();
    assert_eq!(q, back);
}

#[test]
fn validation_rejects_out_of_range_coordinates() {
    let err = qibla::calculate(90.01, 0.0).unwrap_err();
    assert!(matches!(err, CalcError::InvalidLatitude));
    let err = qibla::calculate(-90.01, 0.0).unwrap_err();
    assert!(matches!(err, CalcError::InvalidLatitude));
    let err = qibla::calculate(0.0, 180.01).unwrap_err();
    assert!(matches!(err, CalcError::InvalidLongitude));
    let err = qibla::calculate(0.0, -180.01).unwrap_err();
    assert!(matches!(err, CalcError::InvalidLongitude));
}

#[test]
fn validation_rejects_non_finite_coordinates() {
    let err = qibla::calculate(f64::NAN, 0.0).unwrap_err();
    assert!(matches!(err, CalcError::InvalidLatitude));
    let err = qibla::calculate(0.0, f64::INFINITY).unwrap_err();
    assert!(matches!(err, CalcError::InvalidLongitude));
}

#[test]
fn boundary_coordinates_are_accepted() {
    assert!(qibla::calculate(90.0, 0.0).is_ok());
    assert!(qibla::calculate(-90.0, 0.0).is_ok());
    assert!(qibla::calculate(0.0, 180.0).is_ok());
    assert!(qibla::calculate(0.0, -180.0).is_ok());
}
