use chrono::{Datelike, NaiveDate};
use prayer_times_calculator::{
    calculator::PrayerCalculator,
    methods,
    models::{CalculationRequest, Settings, TimeOffsets},
};

use crate::api::error::AppError;
use crate::api::models::request::PrayerQueryParams;

pub fn calculate_from_query(
    params: PrayerQueryParams,
    requested_date: NaiveDate,
) -> Result<(prayer_times_calculator::Times, String, (f64, f64), f64), AppError> {
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

    // Normalize method name and map it to the correct Params preset
    let method_name = params.method.unwrap_or_else(|| "MWL".to_string()).to_lowercase();
    
    let method_params = match method_name.as_str() {
        "mwl" => methods::mwl(),
        "isna" => methods::isna(),
        "egypt" => methods::egypt(),
        "makkah" => methods::makkah(),
        "karachi" => methods::karachi(),
        "tehran" | "jafari" => methods::jafari(),
        "gulf" => methods::gulf(),
        "kuwait" => methods::kuwait(),
        "qatar" => methods::qatar(),
        "singapore" => methods::singapore(),
        "france" => methods::france(),
        "turkey" => methods::turkey(),
        "russia" => methods::russia(),
        "dubai" => methods::dubai(),
        "jakim" => methods::jakim(),
        "tunisia" => methods::tunisia(),
        "algeria" => methods::algeria(),
        "kemenag" => methods::kemenag(),
        "morocco" => methods::morocco(),
        "portugal" => methods::portugal(),
        "jordan" => methods::jordan(),
        _ => methods::mwl(), // Safe default fallback
    };

    // Build the request object expected by PrayerCalculator
    let req = CalculationRequest {
        year: requested_date.year(),
        month: requested_date.month(),
        day: requested_date.day(),
        lat: params.lat,
        lng: params.lng,
        elv: 0.0, // Default elevation
        timezone: params.timezone.unwrap_or(0.0),
        params: method_params,
        settings: Settings::default(),
        offsets: TimeOffsets::default(),
    };

    // Execute calculation
    let times = PrayerCalculator::calculate(&req)
        .map_err(|e| AppError::Calculation(e.to_string()))?;

    Ok((times, method_name, (params.lat, params.lng), req.timezone))
}