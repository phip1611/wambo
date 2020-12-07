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
    #[display(fmt = "Kibibyte")]
    Kibi,
    #[display(fmt = "Mibibyte")]
    Mibi,
    #[display(fmt = "Gibibyte")]
    Gibi,
}

impl Unit {

    /// Parses the [`NumeralSystem`] from the normalized and validated slice of the input
    /// that corresponds to this type.
    /// * `part_str` slice of normalized and validated user input that corresponds to this type
    pub fn from_input(normalized_input: &str) -> Unit {
        match normalized_input {
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

    /// Transforms a value in a specific value into the base unit.
    pub fn value_to_base(&self, value: usize) -> usize {
        match self {
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
    fn test_parse_unit() {
        // only lowercase here
        // because user input gets transformed to lowercase
        assert_eq!(Unit::Base, Unit::from_input(""), "Must be Unit::Base");
        assert_eq!(Unit::Kilo, Unit::from_input("k"), "Must be Unit::Kilo");
        assert_eq!(Unit::Kibi, Unit::from_input("ki"), "Must be Unit::Kibi");
        assert_eq!(Unit::Kibi, Unit::from_input("kib"), "Must be Unit::Kibi");
        assert_eq!(Unit::Mega, Unit::from_input("m"), "Must be Unit::Mega");
        assert_eq!(Unit::Giga, Unit::from_input("gb"), "Must be Unit::Giga");
    }

}
