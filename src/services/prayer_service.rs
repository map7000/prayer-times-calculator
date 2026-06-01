use chrono::NaiveDate;

use crate::api::error::AppError;
use crate::api::models::request::PrayerQueryParams;
use prayer_times_calculator::{calculator, methods, Times};

pub fn calculate_from_query(
    params: PrayerQueryParams,
    requested_date: NaiveDate,
) -> Result<(Times, String, (f64, f64), f64), AppError> {
    if params.lat < -90.0 || params.lat > 90.0 {
        return Err(AppError::BadRequest(
            "Latitude must be between -90 and 90".into(),
        ));
    }
    if params.lng < -180.0 || params.lng > 180.0 {
        return Err(AppError::BadRequest(
            "Longitude must be between -180 and 180".into(),
        ));
    }

    let method_name = params.method.unwrap_or_else(|| "MWL".to_string());
    let method = methods::get_method(&method_name).ok_or_else(|| {
        AppError::BadRequest(format!("Unknown method: {}", method_name))
    })?;

    let tz = params.timezone.unwrap_or(0.0);

    let times = calculator::compute(
        requested_date.year() as i32,
        requested_date.month() as u8,
        requested_date.day() as u8,
        params.lat,
        params.lng,
        tz,
        &method,
    );

    Ok((times, method_name, (params.lat, params.lng), tz))
}