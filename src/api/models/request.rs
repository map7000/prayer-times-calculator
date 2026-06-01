use serde::Deserialize;
use chrono::NaiveDate;

#[derive(Deserialize, Debug)]
pub struct PrayerQueryParams {
    pub lat: f64,
    pub lng: f64,
    // Serde will automatically parse "?date=2026-06-02" into a NaiveDate
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