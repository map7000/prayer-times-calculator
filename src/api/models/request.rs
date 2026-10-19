use chrono::NaiveDate;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct PrayerQueryParams {
    pub lat: f64,
    pub lng: f64,
    pub date: Option<NaiveDate>,
    pub method: Option<String>,
    pub timezone: Option<f64>,
}

#[derive(Deserialize, Debug)]
pub struct RangeRequest {
    pub lat: f64,
    pub lng: f64,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub method: Option<String>,
    pub timezone: Option<f64>,
}

#[derive(Deserialize, Debug)]
pub struct QiblaQueryParams {
    pub latitude: f64,
    pub longitude: f64,
    /// Height above the WGS-84 ellipsoid in metres (default 0).
    pub altitude_m: Option<f64>,
    /// Evaluation date for the magnetic model (defaults to today).
    pub date: Option<NaiveDate>,
}
