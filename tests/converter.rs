extern crate pretty_bytes;

use pretty_bytes::converter::Unit;
use pretty_bytes::converter::{convert, convert_with_smallest_unit};

#[test]
fn it_converts_bytes_to_human_readable_strings() {
    assert_eq!(convert(0_f64), "0 B");
    assert_eq!(convert(0.4_f64), "0.4 B");
    assert_eq!(convert(0.7_f64), "0.7 B");
    assert_eq!(convert(10_f64), "10 B");
    assert_eq!(convert(10.1_f64), "10.1 B");
    assert_eq!(convert(999_f64), "999 B");
    assert_eq!(convert(1001_f64), "1 kB");
    assert_eq!(convert(1e16), "10 PB");
    assert_eq!(convert(1e30), "1000000 YB");
}

#[test]
fn it_supports_negative_numbers() {
    assert_eq!(convert(-0.4_f64), "-0.4 B");
    assert_eq!(convert(-0.7_f64), "-0.7 B");
    assert_eq!(convert(-10.1_f64), "-10.1 B");
    assert_eq!(convert(-999_f64), "-999 B");
    assert_eq!(convert(-1001_f64), "-1 kB");
}

#[test]
fn it_supports_smallest_unit_specifications() {
    assert_eq!(convert_with_smallest_unit(1234.0, Unit::kB), "1.23 kB");
    assert_eq!(convert_with_smallest_unit(4.0, Unit::kB), "0.00 kB");
    assert_eq!(convert_with_smallest_unit(1000000.0, Unit::kB), "1 MB");
    assert_eq!(convert_with_smallest_unit(1230000.0, Unit::kB), "1.23 MB");
    assert_eq!(convert_with_smallest_unit(230.0, Unit::kB), "0.23 kB");
    assert_eq!(convert_with_smallest_unit(230000.0, Unit::MB), "0.23 MB");
}
