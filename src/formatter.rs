use crate::math::dmath;

pub fn format_time(time: f64, format_24h: bool) -> String {
    if time.is_nan() {
        return "-----".into();
    }
    let time = dmath::fix_hour(time + 0.5 / 60.0);
    let hours = time.floor() as i32;
    let minutes = ((time - hours as f64) * 60.0).floor() as i32;
    if format_24h {
        format!("{hours:02}:{minutes:02}")
    } else {
        let suffix = if hours < 12 { "AM" } else { "PM" };
        let h12 = if hours % 12 == 0 { 12 } else { hours % 12 };
        format!("{h12:02}:{minutes:02} {suffix}")
    }
}
