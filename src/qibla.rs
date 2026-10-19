//! Qibla direction: great-circle bearing and distance to the Kaaba.
//!
//! The Qibla is the direction of the shortest great-circle path from a
//! location on the (spherical) Earth to the Kaaba in Mecca, measured in
//! degrees clockwise from true north.
//!
//! ```
//! use prayer_times_calculator::qibla::{self, Cardinal};
//!
//! // Istanbul -> Qibla is roughly south-south-east
//! let direction = qibla::calculate(41.0082, 28.9784).unwrap();
//! assert!((direction.bearing_degrees - 151.6).abs() < 1.0);
//! assert!(direction.distance_km > 2000.0);
//! assert_eq!(direction.cardinal_direction, Cardinal::SouthSouthEast);
//! ```

use crate::models::CalcError;
use serde::{Deserialize, Serialize};

/// Latitude of the Kaaba (Mecca) in degrees.
pub const KAABA_LATITUDE: f64 = 21.422487;
/// Longitude of the Kaaba (Mecca) in degrees.
pub const KAABA_LONGITUDE: f64 = 39.826206;
/// Mean Earth radius in kilometres, used for the spherical model distances.
pub const EARTH_RADIUS_KM: f64 = 6371.0;

/// Initial great-circle bearing from a location to the Kaaba, in degrees
/// clockwise from true north, normalized to `[0, 360)`.
///
/// Degenerate locations produce a value but the direction may be unstable:
/// at the exact antipode the formula feeds `atan2` values around ±1e-16, so
/// the result is effectively arbitrary, and at the geographic poles it
/// depends on the longitude offset. At the Kaaba itself the bearing is
/// defined as `0`.
pub fn bearing_degrees(latitude: f64, longitude: f64) -> f64 {
    let phi1 = latitude.to_radians();
    let phi2 = KAABA_LATITUDE.to_radians();
    let delta_lambda = (KAABA_LONGITUDE - longitude).to_radians();

    let y = delta_lambda.sin() * phi2.cos();
    let x = phi1.cos() * phi2.sin() - phi1.sin() * phi2.cos() * delta_lambda.cos();

    y.atan2(x).to_degrees().rem_euclid(360.0)
}

/// Great-circle (Haversine) distance from a location to the Kaaba in
/// kilometres, using the mean Earth radius (`EARTH_RADIUS_KM`).
///
/// This is the *spherical* model: real-world distances can deviate by up to
/// ~0.3% from the WGS-84 ellipsoid.
pub fn distance_to_kaaba_km(latitude: f64, longitude: f64) -> f64 {
    let phi1 = latitude.to_radians();
    let phi2 = KAABA_LATITUDE.to_radians();
    let delta_phi = phi2 - phi1;
    let delta_lambda = (KAABA_LONGITUDE - longitude).to_radians();

    let a = (delta_phi / 2.0).sin().powi(2)
        + phi1.cos() * phi2.cos() * (delta_lambda / 2.0).sin().powi(2);
    let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());

    EARTH_RADIUS_KM * c
}

/// One of the 16 compass points (e.g. `Cardinal::SSE`).
///
/// Serializes compactly: `"N"`, `"NNE"`, `"NE"`, … `"NNW"`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Cardinal {
    #[serde(rename = "N")]
    North,
    #[serde(rename = "NNE")]
    NorthNorthEast,
    #[serde(rename = "NE")]
    NorthEast,
    #[serde(rename = "ENE")]
    EastNorthEast,
    #[serde(rename = "E")]
    East,
    #[serde(rename = "ESE")]
    EastSouthEast,
    #[serde(rename = "SE")]
    SouthEast,
    #[serde(rename = "SSE")]
    SouthSouthEast,
    #[serde(rename = "S")]
    South,
    #[serde(rename = "SSW")]
    SouthSouthWest,
    #[serde(rename = "SW")]
    SouthWest,
    #[serde(rename = "WSW")]
    WestSouthWest,
    #[serde(rename = "W")]
    West,
    #[serde(rename = "WNW")]
    WestNorthWest,
    #[serde(rename = "NW")]
    NorthWest,
    #[serde(rename = "NNW")]
    NorthNorthWest,
}

impl Cardinal {
    /// All 16 points, ordered clockwise starting at north.
    pub const ALL: [Cardinal; 16] = [
        Cardinal::North,
        Cardinal::NorthNorthEast,
        Cardinal::NorthEast,
        Cardinal::EastNorthEast,
        Cardinal::East,
        Cardinal::EastSouthEast,
        Cardinal::SouthEast,
        Cardinal::SouthSouthEast,
        Cardinal::South,
        Cardinal::SouthSouthWest,
        Cardinal::SouthWest,
        Cardinal::WestSouthWest,
        Cardinal::West,
        Cardinal::WestNorthWest,
        Cardinal::NorthWest,
        Cardinal::NorthNorthWest,
    ];

    /// The conventional short code of this point (`"N"`, `"NNE"`, …).
    #[must_use]
    pub fn as_str(self) -> &'static str {
        const SHORT: [&str; 16] = [
            "N", "NNE", "NE", "ENE", "E", "ESE", "SE", "SSE", "S", "SSW", "SW", "WSW", "W", "WNW",
            "NW", "NNW",
        ];
        let index = Cardinal::ALL
            .iter()
            .position(|p| *p == self)
            .unwrap_or_default();
        SHORT[index]
    }
}

impl std::fmt::Display for Cardinal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

/// 16-point compass point for a bearing in degrees.
///
/// Values outside `[0, 360)` are wrapped. Exact boundary angles
/// (`11.25 + k * 22.5`) resolve to the next point clockwise. Returns `None`
/// for non-finite input.
#[must_use]
pub fn cardinal_direction(bearing_degrees: f64) -> Option<Cardinal> {
    if !bearing_degrees.is_finite() {
        return None;
    }
    let shifted = (bearing_degrees.rem_euclid(360.0) + 11.25).rem_euclid(360.0);
    Some(Cardinal::ALL[(shifted / 22.5) as usize])
}

/// Qibla direction result for a location.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Qibla {
    /// Input latitude in degrees.
    pub latitude: f64,
    /// Input longitude in degrees.
    pub longitude: f64,
    /// Initial great-circle bearing to the Kaaba, degrees clockwise from
    /// true north, normalized to `[0, 360)`.
    pub bearing_degrees: f64,
    /// Great-circle distance to the Kaaba in kilometres.
    pub distance_km: f64,
    /// Compass point derived from the (true) bearing.
    pub cardinal_direction: Cardinal,
}

/// Validate coordinates and compute the Qibla direction and distance.
///
/// # Errors
/// Returns [`CalcError::InvalidLatitude`] or [`CalcError::InvalidLongitude`]
/// when the coordinates are out of range.
pub fn calculate(latitude: f64, longitude: f64) -> Result<Qibla, CalcError> {
    if !(-90.0..=90.0).contains(&latitude) {
        return Err(CalcError::InvalidLatitude);
    }
    if !(-180.0..=180.0).contains(&longitude) {
        return Err(CalcError::InvalidLongitude);
    }

    let bearing = bearing_degrees(latitude, longitude);
    Ok(Qibla {
        latitude,
        longitude,
        bearing_degrees: bearing,
        distance_km: distance_to_kaaba_km(latitude, longitude),
        cardinal_direction: cardinal_direction(bearing)
            .expect("bearing is finite for validated coordinates"),
    })
}
