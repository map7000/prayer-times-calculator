use crate::models::*;

macro_rules! method {
    ($fajr:expr, $isha:expr, $maghrib:expr, $midnight:expr) => {
        Params {
            fajr: AngleOrMinutes::Angle($fajr),
            isha: $isha,
            maghrib: $maghrib,
            midnight: $midnight,
        }
    };
}

pub fn mwl() -> Params {
    method!(
        18.0,
        AngleOrMinutes::Angle(17.0),
        AngleOrMinutes::Minutes(0.0),
        MidnightMode::Standard
    )
}
pub fn isna() -> Params {
    method!(
        15.0,
        AngleOrMinutes::Angle(15.0),
        AngleOrMinutes::Minutes(0.0),
        MidnightMode::Standard
    )
}
pub fn egypt() -> Params {
    method!(
        19.5,
        AngleOrMinutes::Angle(17.5),
        AngleOrMinutes::Minutes(0.0),
        MidnightMode::Standard
    )
}
pub fn makkah() -> Params {
    method!(
        18.5,
        AngleOrMinutes::Minutes(90.0),
        AngleOrMinutes::Minutes(0.0),
        MidnightMode::Standard
    )
}
pub fn karachi() -> Params {
    method!(
        18.0,
        AngleOrMinutes::Angle(18.0),
        AngleOrMinutes::Minutes(0.0),
        MidnightMode::Standard
    )
}
pub fn tehran() -> Params {
    method!(
        17.7,
        AngleOrMinutes::Angle(14.0),
        AngleOrMinutes::Angle(4.5),
        MidnightMode::Jafari
    )
}
pub fn jafari() -> Params {
    method!(
        16.0,
        AngleOrMinutes::Angle(14.0),
        AngleOrMinutes::Angle(4.0),
        MidnightMode::Jafari
    )
}
pub fn gulf() -> Params {
    method!(
        19.5,
        AngleOrMinutes::Minutes(90.0),
        AngleOrMinutes::Minutes(0.0),
        MidnightMode::Standard
    )
}
pub fn kuwait() -> Params {
    method!(
        18.0,
        AngleOrMinutes::Angle(17.5),
        AngleOrMinutes::Minutes(0.0),
        MidnightMode::Standard
    )
}
pub fn qatar() -> Params {
    method!(
        18.0,
        AngleOrMinutes::Minutes(90.0),
        AngleOrMinutes::Minutes(0.0),
        MidnightMode::Standard
    )
}
pub fn singapore() -> Params {
    method!(
        20.0,
        AngleOrMinutes::Angle(18.0),
        AngleOrMinutes::Minutes(0.0),
        MidnightMode::Standard
    )
}
pub fn france() -> Params {
    method!(
        12.0,
        AngleOrMinutes::Angle(12.0),
        AngleOrMinutes::Minutes(0.0),
        MidnightMode::Standard
    )
}
pub fn turkey() -> Params {
    method!(
        18.0,
        AngleOrMinutes::Angle(17.0),
        AngleOrMinutes::Minutes(0.0),
        MidnightMode::Standard
    )
}
pub fn russia() -> Params {
    method!(
        16.0,
        AngleOrMinutes::Angle(15.0),
        AngleOrMinutes::Minutes(0.0),
        MidnightMode::Standard
    )
}
pub fn dubai() -> Params {
    method!(
        18.2,
        AngleOrMinutes::Angle(18.2),
        AngleOrMinutes::Minutes(0.0),
        MidnightMode::Standard
    )
}
pub fn jakim() -> Params {
    method!(
        20.0,
        AngleOrMinutes::Angle(18.0),
        AngleOrMinutes::Minutes(0.0),
        MidnightMode::Standard
    )
}
pub fn tunisia() -> Params {
    method!(
        18.0,
        AngleOrMinutes::Angle(18.0),
        AngleOrMinutes::Minutes(0.0),
        MidnightMode::Standard
    )
}
pub fn algeria() -> Params {
    method!(
        18.0,
        AngleOrMinutes::Angle(17.0),
        AngleOrMinutes::Minutes(0.0),
        MidnightMode::Standard
    )
}
pub fn kemenag() -> Params {
    method!(
        20.0,
        AngleOrMinutes::Angle(18.0),
        AngleOrMinutes::Minutes(0.0),
        MidnightMode::Standard
    )
}
pub fn morocco() -> Params {
    method!(
        19.0,
        AngleOrMinutes::Angle(17.0),
        AngleOrMinutes::Minutes(0.0),
        MidnightMode::Standard
    )
}
pub fn portugal() -> Params {
    method!(
        18.0,
        AngleOrMinutes::Minutes(77.0),
        AngleOrMinutes::Minutes(3.0),
        MidnightMode::Standard
    )
}
pub fn jordan() -> Params {
    method!(
        18.0,
        AngleOrMinutes::Angle(18.0),
        AngleOrMinutes::Minutes(5.0),
        MidnightMode::Standard
    )
}
