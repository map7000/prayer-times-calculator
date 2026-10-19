//! Tests for the degree-based math helpers in `math::dmath`.

use prayer_times_calculator::math::dmath;

const EPS: f64 = 1e-12;

fn close(a: f64, b: f64) -> bool {
    (a - b).abs() < EPS
}

#[test]
fn dtr_rtd_are_inverse() {
    for deg in [
        -720.5, -180.0, -90.0, -0.25, 0.0, 45.0, 179.5, 360.0, 1080.25,
    ] {
        assert!(
            close(dmath::rtd(dmath::dtr(deg)), deg),
            "roundtrip failed for {deg}"
        );
    }
}

#[test]
fn dtr_converts_180_degrees_to_pi() {
    assert!(close(dmath::dtr(180.0), std::f64::consts::PI));
}

#[test]
fn rtd_converts_pi_to_180_degrees() {
    assert!(close(dmath::rtd(std::f64::consts::PI), 180.0));
}

#[test]
fn trig_functions_work_in_degrees() {
    assert!(close(dmath::sin_deg(30.0), 0.5));
    assert!(close(dmath::sin_deg(0.0), 0.0));
    assert!(close(dmath::sin_deg(90.0), 1.0));
    assert!(close(dmath::cos_deg(0.0), 1.0));
    assert!(close(dmath::cos_deg(60.0), 0.5));
    assert!(close(dmath::cos_deg(90.0), 0.0));
    assert!(close(dmath::tan_deg(0.0), 0.0));
    assert!(close(dmath::tan_deg(45.0), 1.0));
}

#[test]
fn inverse_trig_functions_work_in_degrees() {
    assert!(close(dmath::arcsin_deg(0.5), 30.0));
    assert!(close(dmath::arcsin_deg(1.0), 90.0));
    assert!(close(dmath::arccos_deg(0.5), 60.0));
    assert!(close(dmath::arccos_deg(1.0), 0.0));
    assert!(close(dmath::arctan_deg(1.0), 45.0));
    // cot(x) = 1/tan(x), so arccot(1) = 45 and arccot(0) = 90
    assert!(close(dmath::arccot_deg(1.0), 45.0));
    assert!(close(dmath::arccot_deg(0.0), 90.0));
}

#[test]
fn arctan2_quadrants() {
    assert!(close(dmath::arctan2_deg(1.0, 1.0), 45.0));
    assert!(close(dmath::arctan2_deg(-1.0, 1.0), -45.0));
    assert!(close(dmath::arctan2_deg(-1.0, -1.0), -135.0));
    assert!(close(dmath::arctan2_deg(1.0, -1.0), 135.0));
    assert!(close(dmath::arctan2_deg(0.0, 1.0), 0.0));
    assert!(close(dmath::arctan2_deg(0.0, -1.0), 180.0));
}

#[test]
fn fix_angle_wraps_into_0_360() {
    assert!(close(dmath::fix_angle(0.0), 0.0));
    assert!(close(dmath::fix_angle(360.0), 0.0));
    assert!(close(dmath::fix_angle(370.0), 10.0));
    assert!(close(dmath::fix_angle(360.5), 0.5));
    assert!(close(dmath::fix_angle(-10.0), 350.0));
    assert!(close(dmath::fix_angle(-361.0), 359.0));
    assert!(close(dmath::fix_angle(1080.0), 0.0));
    // never negative, never >= 360
    let a = dmath::fix_angle(-1234.56);
    assert!((0.0..360.0).contains(&a));
}

#[test]
fn fix_hour_wraps_into_0_24() {
    assert!(close(dmath::fix_hour(0.0), 0.0));
    assert!(close(dmath::fix_hour(24.0), 0.0));
    assert!(close(dmath::fix_hour(25.0), 1.0));
    assert!(close(dmath::fix_hour(24.5), 0.5));
    assert!(close(dmath::fix_hour(-1.0), 23.0));
    assert!(close(dmath::fix_hour(-49.0), 23.0));
    let h = dmath::fix_hour(-36.5);
    assert!((0.0..24.0).contains(&h));
}
