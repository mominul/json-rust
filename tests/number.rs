#[macro_use]
extern crate json;

use json::number::Number;
use std::f64;

#[test]
fn is_nan() {
    assert!(Number::from(f64::NAN).is_nan());
}

#[test]
fn is_zero() {
    assert!(Number::from(0).is_zero());
    assert!(Number::from_parts(true, 0, 0).is_zero());
    assert!(Number::from_parts(true, 0, 100).is_zero());
    assert!(Number::from_parts(true, 0, -100).is_zero());
    assert!(Number::from_parts(false, 0, 0).is_zero());
    assert!(Number::from_parts(false, 0, 100).is_zero());
    assert!(Number::from_parts(false, 0, -100).is_zero());
    assert!(!Number::from(f64::NAN).is_zero());
}

#[test]
fn is_empty() {
    assert!(Number::from(0).is_empty());
    assert!(Number::from(f64::NAN).is_empty());
}

#[test]
fn eq() {
    assert_eq!(
        Number::from_parts(true, 500, 0),
        Number::from_parts(true, 500, 0)
    );
}

#[test]
fn eq_normalize_left_positive() {
    assert_eq!(
        Number::from_parts(true, 5, 2),
        Number::from_parts(true, 500, 0)
    );
}

#[test]
fn eq_normalize_left_negative() {
    assert_eq!(
        Number::from_parts(true, 5, -2),
        Number::from_parts(true, 500, -4)
    );
}

#[test]
fn eq_normalize_right_positive() {
    assert_eq!(
        Number::from_parts(true, 500, 0),
        Number::from_parts(true, 5, 2)
    );
}

#[test]
fn eq_normalize_right_negative() {
    assert_eq!(
        Number::from_parts(true, 500, -4),
        Number::from_parts(true, 5, -2)
    );
}

#[test]
fn from_small_float() {
    assert_eq!(Number::from(0.05), Number::from_parts(true, 5, -2));
}

#[test]
fn from_very_small_float() {
    assert_eq!(Number::from(5e-50), Number::from_parts(true, 5, -50));
}

#[test]
fn from_big_float() {
    assert_eq!(Number::from(500), Number::from_parts(true, 500, 0));
}

#[test]
fn from_very_big_float() {
    assert_eq!(Number::from(5e50), Number::from_parts(true, 5, 50));
}

#[test]
fn into_very_small_float() {
    let number = Number::from_parts(true, 2225073858507201136, -326);

    assert_eq!(f64::from(number), 2.225073858507201e-308);
}

#[test]
fn as_fixed_point_u64() {
    assert_eq!(Number::from(1.2345).as_fixed_point_u64(4).unwrap(), 12345);
    assert_eq!(Number::from(1.2345).as_fixed_point_u64(2).unwrap(), 123);
    assert_eq!(Number::from(1.2345).as_fixed_point_u64(0).unwrap(), 1);

    assert_eq!(Number::from(5).as_fixed_point_u64(0).unwrap(), 5);
    assert_eq!(Number::from(5).as_fixed_point_u64(2).unwrap(), 500);
    assert_eq!(Number::from(5).as_fixed_point_u64(4).unwrap(), 50000);

    assert_eq!(Number::from(-1).as_fixed_point_u64(0), None);
    assert_eq!(Number::from(f64::NAN).as_fixed_point_u64(0), None);
}

#[test]
fn as_fixed_point_i64() {
    assert_eq!(Number::from(-1.2345).as_fixed_point_i64(4).unwrap(), -12345);
    assert_eq!(Number::from(-1.2345).as_fixed_point_i64(2).unwrap(), -123);
    assert_eq!(Number::from(-1.2345).as_fixed_point_i64(0).unwrap(), -1);

    assert_eq!(Number::from(-5).as_fixed_point_i64(0).unwrap(), -5);
    assert_eq!(Number::from(-5).as_fixed_point_i64(2).unwrap(), -500);
    assert_eq!(Number::from(-5).as_fixed_point_i64(4).unwrap(), -50000);

    assert_eq!(Number::from(-1).as_fixed_point_i64(0), Some(-1));
    assert_eq!(Number::from(f64::NAN).as_fixed_point_i64(0), None);
}

#[test]
fn convert_f64_precision() {
    assert_eq!(Number::from_parts(true, 4750000000000001, -18), 0.004750000000000001);
}

#[test]
fn issue_107() {
    let n = Number::from_parts(true, 1, -32768);
    assert_eq!(format!("{}", n), "1e-32768");
}