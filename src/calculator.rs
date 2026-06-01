use crate::math::dmath;
use crate::models::*;

pub struct PrayerCalculator;

impl PrayerCalculator {
    pub fn calculate(req: &CalculationRequest) -> Result<Times, CalcError> {
        // Validation
        if !(1..=12).contains(&req.month) || !(1..=31).contains(&req.day) {
            return Err(CalcError::InvalidDate {
                year: req.year,
                month: req.month,
                day: req.day,
            });
        }
        if !(-90.0..=90.0).contains(&req.lat) {
            return Err(CalcError::InvalidLatitude);
        }
        if !(-180.0..=180.0).contains(&req.lng) {
            return Err(CalcError::InvalidLongitude);
        }

        let jdate = Self::julian(req.year, req.month, req.day) - req.lng / (15.0 * 24.0);
        let ctx = CalcContext {
            lat: req.lat,
            lng: req.lng,
            elv: req.elv,
            timezone: req.timezone,
            jdate,
            params: req.params,
            settings: req.settings,
        };
        Ok(ctx.compute(req.offsets))
    }

    fn julian(year: i32, month: u32, day: u32) -> f64 {
        let (mut y, mut m) = (year as f64, month as f64);
        if m <= 2.0 {
            y -= 1.0;
            m += 12.0;
        }
        let a = (y / 100.0).floor();
        let b = 2.0 - a + (a / 4.0).floor();
        (365.25 * (y + 4716.0)).floor() + (30.6001 * (m + 1.0)).floor() + day as f64 + b - 1524.5
    }
}

struct CalcContext {
    lat: f64,
    lng: f64,
    elv: f64,
    timezone: f64,
    jdate: f64,
    params: Params,
    settings: Settings,
}

impl CalcContext {
    const ITERATIONS: usize = 2;

    fn sun_position(&self, jd: f64) -> (f64, f64) {
        let d = jd - 2451545.0;
        let g = dmath::fix_angle(357.529 + 0.98560028 * d);
        let q = dmath::fix_angle(280.459 + 0.98564736 * d);
        let l = dmath::fix_angle(q + 1.915 * dmath::sin_deg(g) + 0.020 * dmath::sin_deg(2.0 * g));
        let e = 23.439 - 0.00000036 * d;
        let ra =
            dmath::arctan2_deg(dmath::cos_deg(e) * dmath::sin_deg(l), dmath::cos_deg(l)) / 15.0;
        let eqt = q / 15.0 - dmath::fix_hour(ra);
        let decl = dmath::arcsin_deg(dmath::sin_deg(e) * dmath::sin_deg(l));
        (decl, eqt)
    }

    fn mid_day(&self, time: f64) -> f64 {
        let (_, eqt) = self.sun_position(self.jdate + time);
        dmath::fix_hour(12.0 - eqt)
    }

    fn sun_angle_time(&self, angle: f64, time: f64, ccw: bool) -> f64 {
        let (decl, _) = self.sun_position(self.jdate + time);
        let noon = self.mid_day(time);
        let cos_ha = (-dmath::sin_deg(angle) - dmath::sin_deg(decl) * dmath::sin_deg(self.lat))
            / (dmath::cos_deg(decl) * dmath::cos_deg(self.lat));
        let t = (1.0 / 15.0) * dmath::arccos_deg(cos_ha.clamp(-1.0, 1.0));
        if ccw { noon - t } else { noon + t }
    }

    fn asr_time(&self, factor: f64, time: f64) -> f64 {
        let (decl, _) = self.sun_position(self.jdate + time);
        let angle = -dmath::arccot_deg(factor + dmath::tan_deg((self.lat - decl).abs()));
        self.sun_angle_time(angle, time, false)
    }

    #[inline]
    fn rise_set_angle(&self) -> f64 {
        0.833 + 0.0347 * self.elv.sqrt()
    }

    #[inline]
    fn asr_factor(&self) -> f64 {
        match self.settings.asr {
            AsrJuristic::Standard => 1.0,
            AsrJuristic::Hanafi => 2.0,
            AsrJuristic::Custom(f) => f,
        }
    }

