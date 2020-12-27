/*
MIT License

Copyright (c) 2020 Philipp Schuster

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
*/

//! Parsing code for unit.

use crate::parse::error::ParseError;
use derive_more::Display;

#[derive(Debug, PartialEq, Copy, Clone, Display)]
pub enum Unit {
    #[display(fmt = "(Base)")]
    Base,
    #[display(fmt = "Kilobyte")]
    Kilo,
    #[display(fmt = "Megabyte")]
    Mega,
    #[display(fmt = "Gigabyte")]
    Giga,
    #[display(fmt = "Terabyte")]
    Tera,
    #[display(fmt = "Kibibyte")]
    Kibi,
    #[display(fmt = "Mibibyte")]
    Mibi,
    #[display(fmt = "Gibibyte")]
    Gibi,
    #[display(fmt = "Tebibyte")]
    Tebi,
}

impl Unit {
    /// Parses the [`super::ns::NumeralSystem`] from the normalized and validated slice of the input
    /// that corresponds to this type.
    /// * `part_str` slice of normalized and validated user input that corresponds to this type
    pub fn from_input(part_str: &str) -> Result<Unit, ParseError> {
        let x = match part_str {
            // attention! must match our regex!
            "" => Unit::Base,
            "k" | "kb" => Unit::Kilo,
            "m" | "mb" => Unit::Mega,
            "g" | "gb" => Unit::Giga,
            "t" | "tb" => Unit::Tera,
            "ki" | "kib" => Unit::Kibi,
            "mi" | "mib" => Unit::Mibi,
            "gi" | "gib" => Unit::Gibi,
            "ti" | "tib" => Unit::Tebi,
            _ => return Err(ParseError::InvalidUnit(part_str.to_owned())),
        };
        Ok(x)
    }

    /// Transforms a value in a specific value into the base unit.
    pub fn base_to_target(self, value: f64) -> f64 {
        match self {
            Unit::Base => value,
            Unit::Kilo => value / 1E3_f64,
            Unit::Mega => value / 1E6_f64,
            Unit::Giga => value / 1E9_f64,
            Unit::Tera => value / 1E12_f64,
            Unit::Kibi => value / 1024_f64,
            Unit::Mibi => value / 1024_f64.powf(2_f64),
            Unit::Gibi => value / 1024_f64.powf(3_f64),
            Unit::Tebi => value / 1024_f64.powf(4_f64),
        }
    }

    /*pub fn value_to_base_f64(self, value: f64) -> f64 {
        match self {
            Unit::Base => value,
            Unit::Kilo => value * 1E3_f64,
            Unit::Mega => value * 1E6_f64,
            Unit::Giga => value * 1E9_f64,
            Unit::Kibi => value * 1024_f64,
            Unit::Mibi => value * 1024_f64.powf(2_f64),
            Unit::Gibi => value * 1024_f64.powf(3_f64),
        }
    }*/

    /// Converts the integer input to the base unit.
    /// This is fine as long as we don't support fractional input
    pub fn value_to_base_u64(self, value: u64) -> u64 {
        match self {
            Unit::Base => value,
            Unit::Kilo => value * 1E3 as u64,
            Unit::Mega => value * 1E6 as u64,
            Unit::Giga => value * 1E9 as u64,
            Unit::Tera => value * 1E12 as u64,
            Unit::Kibi => value * 1024_u64,
            Unit::Mibi => value * 1024_u64.pow(2),
            Unit::Gibi => value * 1024_u64.pow(3),
            Unit::Tebi => value * 1024_u64.pow(4),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_parse_unit() {
        // only lowercase here
        // because user input gets transformed to lowercase
        assert_eq!(
            Unit::Base,
            Unit::from_input("").unwrap(),
            "Must be Unit::Base"
        );
        assert_eq!(
            Unit::Kilo,
            Unit::from_input("k").unwrap(),
            "Must be Unit::Kilo"
        );
        assert_eq!(
            Unit::Kibi,
            Unit::from_input("ki").unwrap(),
            "Must be Unit::Kibi"
        );
        assert_eq!(
            Unit::Kibi,
            Unit::from_input("kib").unwrap(),
            "Must be Unit::Kibi"
        );
        assert_eq!(
            Unit::Mega,
            Unit::from_input("m").unwrap(),
            "Must be Unit::Mega"
        );
        assert_eq!(
            Unit::Giga,
            Unit::from_input("gb").unwrap(),
            "Must be Unit::Giga"
        );
        assert_eq!(
            Unit::Tera,
            Unit::from_input("tb").unwrap(),
            "Must be Unit::Tera"
        );
        assert!(Unit::from_input("afaf").is_err());
    }
}
