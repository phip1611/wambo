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
use crate::parse::error::ParseError;
use crate::parse::ns::NumeralSystem;
use crate::parse::sign::Sign;
use crate::parse::unit::Unit;
use regex::Regex;

mod error;
mod ns;
mod sign;
pub mod unit;

/// Regex using named capture groups to validate the input.
/// Valid all are lower case inputs in all four known numeral systems
/// in all known units and with either minus sign or none.
pub const INPUT_REGEX: &str =
    "^(?P<sign>-)?(?P<ns>0(b|o|x){1})?(?P<value>[0-9abcdef]+)(?P<unit>[a-z]{1,4})?$";

/// Takes the input, normalizes it, checks if it is valid
/// and transform it into an usize value.
pub fn parse_input(input: &str) -> Result<Parsed, ParseError> {
    let normalized_input = normalize_input(input);

    // validate and get input split
    // via named regex capture groups
    let input_split = get_input_split(&normalized_input)?;

    let numeral_system = NumeralSystem::from_input(input_split.ns.unwrap_or(""));
    let unit = Unit::from_input(input_split.unit.unwrap_or(""))?;
    let sign = Sign::from_input(input_split.sign.unwrap_or(""));
    let value_str = input_split.value.unwrap().to_owned();

    Ok(Parsed::new(
        normalized_input,
        numeral_system,
        value_str,
        unit,
        sign,
    ))
}

#[derive(Debug)]
struct InputSplit<'a> {
    sign: Option<&'a str>,
    ns: Option<&'a str>,
    value: Option<&'a str>,
    unit: Option<&'a str>,
}

/// Validates the input against [`INPUT_REGEX`] and returns all the named
/// input groups.
fn get_input_split(normalized_input: &str) -> Result<InputSplit, ParseError> {
    let regex = Regex::new(INPUT_REGEX).unwrap();
    let captures = regex
        .captures(normalized_input)
        .ok_or_else(|| ParseError::InvalidFormat("Input doesn't match Regex".to_owned()))?;

    // the capture at index 0 is by definition of the crate always the main/long/full capture
    let is = InputSplit {
        sign: captures.name("sign").map(|m| m.as_str()),
        ns: captures.name("ns").map(|m| m.as_str()),
        value: captures.name("value").map(|m| m.as_str()),
        unit: captures.name("unit").map(|m| m.as_str()),
    };
    Ok(is)
}

#[derive(Debug)]
pub struct Parsed {
    normalized_input: String,
    numeral_system: NumeralSystem,
    unit: Unit,
    sign: Sign,
    value: u64,
    value_str: String,
}

impl Parsed {
    fn new(
        normalized_input: String,
        numeral_system: NumeralSystem,
        value_str: String,
        unit: Unit,
        sign: Sign,
    ) -> Self {
        let value = match numeral_system {
            NumeralSystem::Bin => u64::from_str_radix(&value_str, 2).unwrap(),
            NumeralSystem::Octal => u64::from_str_radix(&value_str, 8).unwrap(),
            NumeralSystem::Decimal => value_str.parse::<u64>().unwrap(),
            NumeralSystem::Hex => u64::from_str_radix(&value_str, 16).unwrap(),
        };

        Self {
            normalized_input,
            numeral_system,
            unit,
            sign,
            value,
            value_str,
        }
    }

    /// Getter for `normalized_input`.
    #[allow(dead_code)]
    pub fn normalized_input(&self) -> &str {
        &self.normalized_input
    }

    /// Getter for `numeral_system`.
    #[allow(dead_code)]
    pub fn numeral_system(&self) -> NumeralSystem {
        self.numeral_system
    }

    /// Getter for `sign`.
    #[allow(dead_code)]
    pub fn sign(&self) -> Sign {
        self.sign
    }

    /// Getter for `value`. The actual number
    /// as unsigned u64 as base unit.
    pub fn value(&self) -> u64 {
        self.value
    }

    /// Getter for `value_str`. The actual number
    /// but without numeral system or unit.
    #[allow(dead_code)]
    pub fn value_str(&self) -> u64 {
        self.value
    }

    /// Getter for `unit`.
    pub fn unit(&self) -> Unit {
        self.unit
    }
}

/// Removes all '_' from the input and transforms it to lowercase.
fn normalize_input(input: &str) -> String {
    input.trim().replace('_', "").to_lowercase()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_get_input_split() {
        let split = get_input_split("-0x123ki").unwrap();
        assert_eq!("-", split.sign.unwrap());
        assert_eq!("0x", split.ns.unwrap());
        assert_eq!("123", split.value.unwrap());
        assert_eq!("ki", split.unit.unwrap());

        let split = get_input_split("123").unwrap();
        assert!(split.sign.is_none());
        assert!(split.ns.is_none());
        assert_eq!("123", split.value.unwrap());
        assert!(split.unit.is_none());
    }

    #[test]
    fn test_parse_input() {
        let parsed = parse_input("-0xFMib").unwrap();
        assert_eq!(Sign::Negative, parsed.sign());
        assert_eq!(15, parsed.value());
        assert_eq!(NumeralSystem::Hex, parsed.numeral_system());
        assert_eq!(Unit::Mibi, parsed.unit());
    }

    #[test]
    fn test_regex() {
        let regex = Regex::new(INPUT_REGEX).unwrap();
        let captures = regex.captures("-0x123ki").unwrap();
    }

    #[test]
    fn test_normalized_input() {
        assert_eq!("0xdeadbeefmb", normalize_input("0x_dead_beefMB"));
        assert_eq!("123456789", normalize_input("123456789"));
    }
}
