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

//! Parsing code for the numeral system.

use derive_more::Display;

/// Numeral system.
#[derive(Debug, PartialEq, Eq, Copy, Clone, Display)]
pub enum NumeralSystem {
    /// Base of 2 (0-1).
    #[display(fmt = "Bin (0b)")]
    Bin,
    /// Base of 8 (0-7).
    #[display(fmt = "Oct (0o)")]
    Octal,
    /// Base of 10 (0-9).
    #[display(fmt = "Dec")]
    Decimal,
    /// Base of 16 (0-F).
    #[display(fmt = "Hex (0x)")]
    Hex,
}

impl NumeralSystem {
    /// Parses the [`NumeralSystem`] from the normalized and validated slice of the input
    /// that corresponds to this type.
    /// * `part_str` slice of normalized and validated user input that corresponds to this type
    pub fn from_input(part_str: &str) -> Self {
        if part_str.starts_with("0b") {
            Self::Bin
        } else if part_str.starts_with("0o") {
            Self::Octal
        } else if part_str.starts_with("0x") {
            Self::Hex
        } else {
            Self::Decimal
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_parse_numeral_system() {
        // we test only normalized inputs here
        assert_eq!(
            NumeralSystem::Bin,
            NumeralSystem::from_input("0b1"),
            "Must be NumeralSystem::Bin"
        );
        assert_eq!(
            NumeralSystem::Octal,
            NumeralSystem::from_input("0o1"),
            "Must be NumeralSystem::Octal"
        );
        assert_eq!(
            NumeralSystem::Decimal,
            NumeralSystem::from_input("12345"),
            "Must be NumeralSystem::Decimal"
        );
        assert_eq!(
            NumeralSystem::Hex,
            NumeralSystem::from_input("0x1"),
            "Must be NumeralSystem::Hex"
        );
    }
}
