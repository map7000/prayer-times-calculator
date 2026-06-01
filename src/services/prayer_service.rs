use chrono::{Datelike, NaiveDate, Utc};
use prayer_times_calculator::{CalculationRequest, PrayerCalculator, Settings, TimeOffsets, Times};
use crate::api::models::request::{PrayerQueryParams, RangeRequest, DateComponents};
use crate::api::error::AppError;
use prayer_times_calculator::presets;

pub fn resolve_method(method: Option<&str>) -> prayer_times_calculator::Params {
    match method {
        Some("mwl") => presets::mwl(),
        Some("egypt") => presets::egypt(),
        Some("makkah") => presets::makkah(),
        Some("karachi") => presets::karachi(),
        Some("tehran") => presets::tehran(),
        Some("jafari") => presets::jafari(),
        Some("gulf") => presets::gulf(),
        Some("kuwait") => presets::kuwait(),
        Some("qatar") => presets::qatar(),
        Some("singapore") => presets::singapore(),
        Some("france") => presets::france(),
        Some("turkey") => presets::turkey(),
        Some("russia") => presets::russia(),
        Some("dubai") => presets::dubai(),
        Some("jakim") => presets::jakim(),
        Some("tunisia") => presets::tunisia(),
        Some("algeria") => presets::algeria(),
        Some("kemenag") => presets::kemenag(),
        Some("morocco") => presets::morocco(),
        Some("portugal") => presets::portugal(),
        Some("jordan") => presets::jordan(),
        _ => presets::isna(),
    }
}

pub fn calculate_from_query(params: PrayerQueryParams) -> Result<(Times, String, (f64, f64), f64), AppError> {
    if !(-90.0..=90.0).contains(&params.lat) {
        return Err(AppError::BadRequest("Latitude must be between -90 and 90".into()));
    }
    if !(-180.0..=180.0).contains(&params.lng) {
        return Err(AppError::BadRequest("Longitude must be between -180 and 180".into()));
    }

    let now = Utc::now();
    let year = params.year.unwrap_or(now.year());
    let month = params.month.unwrap_or(now.month());
    let day = params.day.unwrap_or(now.day());

    if NaiveDate::from_ymd_opt(year, month, day).is_none() {
        return Err(AppError::InvalidDate(
            "Invalid date".into(),
            format!("{}-{:02}-{:02} is not a valid date", year, month, day),
        ));
    }

    let method_name = params.method.clone().unwrap_or_else(|| "isna".into());
    let timezone = params.timezone.unwrap_or(0.0);

    let request = CalculationRequest {
        year, month, day,
        lat: params.lat, lng: params.lng,
        elv: params.elv.unwrap_or(0.0),
        timezone,
        params: resolve_method(Some(&method_name)),
        settings: Settings::default(),
        offsets: TimeOffsets::default(),
    };

    let times = PrayerCalculator::calculate(&request)
        .map_err(|e| AppError::Calculation(e.to_string()))?;

    Ok((times, method_name, (params.lat, params.lng), timezone))
}

pub fn calculate_from_request(req: CalculationRequest) -> Result<Times, AppError> {
    PrayerCalculator::calculate(&req)
        .map_err(|e| AppError::Calculation(e.to_string()))
}

pub fn calculate_range(req: RangeRequest) -> Result<Vec<(DateComponents, Times)>, AppError> {
    let start = NaiveDate::from_ymd_opt(req.start_date.year, req.start_date.month, req.start_date.day)
        .ok_or_else(|| AppError::InvalidDate("Invalid start date".into(), format!("{}", req.start_date)))?;
    let end = NaiveDate::from_ymd_opt(req.end_date.year, req.end_date.month, req.end_date.day)
        .ok_or_else(|| AppError::InvalidDate("Invalid end date".into(), format!("{}", req.end_date)))?;

    let mut results = Vec::new();
    let mut current = start;
    while current <= end {
        let calc_req = CalculationRequest {
            year: current.year(),
            month: current.month(),
            day: current.day(),
            lat: req.lat,
            lng: req.lng,
            elv: req.elv,
            timezone: req.timezone,
            params: req.params.clone(), // Ensure Params derives Clone
            settings: req.settings.clone(),
            offsets: req.offsets,
        };
        let times = calculate_from_request(calc_req)?;
        results.push((DateComponents { year: current.year(), month: current.month(), day: current.day() }, times));
        current = current.succ_opt().unwrap_or(current);
    }
    Ok(results)
}