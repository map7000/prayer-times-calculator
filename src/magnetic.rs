//! Magnetic declination via the NOAA World Magnetic Model.
//!
//! The [`world_magnetic_model`] crate ships the WMM2020 and WMM2025
//! coefficient sets as compiled-in constants (no model files at runtime) and
//! selects the correct epoch automatically from the evaluation date. The
//! bundled models are valid from **2020-01-01 to 2029-12-31**.
//!
//! # Precision
//!
//! The NOAA implementation computes in single precision (`f32`): declination
//! values are meaningful to about 6 significant digits, and input
//! coordinates/altitudes are truncated from `f64` to `f32` on entry
//! (an error of at most ~1e-5 degrees, well below the ~0.25° WMM
//! uncertainty). The extra digits visible on returned `f64` values are an
//! artifact of the conversion and carry no information.
//!
//! ```
//! use chrono::NaiveDate;
//! use prayer_times_calculator::{magnetic, qibla};
//!
//! let date = NaiveDate::from_ymd_opt(2026, 10, 19).unwrap();
//!
//! // Istanbul: positive (easterly) declination
//! let declination =
//!     magnetic::declination_degrees(41.0082, 28.9784, 0.0, date).unwrap();
//! assert!(declination > 0.0);
//!
//! // A magnetic compass reads `true bearing - declination`
//! let true_bearing = qibla::bearing_degrees(41.0082, 28.9784);
//! let magnetic_bearing = magnetic::to_magnetic_bearing(true_bearing, declination);
//! assert!((magnetic_bearing - (true_bearing - declination)).abs() < 1e-9);
//! ```

use chrono::{Datelike, NaiveDate};
use world_magnetic_model::time::Date as WmmDate;
use world_magnetic_model::uom::si::angle::degree;
use world_magnetic_model::uom::si::f32::{Angle, Length};
use world_magnetic_model::uom::si::length::meter;
use world_magnetic_model::GeomagneticField;

/// Re-export for advanced use (field components, uncertainty, warnings…).
pub use world_magnetic_model;

#[derive(Debug, thiserror::Error)]
pub enum MagneticError {
    #[error("date outside of model validity range (2020-01-01 to 2029-12-31)")]
    DateOutsideOfValidityRange,
    #[error("height above WGS 84 ellipsoid outside of model validity range (-1 km to 850 km)")]
    HeightOutsideOfValidityRange,
    #[error("invalid latitude (-90 to 90)")]
    InvalidLatitude,
    #[error("invalid longitude (-180 to 180)")]
    InvalidLongitude,
}

fn map_error(e: world_magnetic_model::Error) -> MagneticError {
    match e {
        world_magnetic_model::Error::DateOutsideOfValidityRange { .. } => {
            MagneticError::DateOutsideOfValidityRange
        }
        world_magnetic_model::Error::HeightOutsideOfValidityRange { .. } => {
            MagneticError::HeightOutsideOfValidityRange
        }
        world_magnetic_model::Error::InvalidLatitude => MagneticError::InvalidLatitude,
        world_magnetic_model::Error::InvalidLongitude => MagneticError::InvalidLongitude,
    }
}

/// Magnetic declination at a WGS-84 coordinate on a date, in degrees east of
/// true north (positive east, negative west).
///
/// * `altitude_m` — height above the WGS-84 ellipsoid in metres,
///   valid from -1,000 to the 850,000 m.
/// * `date` — evaluation date; falls within the model's
///   2020-01-01 to 2029-12-31 validity window.
pub fn declination_degrees(
    latitude: f64,
    longitude: f64,
    altitude_m: f64,
    date: NaiveDate,
) -> Result<f64, MagneticError> {
    let wmm_date = WmmDate::from_ordinal_date(date.year(), date.ordinal() as u16)
        .map_err(|_| MagneticError::DateOutsideOfValidityRange)?;

    let field = GeomagneticField::new(
        Length::new::<meter>(altitude_m as f32),
        Angle::new::<degree>(latitude as f32),
        Angle::new::<degree>(longitude as f32),
        wmm_date,
    )
    .map_err(map_error)?;

    Ok(field.declination().get::<degree>() as f64)
}

/// Convert a true-north bearing to a compass-reading magnetic bearing by
/// subtracting easterly-declination and normalizing into `[0, 360)`.
pub fn to_magnetic_bearing(true_bearing_degrees: f64, declination_degrees: f64) -> f64 {
    (true_bearing_degrees - declination_degrees).rem_euclid(360.0)
}
