use std::{cmp, ops::Index};

pub fn convert(num: f64) -> String {
    let negative = if num.is_sign_positive() { "" } else { "-" };
    let num = num.abs();
    let units = ["B", "kB", "MB", "GB", "TB", "PB", "EB", "ZB", "YB"];
    if num < 1_f64 {
        return format!("{}{} {}", negative, num, "B");
    }
    let delimiter = 1000_f64;
    let exponent = cmp::min(
        (num.ln() / delimiter.ln()).floor() as i32,
        (units.len() - 1) as i32,
    );
    let pretty_bytes = format!("{:.2}", num / delimiter.powi(exponent))
        .parse::<f64>()
        .unwrap()
        * 1_f64;
    let unit = units[exponent as usize];
    format!("{}{} {}", negative, pretty_bytes, unit)
}

#[derive(Debug, Clone)]
pub enum Unit {
    B,
    kB,
    MB,
    GB,
    TB,
    PB,
    EB,
    ZB,
    YB,
}

///Converts bytes to a readable format rounding to the nearest hundedths place of a specified unit.
///
/// # Arguments
/// * `num` - number to convert
/// * `smallest_unit` - smallest unit to round to
///
/// # Examples
/// ```convert_and_round(1234.0, kB)``` => 1.23 kB
///
/// ```convert_and_round(234.0, kB)``` => 0.23 kB
///
/// ```convert_and_round(4.0, kB)``` => 0.00 kB
///
/// ```convert_and_round(1000000, kB)``` => 1 MB
pub fn convert_and_round(num: f64, smallest_unit: Unit) -> String {
    let power = smallest_unit.clone() as u8 as f64;
    let converted_num = num / 1000_f64.powf(power);
    if converted_num < 1.0 {
        let converted_num = num / 1000_f64.powf(power);
        return format!("{:.2} {:?}", converted_num, smallest_unit);
    }
    convert(num)
}
