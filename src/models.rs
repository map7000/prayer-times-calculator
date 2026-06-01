use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum AngleOrMinutes {
    Angle(f64),
    Minutes(f64),
}
impl AngleOrMinutes {
    #[inline]
    pub fn value(&self) -> f64 {
        match self {
            AngleOrMinutes::Angle(v) | AngleOrMinutes::Minutes(v) => *v,
        }
    }
    #[inline]
    pub fn is_minutes(&self) -> bool {
        matches!(self, AngleOrMinutes::Minutes(_))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum AsrJuristic {
    Standard,
    Hanafi,
    Custom(f64),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MidnightMode {
    Standard,
    Jafari,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HighLatMethod {
    NightMiddle,
    AngleBased,
    OneSeventh,
    None,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Params {
    pub fajr: AngleOrMinutes,
    pub isha: AngleOrMinutes,
    pub maghrib: AngleOrMinutes,
    pub midnight: MidnightMode,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Settings {
    pub imsak: AngleOrMinutes,
    pub dhuhr: AngleOrMinutes,
    pub asr: AsrJuristic,
    pub high_lats: HighLatMethod,
}
impl Default for Settings {
    fn default() -> Self {
        Self {
            imsak: AngleOrMinutes::Minutes(10.0),
            dhuhr: AngleOrMinutes::Minutes(0.0),
            asr: AsrJuristic::Standard,
            high_lats: HighLatMethod::NightMiddle,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct TimeOffsets {
    pub imsak: f64,
    pub fajr: f64,
    pub sunrise: f64,
    pub dhuhr: f64,
    pub asr: f64,
    pub sunset: f64,
    pub maghrib: f64,
    pub isha: f64,
    pub midnight: f64,
}
impl Default for TimeOffsets {
    fn default() -> Self {
        Self {
            imsak: 0.0,
            fajr: 0.0,
            sunrise: 0.0,
            dhuhr: 0.0,
            asr: 0.0,
            sunset: 0.0,
            maghrib: 0.0,
            isha: 0.0,
            midnight: 0.0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Times {
    pub imsak: f64,
    pub fajr: f64,
    pub sunrise: f64,
    pub dhuhr: f64,
    pub asr: f64,
    pub sunset: f64,
    pub maghrib: f64,
    pub isha: f64,
    pub midnight: f64,
}

/// API Request DTO
#[derive(Debug, Serialize, Deserialize)]
pub struct CalculationRequest {
    pub year: i32,
    pub month: u32,
    pub day: u32,
    pub lat: f64,
    pub lng: f64,
    pub elv: f64,
    pub timezone: f64,
    pub params: Params,
    #[serde(default)]
    pub settings: Settings,
    #[serde(default)]
    pub offsets: TimeOffsets,
}

#[derive(Debug, thiserror::Error)]
pub enum CalcError {
    #[error("Invalid date: {year}-{month}-{day}")]
    InvalidDate { year: i32, month: u32, day: u32 },
    #[error("Latitude must be between -90 and 90")]
    InvalidLatitude,
    #[error("Longitude must be between -180 and 180")]
    InvalidLongitude,
}
