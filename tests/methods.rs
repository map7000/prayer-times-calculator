//! Tests for the calculation-method presets in `methods` and for the fact
//! that every preset produces sane output for an ordinary location/date.

use prayer_times_calculator::{
    calculator::PrayerCalculator,
    methods,
    models::{
        AngleOrMinutes, AsrJuristic, CalculationRequest, MidnightMode, Params, Settings,
        TimeOffsets,
    },
    presets,
};

#[test]
fn mwl_preset_values() {
    let p = methods::mwl();
    assert_eq!(p.fajr, AngleOrMinutes::Angle(18.0));
    assert_eq!(p.isha, AngleOrMinutes::Angle(17.0));
    assert_eq!(p.maghrib, AngleOrMinutes::Minutes(0.0));
    assert_eq!(p.midnight, MidnightMode::Standard);
}

#[test]
fn isna_preset_values() {
    let p = methods::isna();
    assert_eq!(p.fajr, AngleOrMinutes::Angle(15.0));
    assert_eq!(p.isha, AngleOrMinutes::Angle(15.0));
}

#[test]
fn egypt_preset_values() {
    let p = methods::egypt();
    assert_eq!(p.fajr, AngleOrMinutes::Angle(19.5));
    assert_eq!(p.isha, AngleOrMinutes::Angle(17.5));
}

#[test]
fn makkah_uses_ninety_minutes_for_isha() {
    let p = methods::makkah();
    assert_eq!(p.fajr, AngleOrMinutes::Angle(18.5));
    assert_eq!(p.isha, AngleOrMinutes::Minutes(90.0));
    assert_eq!(p.maghrib, AngleOrMinutes::Minutes(0.0));
    assert_eq!(p.midnight, MidnightMode::Standard);
}

#[test]
fn karachi_preset_values() {
    let p = methods::karachi();
    assert_eq!(p.fajr, AngleOrMinutes::Angle(18.0));
    assert_eq!(p.isha, AngleOrMinutes::Angle(18.0));
}

#[test]
fn tehran_preset_is_angle_maghrib_with_jafari_midnight() {
    let p = methods::tehran();
    assert_eq!(p.fajr, AngleOrMinutes::Angle(17.7));
    assert_eq!(p.isha, AngleOrMinutes::Angle(14.0));
    assert_eq!(p.maghrib, AngleOrMinutes::Angle(4.5));
    assert_eq!(p.midnight, MidnightMode::Jafari);
}

#[test]
fn jafari_preset_values() {
    let p = methods::jafari();
    assert_eq!(p.fajr, AngleOrMinutes::Angle(16.0));
    assert_eq!(p.isha, AngleOrMinutes::Angle(14.0));
    assert_eq!(p.maghrib, AngleOrMinutes::Angle(4.0));
    assert_eq!(p.midnight, MidnightMode::Jafari);
}

#[test]
fn minutes_based_isha_preset_values() {
    // Gulf and Qatar use 90 minutes for isha, Portugal uses 77 ...
    assert_eq!(methods::gulf().isha, AngleOrMinutes::Minutes(90.0));
    assert_eq!(methods::qatar().isha, AngleOrMinutes::Minutes(90.0));
    assert_eq!(methods::portugal().isha, AngleOrMinutes::Minutes(77.0));

    // ... while Kuwait keeps an angle
    assert_eq!(methods::kuwait().isha, AngleOrMinutes::Angle(17.5));
}

#[test]
fn portugal_shifts_maghrib_by_three_minutes() {
    let p = methods::portugal();
    assert_eq!(p.maghrib, AngleOrMinutes::Minutes(3.0));
    assert_eq!(p.fajr, AngleOrMinutes::Angle(18.0));
}

#[test]
fn jordan_shifts_maghrib_by_five_minutes() {
    let p = methods::jordan();
    assert_eq!(p.maghrib, AngleOrMinutes::Minutes(5.0));
    assert_eq!(p.isha, AngleOrMinutes::Angle(18.0));
}

