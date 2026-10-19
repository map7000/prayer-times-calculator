use prayer_times_calculator::{magnetic, qibla};

use crate::api::error::AppError;
use crate::api::models::request::QiblaQueryParams;

/// Fully validated qibla computation.
///
/// Coordinate, altitude and date validation lives in the library functions
/// ([`qibla::calculate`] and [`magnetic::declination_degrees`]); errors are
/// surfaced as user-facing `400` messages derived from the library errors.
pub struct QiblaComputation {
    pub qibla: qibla::Qibla,
    pub declination_degrees: f64,
    pub magnetic_bearing_degrees: f64,
    pub date_used: chrono::NaiveDate,
}

pub fn calculate_from_query(
    params: QiblaQueryParams,
    requested_date: chrono::NaiveDate,
) -> Result<QiblaComputation, AppError> {
    let altitude_m = params.altitude_m.unwrap_or(0.0);

    let direction = qibla::calculate(params.latitude, params.longitude)
        .map_err(|e| AppError::BadRequest(e.to_string()))?;

    let declination_degrees = magnetic::declination_degrees(
        params.latitude,
        params.longitude,
        altitude_m,
        requested_date,
    )
    .map_err(|e| AppError::BadRequest(e.to_string()))?;

    let magnetic_bearing_degrees =
        magnetic::to_magnetic_bearing(direction.bearing_degrees, declination_degrees);

    Ok(QiblaComputation {
        qibla: direction,
        declination_degrees,
        magnetic_bearing_degrees,
        date_used: requested_date,
    })
}
