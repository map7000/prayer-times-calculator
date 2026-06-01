pub mod dmath {
    #[inline]
    pub fn dtr(d: f64) -> f64 {
        d.to_radians()
    }
    #[inline]
    pub fn rtd(r: f64) -> f64 {
        r.to_degrees()
    }
    #[inline]
    pub fn sin_deg(d: f64) -> f64 {
        d.to_radians().sin()
    }
    #[inline]
    pub fn cos_deg(d: f64) -> f64 {
        d.to_radians().cos()
    }
    #[inline]
    pub fn tan_deg(d: f64) -> f64 {
        d.to_radians().tan()
    }
    #[inline]
    pub fn arcsin_deg(d: f64) -> f64 {
        d.asin().to_degrees()
    }
    #[inline]
    pub fn arccos_deg(d: f64) -> f64 {
        d.acos().to_degrees()
    }
    #[inline]
    pub fn arctan_deg(d: f64) -> f64 {
        d.atan().to_degrees()
    }
    #[inline]
    pub fn arccot_deg(x: f64) -> f64 {
        (1.0 / x).atan().to_degrees()
    }
    #[inline]
    pub fn arctan2_deg(y: f64, x: f64) -> f64 {
        y.atan2(x).to_degrees()
    }
    #[inline]
    pub fn fix_angle(a: f64) -> f64 {
        a.rem_euclid(360.0)
    }
    #[inline]
    pub fn fix_hour(a: f64) -> f64 {
        a.rem_euclid(24.0)
    }
}
