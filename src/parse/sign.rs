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

//! Parsing code for the actual number/numeric value.

use derive_more::Display;

#[derive(Debug, PartialEq, Copy, Clone, Display)]
pub enum Sign {
    #[display(fmt = "")]
    Positive,
    #[display(fmt = "-")]
    Negative,
}

impl Sign {
    /// Parses the [`NumeralSystem`] from the normalized and validated slice of the input
    /// that corresponds to this type.
    /// * `part_str` slice of normalized and validated user input that corresponds to this type
    pub fn from_input(normalized_input: &str) -> Sign {
        if normalized_input == "-" {
            Sign::Negative
        } else {
            Sign::Positive
        }
    }

    /// Convenient function to check if the sign is positive.
    pub fn is_pos(&self) -> bool {
        match self {
            Sign::Positive => true,
            Sign::Negative => false,
        }
    }

    /// Convenient function to check if the sign is positive.
    #[allow(dead_code)]
    pub fn is_neg(&self) -> bool {
        !self.is_pos()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_parse_numeral_system() {
        // we test only normalized inputs here
        assert_eq!(
            Sign::Positive,
            Sign::from_input(""),
            "Must be Sign::Positive"
        );
        assert_eq!(
            Sign::Negative,
            Sign::from_input("-"),
            "Must be Sign::Negative"
        );
    }
}
