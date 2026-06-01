use serde::{Deserialize, Serialize};
use prayer_times_calculator::{Params, Settings, TimeOffsets};

#[derive(Debug, Deserialize)]
pub struct PrayerQueryParams {
    pub lat: f64,
    pub lng: f64,
    pub year: Option<i32>,
    pub month: Option<u32>,
    pub day: Option<u32>,
    pub timezone: Option<f64>,
    pub elv: Option<f64>,
    pub method: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DateComponents {
    pub year: i32,
    pub month: u32,
    pub day: u32,
}

impl std::fmt::Display for DateComponents {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{:02}-{:02}", self.year, self.month, self.day)
    }
}

#[derive(Debug, Deserialize)]
pub struct RangeRequest {
    pub lat: f64,
    pub lng: f64,
    pub elv: f64,
    pub timezone: f64,
    pub start_date: DateComponents,
    pub end_date: DateComponents,
    pub params: Params,
    #[serde(default)]
    pub settings: Settings,
    #[serde(default)]
    pub offsets: TimeOffsets,
}