    fn compute_prayer_times(&self, times: &Times) -> Times {
        let day_portion = |t: f64| t / 24.0;
        let t = Times {
            imsak: day_portion(times.imsak),
            fajr: day_portion(times.fajr),
            sunrise: day_portion(times.sunrise),
            dhuhr: day_portion(times.dhuhr),
            asr: day_portion(times.asr),
            sunset: day_portion(times.sunset),
            maghrib: day_portion(times.maghrib),
            isha: day_portion(times.isha),
            midnight: 0.0,
        };
        Times {
            imsak: self.sun_angle_time(self.params.fajr.value(), t.imsak, true),
            fajr: self.sun_angle_time(self.params.fajr.value(), t.fajr, true),
            sunrise: self.sun_angle_time(self.rise_set_angle(), t.sunrise, true),
            dhuhr: self.mid_day(t.dhuhr),
            asr: self.asr_time(self.asr_factor(), t.asr),
            sunset: self.sun_angle_time(self.rise_set_angle(), t.sunset, false),
            maghrib: self.sun_angle_time(self.params.maghrib.value(), t.maghrib, false),
            isha: self.sun_angle_time(self.params.isha.value(), t.isha, false),
            midnight: 0.0,
        }
    }

    fn adjust_times(&self, mut times: Times) -> Times {
        for t in [
            &mut times.imsak,
            &mut times.fajr,
            &mut times.sunrise,
            &mut times.dhuhr,
            &mut times.asr,
            &mut times.sunset,
            &mut times.maghrib,
            &mut times.isha,
        ] {
            *t += self.timezone - self.lng / 15.0;
        }
        if self.settings.high_lats != HighLatMethod::None {
            times = self.adjust_high_lats(times);
        }
        if self.params.fajr.is_minutes() {
            times.imsak = times.fajr - self.params.fajr.value() / 60.0;
        }
        if self.params.maghrib.is_minutes() {
            times.maghrib = times.sunset + self.params.maghrib.value() / 60.0;
        }
        if self.params.isha.is_minutes() {
            times.isha = times.maghrib + self.params.isha.value() / 60.0;
        }
        times.dhuhr += self.settings.dhuhr.value() / 60.0;
        times
    }

    fn adjust_high_lats(&self, mut times: Times) -> Times {
        let night = self.time_diff(times.sunset, times.sunrise);
        times.imsak = self.adjust_hl_time(
            times.imsak,
            times.sunrise,
            self.params.fajr.value(),
            night,
            true,
        );
        times.fajr = self.adjust_hl_time(
            times.fajr,
            times.sunrise,
            self.params.fajr.value(),
            night,
            true,
        );
        times.isha = self.adjust_hl_time(
            times.isha,
            times.sunset,
            self.params.isha.value(),
            night,
            false,
        );
        times.maghrib = self.adjust_hl_time(
            times.maghrib,
            times.sunset,
            self.params.maghrib.value(),
            night,
            false,
        );
        times
    }

    fn adjust_hl_time(&self, time: f64, base: f64, angle: f64, night: f64, ccw: bool) -> f64 {
        let portion = self.night_portion(angle, night);
        let diff = if ccw {
            self.time_diff(time, base)
        } else {
            self.time_diff(base, time)
        };
        if time.is_nan() || diff > portion {
            if ccw { base - portion } else { base + portion }
        } else {
            time
        }
    }

    fn night_portion(&self, angle: f64, night: f64) -> f64 {
        let factor = match self.settings.high_lats {
            HighLatMethod::NightMiddle => 0.5,
            HighLatMethod::AngleBased => angle / 60.0,
            HighLatMethod::OneSeventh => 1.0 / 7.0,
            HighLatMethod::None => 0.5,
        };
        factor * night
    }

    #[inline]
    fn time_diff(&self, t1: f64, t2: f64) -> f64 {
        dmath::fix_hour(t2 - t1)
    }

    fn compute(&self, offsets: TimeOffsets) -> Times {
        let mut times = Times {
            imsak: 5.0,
            fajr: 5.0,
            sunrise: 6.0,
            dhuhr: 12.0,
            asr: 13.0,
            sunset: 18.0,
            maghrib: 18.0,
            isha: 18.0,
            midnight: 0.0,
        };
        for _ in 0..Self::ITERATIONS {
            times = self.compute_prayer_times(&times);
        }
        times = self.adjust_times(times);

        times.midnight = if self.params.midnight == MidnightMode::Jafari {
            times.sunset + self.time_diff(times.sunset, times.fajr) / 2.0
        } else {
            times.sunset + self.time_diff(times.sunset, times.sunrise) / 2.0
        };

        let apply_offset = |t: f64, off: f64| t + off / 60.0;
        Times {
            imsak: apply_offset(times.imsak, offsets.imsak),
            fajr: apply_offset(times.fajr, offsets.fajr),
            sunrise: apply_offset(times.sunrise, offsets.sunrise),
            dhuhr: apply_offset(times.dhuhr, offsets.dhuhr),
            asr: apply_offset(times.asr, offsets.asr),
            sunset: apply_offset(times.sunset, offsets.sunset),
            maghrib: apply_offset(times.maghrib, offsets.maghrib),
            isha: apply_offset(times.isha, offsets.isha),
            midnight: apply_offset(times.midnight, offsets.midnight),
        }
    }
}
