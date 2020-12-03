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
use regex::Regex;


/// Pattern to capture only the optional numeral system (hex, bin) of the input.
pub const NUMERAL_SYSTEM_PATTERN: &str = "(0(b|x){1})?";
/// Pattern to capture only the binary, decimal, or hexadecimal part of the input.
pub const NUM_PATTERN: &str = "([0-9abcdef]+)";
/// Pattern to capture only the optional unit of the input.
pub const UNIT_PATTERN: &str = "(k|kb|m|mb|g|gb|ki|kib|mi|mib|gi|gib)?$";

pub struct Parsed {
    original_input: String,
    normalized_input: String,
    /// Just the string that describes the unit (e.g. k, or mib
    unit_string: String,
    // /// Numeral system string (0x or 0b)
    // ns_string: String,
    /// the actual number string (dec, hex, or bin)
    number_string: String,
    /// The parsed unit
    unit: Unit,
    /// The parsed numeral system
    numeral_sysem: NumeralSystem,
    /// The actual value, after taking care of constraints
    /// from units and numeric systems.
    actual_value: usize,
}

impl Parsed {
    pub fn original_input(&self) -> &str {
        &self.original_input
    }
    pub fn normalized_input(&self) -> &str {
        &self.normalized_input
    }
    pub fn unit_string(&self) -> &str {
        &self.unit_string
    }
    pub fn number_string(&self) -> &str {
        &self.number_string
    }
    pub fn unit(&self) -> Unit {
        self.unit
    }
    pub fn numeral_sysem(&self) -> &NumeralSystem {
        &self.numeral_sysem
    }
    pub fn actual_value(&self) -> usize {
        self.actual_value
    }
}

#[derive(Debug, PartialEq)]
pub enum NumeralSystem {
    Decimal,
    Bin,
    Hex,
}

impl NumeralSystem {
    fn from_input(normalized_input: &str) -> NumeralSystem {
        if normalized_input.starts_with("0x") {
            NumeralSystem::Hex
        } else if normalized_input.starts_with("0b") {
            NumeralSystem::Bin
        } else {
            NumeralSystem::Decimal
        }
    }
}

/// Builds the full Regex for the input pattern. Possible values are for example:
/// * `12234` (decimal)
/// * `12234MB` (decimal megabyte)
/// * `12234gib` (decimal gibibyte)
/// * `0x2000` (hex)
/// * `0b10000000_11110000` (bin)
/// * `0x60_20kb` (hex kilobyte)
fn build_full_regex() -> Regex {
    Regex::new(
        &format!("^{}{}{}$", NUMERAL_SYSTEM_PATTERN, NUM_PATTERN, UNIT_PATTERN)
    ).expect("Regex must be valid")
}

/// Takes the input, normalizes it, checks if it is valid
/// and transform it into an usize value.
pub fn parse_input(input: &str) -> Result<Parsed, &str> {
    let orig_input = input;
    let normalized_input = input_normalizer(input);
    if !input_valid(&normalized_input) {
        Err("The input is not valid.")
    } else {
        let ns = NumeralSystem::from_input(&normalized_input);

        // remove leading 0x or 0b
        let input_without_ns = if ns == NumeralSystem::Decimal {
            normalized_input.clone()
        } else {
            normalized_input[2..normalized_input.len()].to_string()
        };

        let matches = Regex::new(NUM_PATTERN).unwrap().captures(&input_without_ns).unwrap();
        // we only can have that single match at this point
        let regmatch = matches.get(0).unwrap();
        let num_str = &input_without_ns[regmatch.range()];
        let unit_str = &input_without_ns[regmatch.end()..input_without_ns.len()];
        let unit = Unit::from_str(unit_str);

        let actual_value = match ns {
            NumeralSystem::Decimal => {num_str.parse::<usize>().unwrap()}
            NumeralSystem::Bin => {usize::from_str_radix(num_str, 2).unwrap()}
            NumeralSystem::Hex => {usize::from_str_radix(num_str, 16).unwrap()}
        };
        let actual_value = Unit::value_to_base(actual_value, unit);

        Ok(Parsed {
            original_input: orig_input.to_string(),
            normalized_input: normalized_input.to_string(),
            unit_string: unit_str.to_string(),
            number_string: num_str.to_string(),
            unit,
            numeral_sysem: ns,
            actual_value: actual_value
        })
    }
}

/// Removes all '_' from the input and transforms it to lowercase.
fn input_normalizer(input: &str) -> String {
    input.trim()
        .replace('_', "")
        .to_lowercase()
}

/// Checks if the input is valid. The input is normalize.d
fn input_valid(normalized_: &str) -> bool {
    let regex = build_full_regex();
    regex.is_match(normalized_)
}


#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Unit {
    Base,
    Kilo,
    Mega,
    Giga,
    Kibi,
    Mibi,
    Gibi,
}

impl Unit {
    /// * `unit_string` the unit string and only the unit string in lowercase
    fn from_str(unit_string: &str) -> Unit {
        match unit_string {
            // attention! must match our regex!
            "k" | "kb" => { Unit::Kilo },
            "m" | "mb" => { Unit::Mega },
            "g" | "gb" => { Unit::Giga },
            "ki" | "kib" => { Unit::Kibi },
            "mi" | "mib" => { Unit::Mibi },
            "gi" | "gib" => { Unit::Gibi },
            _ => { Unit::Base },
        }
    }

    fn value_to_base(value: usize, from_unit: Unit) -> usize {
        match from_unit {
            Unit::Base => { value }
            Unit::Kilo => { value * 1E3 as usize }
            Unit::Mega => { value * 1E6 as usize }
            Unit::Giga => { value * 1E9 as usize }
            Unit::Kibi => { value * 1024_usize }
            Unit::Mibi => { value * 1024_usize.pow(2) }
            Unit::Gibi => { value * 1024_usize.pow(3) }
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_input_valid() {
        assert!(input_valid("12234"));
        assert!(input_valid("12234mb"));
        assert!(input_valid("12234gib"));
        assert!(input_valid("0x2000"));
        assert!(input_valid("0b1000000011110000"));
        assert!(input_valid("0x6020kb"));

        assert!(!input_valid("12234gib124"));
    }

    #[test]
    fn test_numeral_system_from_input() {
        assert_eq!(NumeralSystem::Decimal, NumeralSystem::from_input("11314"));
        assert_eq!(NumeralSystem::Decimal, NumeralSystem::from_input("11314kb"));
        assert_eq!(NumeralSystem::Hex, NumeralSystem::from_input("0x2000"));
        assert_eq!(NumeralSystem::Bin, NumeralSystem::from_input("0b00101"));
    }

    #[test]
    fn test_parse_input() {
        parse_input("1234").expect("should work");
        parse_input("1234kb").expect("should work");
        parse_input("0x2000_4000_mb").expect("should work");
        let parsed = parse_input("0x2000_4000_mib").expect("should work");
        assert_eq!(parsed.unit, Unit::Mibi);
        assert_eq!(parsed.numeral_sysem, NumeralSystem::Hex);
    }

    #[test]
    fn test_unit_to_base() {
        assert_eq!(Unit::value_to_base(1, Unit::Kibi), 1024)
    }
}