#[test]
fn spot_check_angle_values_of_remaining_presets() {
    // (preset, fajr value, isha value — value() is kind-agnostic)
    macro_rules! check {
        ($fn_name:ident, $fajr:expr, $isha:expr) => {{
            let p = methods::$fn_name();
            assert_eq!(p.fajr.value(), $fajr, "fajr of {}", stringify!($fn_name));
            assert_eq!(p.isha.value(), $isha, "isha of {}", stringify!($fn_name));
        }};
    }
    check!(gulf, 19.5, 90.0);
    check!(singapore, 20.0, 18.0);
    check!(france, 12.0, 12.0);
    check!(turkey, 18.0, 17.0);
    check!(russia, 16.0, 15.0);
    check!(dubai, 18.2, 18.2);
    check!(jakim, 20.0, 18.0);
    check!(tunisia, 18.0, 18.0);
    check!(algeria, 18.0, 17.0);
    check!(kemenag, 20.0, 18.0);
    check!(morocco, 19.0, 17.0);
}

#[test]
fn presets_module_reexports_methods() {
    // `presets` is a re-export of `methods` — same values for a sample method
    assert_eq!(presets::makkah(), methods::makkah());
}

type PresetEntry = (&'static str, fn() -> Params);

#[test]
fn all_presets_calculate_for_a_temperate_case() {
    // London in mid-April: no polar degeneracy for any of the 22 methods.
    let presets_list: [PresetEntry; 22] = [
        ("mwl", methods::mwl),
        ("isna", methods::isna),
        ("egypt", methods::egypt),
        ("makkah", methods::makkah),
        ("karachi", methods::karachi),
        ("tehran", methods::tehran),
        ("jafari", methods::jafari),
        ("gulf", methods::gulf),
        ("kuwait", methods::kuwait),
        ("qatar", methods::qatar),
        ("singapore", methods::singapore),
        ("france", methods::france),
        ("turkey", methods::turkey),
        ("russia", methods::russia),
        ("dubai", methods::dubai),
        ("jakim", methods::jakim),
        ("tunisia", methods::tunisia),
        ("algeria", methods::algeria),
        ("kemenag", methods::kemenag),
        ("morocco", methods::morocco),
        ("portugal", methods::portugal),
        ("jordan", methods::jordan),
    ];

    for (name, params_fn) in presets_list {
        let req = CalculationRequest {
            year: 2026,
            month: 4,
            day: 15,
            lat: 51.5074,
            lng: -0.1278,
            elv: 0.0,
            timezone: 1.0,
            params: params_fn(),
            settings: Settings::default(),
            offsets: TimeOffsets::default(),
        };
        let times = PrayerCalculator::calculate(&req)
            .unwrap_or_else(|e| panic!("method {name}: calculation failed: {e}"));

        for (field, value) in [
            ("imsak", times.imsak),
            ("fajr", times.fajr),
            ("sunrise", times.sunrise),
            ("dhuhr", times.dhuhr),
            ("asr", times.asr),
            ("sunset", times.sunset),
            ("maghrib", times.maghrib),
            ("isha", times.isha),
            ("midnight", times.midnight),
        ] {
            assert!(
                value.is_finite(),
                "method {name}: {field} is not finite: {value}"
            );
        }
        assert!(
            times.fajr <= times.sunrise,
            "method {name}: fajr {} after sunrise {}",
            times.fajr,
            times.sunrise
        );
        assert!(
            times.isha >= times.maghrib,
            "method {name}: isha {} before maghrib {}",
            times.isha,
            times.maghrib
        );
    }
}

#[test]
fn asr_juristic_is_its_own_setting() {
    // The juristic Asr choice lives in Settings, not in the method params
    let s = Settings {
        asr: AsrJuristic::Hanafi,
        ..Settings::default()
    };
    assert!(matches!(s.asr, AsrJuristic::Hanafi));
}
