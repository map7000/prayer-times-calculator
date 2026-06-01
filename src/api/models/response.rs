use serde::Serialize;
use prayer_times_calculator::Times;

#[derive(Serialize)]
pub struct PrayerTimesResponse {
    pub status: String,
    pub data: Times,
    pub request: RequestMeta,
}

#[derive(Serialize)]
pub struct RequestMeta {
    pub date: String,
    pub coordinates: (f64, f64),
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<f64>,
}

#[derive(Serialize)]
pub struct RangeResponse {
    pub status: String,
    pub count: usize,
    pub data: Vec<RangeItem>,
}

#[derive(Serialize)]
pub struct RangeItem {
    pub date: String,
    pub times: RangeTimes,
}

#[derive(Serialize)]
pub struct RangeTimes {
    pub fajr: f64,
    pub sunrise: f64,
    pub dhuhr: f64,
    pub asr: f64,
    pub maghrib: f64,
    pub isha: f64,
}

impl From<Times> for RangeTimes {
    fn from(t: Times) -> Self {
        Self {
            fajr: t.fajr, sunrise: t.sunrise, dhuhr: t.dhuhr,
            asr: t.asr, maghrib: t.maghrib, isha: t.isha,
        }
    }
}