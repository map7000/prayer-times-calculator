use crate::formatter;
use prayer_times_calculator::Times;
use serde::Serialize;

#[derive(Serialize)]
pub struct FormattedTimes {
    pub imsak: String,
    pub fajr: String,
    pub sunrise: String,
    pub dhuhr: String,
    pub asr: String,
    pub sunset: String,
    pub maghrib: String,
    pub isha: String,
    pub midnight: String,
}

// This enables the `times.into()` call in the handler
impl From<Times> for FormattedTimes {
    fn from(t: Times) -> Self {
        Self {
            imsak: formatter::format_time(t.imsak, true),
            fajr: formatter::format_time(t.fajr, true),
            sunrise: formatter::format_time(t.sunrise, true),
            dhuhr: formatter::format_time(t.dhuhr, true),
            asr: formatter::format_time(t.asr, true),
            sunset: formatter::format_time(t.sunset, true),
            maghrib: formatter::format_time(t.maghrib, true),
            isha: formatter::format_time(t.isha, true),
            midnight: formatter::format_time(t.midnight, true),
        }
    }
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
pub struct PrayerTimesResponse {
    pub status: String,
    pub data: FormattedTimes,
    pub request: RequestMeta,
}

#[derive(Serialize)]
pub struct RangeItem {
    pub date: String,
    pub fajr: String,
    pub sunrise: String,
    pub dhuhr: String,
    pub asr: String,
    pub maghrib: String,
    pub isha: String,
}

#[derive(Serialize)]
pub struct RangeResponse {
    pub status: String,
    pub data: Vec<RangeItem>,
    pub request: RequestMeta,
}