use crate::parse::unit::Unit;
use crate::parse::{NumeralSystem, ParsedUserInput};
use derive_more::Display;
use fraction_list_fmt_align::{fmt_align_fractions, FormatPrecision, FractionNumber};

const MAX_PRECISION: u8 = 4;

pub fn get_output_group(parsed: &ParsedUserInput, representation: Interpretation) -> OutputGroup {
    match representation {
        Interpretation::NumeralSystems => build_numeral_systems_og(parsed),
        Interpretation::Bit64BigEndian => build_bits_og(parsed),
        Interpretation::SignedIntegers => build_signed_integers_og(parsed),
        Interpretation::UnsignedIntegers => build_unsigned_integers_og(parsed),
        Interpretation::IEEE754 => build_ieee754_og(parsed),
        Interpretation::Bytes => build_bytes_og(parsed),
        Interpretation::Ibibytes => build_ibi_bytes_og(parsed),
    }
}

fn build_numeral_systems_og(parsed: &ParsedUserInput) -> OutputGroup {
    OutputGroup {
        title: Interpretation::NumeralSystems,
        value_alignment: ValueAlignment::Right,
        lines: vec![
            OutputLine {
                key: format!("{}", NumeralSystem::Decimal),
                value: format!("{}{}", parsed.sign(), parsed.value()),
            },
            OutputLine {
                key: format!("{}", NumeralSystem::Bin),
                value: format!("{}{:b}", parsed.sign(), parsed.value()),
            },
            OutputLine {
                key: format!("{}", NumeralSystem::Octal),
                value: format!("{}{:o}", parsed.sign(), parsed.value()),
            },
            OutputLine {
                key: format!("{}", NumeralSystem::Hex),
                value: format!("{}{:x}", parsed.sign(), parsed.value()),
            },
        ],
    }
}

fn build_bits_og(parsed: &ParsedUserInput) -> OutputGroup {
    OutputGroup {
        title: Interpretation::Bit64BigEndian,
        value_alignment: ValueAlignment::Left,
        lines: vec![
            OutputLine {
                key: "Bin (Rust-style)".to_string(),
                value: format!("0b{}", format_64bit_bin_rust_style(parsed.value())),
            },
            OutputLine {
                key: "Bin (C-style)".to_string(),
                value: format!("0b{:064b}", parsed.value()),
            },
            OutputLine {
                key: "Hex".to_string(),
                value: format!("0x{:016x}", parsed.value()),
            },
        ],
    }
}

fn build_signed_integers_og(parsed: &ParsedUserInput) -> OutputGroup {
    OutputGroup {
        title: Interpretation::SignedIntegers,
        value_alignment: ValueAlignment::Right,
        lines: vec![
            OutputLine {
                key: " i8".to_string(),
                value: format!("{}{}", parsed.sign(), parsed.value() as i8),
            },
            OutputLine {
                key: "i16".to_string(),
                value: format!("{}{}", parsed.sign(), parsed.value() as i16),
            },
            OutputLine {
                key: "i32".to_string(),
                value: format!("{}{}", parsed.sign(), parsed.value() as i32),
            },
            OutputLine {
                key: "i64".to_string(),
                value: format!("{}{}", parsed.sign(), parsed.value() as i64),
            },
        ],
    }
}

fn build_unsigned_integers_og(parsed: &ParsedUserInput) -> OutputGroup {
    OutputGroup {
        title: Interpretation::UnsignedIntegers,
        value_alignment: ValueAlignment::Right,
        lines: vec![
            OutputLine {
                key: " u8".to_string(),
                value: format!("{}", parsed.value() as u8),
            },
            OutputLine {
                key: "u16".to_string(),
                value: format!("{}", parsed.value() as u16),
            },
            OutputLine {
                key: "u32".to_string(),
                value: format!("{}", parsed.value() as u32),
            },
            OutputLine {
                key: "u64".to_string(),
                value: format!("{}", parsed.value()),
            },
        ],
    }
}

fn build_ieee754_og(parsed: &ParsedUserInput) -> OutputGroup {
    // maximum 15 digits fractional precision
    // also rounds the number at the 15'th place/digit
    let f32_num = f32::from_ne_bytes((parsed.value() as i32).to_ne_bytes());
    let f64_num = f64::from_ne_bytes(parsed.value().to_ne_bytes());
    let fmt_vec = fmt_align_fractions(
        &[FractionNumber::F32(f32_num), FractionNumber::F64(f64_num)],
        FormatPrecision::Max(MAX_PRECISION),
    );
    OutputGroup {
        title: Interpretation::IEEE754,
        // not important here if left or right because the formatting
        // utility already makes sure that all values are same length (via spaces)
        value_alignment: ValueAlignment::Left,
        lines: vec![
            OutputLine {
                key: "f32".to_string(),
                // value: format!("'{}'", f32_fmt),
                value: fmt_vec[0].as_str().to_string(),
            },
            OutputLine {
                key: "f64".to_string(),
                // value: format!("'{}'", f64_fmt),
                value: fmt_vec[1].as_str().to_string(),
            },
        ],
    }
}

fn build_bytes_og(parsed: &ParsedUserInput) -> OutputGroup {
    let base_value_f64 = parsed.value() as f64;
    let fmt_vec = fmt_align_fractions(
        &[
            FractionNumber::F64(base_value_f64),
            FractionNumber::F64(Unit::base_to_target(Unit::Kilo, base_value_f64)),
            FractionNumber::F64(Unit::base_to_target(Unit::Mega, base_value_f64)),
            FractionNumber::F64(Unit::base_to_target(Unit::Giga, base_value_f64)),
            FractionNumber::F64(Unit::base_to_target(Unit::Tera, base_value_f64)),
        ],
        FormatPrecision::Max(MAX_PRECISION),
    );
    OutputGroup {
        title: Interpretation::Bytes,
        // not important here if left or right because the formatting
        // utility already makes sure that all values are same length (via spaces)
        value_alignment: ValueAlignment::Left,
        lines: vec![
            OutputLine {
                key: " B".to_string(),
                value: fmt_vec[0].to_string(),
            },
            OutputLine {
                key: "KB".to_string(),
                value: fmt_vec[1].to_string(),
            },
            OutputLine {
                key: "MB".to_string(),
                value: fmt_vec[2].to_string(),
            },
            OutputLine {
                key: "GB".to_string(),
                value: fmt_vec[3].to_string(),
            },
            OutputLine {
                key: "TB".to_string(),
                value: fmt_vec[4].to_string(),
            },
        ],
    }
}

fn build_ibi_bytes_og(parsed: &ParsedUserInput) -> OutputGroup {
    let base_value_f64 = parsed.value() as f64;
    let fmt_vec = fmt_align_fractions(
        &[
            FractionNumber::F64(base_value_f64),
            FractionNumber::F64(Unit::base_to_target(Unit::Kibi, base_value_f64)),
            FractionNumber::F64(Unit::base_to_target(Unit::Mibi, base_value_f64)),
            FractionNumber::F64(Unit::base_to_target(Unit::Gibi, base_value_f64)),
            FractionNumber::F64(Unit::base_to_target(Unit::Tebi, base_value_f64)),
        ],
        FormatPrecision::Max(MAX_PRECISION),
    );
    OutputGroup {
        title: Interpretation::Ibibytes,
        // because they are already aligned Left and not right
        value_alignment: ValueAlignment::Left,
        lines: vec![
            OutputLine {
                key: " iB".to_string(),
                value: fmt_vec[0].to_string(),
            },
            OutputLine {
                key: "KiB".to_string(),
                value: fmt_vec[1].to_string(),
            },
            OutputLine {
                key: "MiB".to_string(),
                value: fmt_vec[2].to_string(),
            },
            OutputLine {
                key: "GiB".to_string(),
                value: fmt_vec[3].to_string(),
            },
            OutputLine {
                key: "TiB".to_string(),
                value: fmt_vec[4].to_string(),
            },
        ],
    }
}

/// Describes the kind of an output group that is dedicated to
/// a specific class of interpretations.
#[derive(Debug, Display, Copy, Clone)]
pub enum Interpretation {
    #[display("Numeral Systems")]
    NumeralSystems,
    #[display("64 bit (Big Endian)")]
    Bit64BigEndian,
    #[display("Signed Integers")]
    SignedIntegers,
    #[display("Unsigned Integers")]
    UnsignedIntegers,
    #[display("Integer Bits as IEEE-754")]
    IEEE754,
    #[display("Size in Bytes")]
    Bytes,
    #[display("Size in *ebi/*ibi Bytes")]
    Ibibytes,
}

/// Alignment of the value against the other values
/// of the same group.
#[derive(Clone, Copy, Debug, PartialEq)]
enum ValueAlignment {
    /// align like this:
    /// ```ignore
    /// 123
    /// 1
    /// 1234
    /// ```
    Left,
    /// align like this:
    /// ```ignore
    ///  123
    ///    1
    /// 1234
    /// ```
    Right,
}

/// An output group describes all information to ASCII-print multiple
/// values of a certain representation on a line-by-line base. It has
/// all information so that multiple lines can be aligned.
#[derive(Clone, Debug)]
pub struct OutputGroup {
    title: Interpretation,
    lines: Vec<OutputLine>,
    value_alignment: ValueAlignment,
}

impl OutputGroup {
    fn find_longest_value_string(&self) -> usize {
        self.lines.iter().map(|i| i.value().len()).max().unwrap()
    }

    fn find_longest_key_string(&self) -> usize {
        self.lines.iter().map(|i| i.key().len()).max().unwrap()
    }

    pub const fn title(&self) -> Interpretation {
        self.title
    }

    /// Returns an iterator of type [`OutputGroupIterator`].
    pub const fn iter(&self) -> OutputGroupIterator<'_> {
        OutputGroupIterator::new(self)
    }
}

/// Iterator over the lines of an [`OutputGroup`].
/// Returns pairs of `(key: String, value: String)` but
/// each value is padded with enough spaces for a proper alignment.
/// There is no space between the key and the value. It is up to
/// API users to add `": "` or similar after each key.
pub struct OutputGroupIterator<'a> {
    og: &'a OutputGroup,
    line: usize,
}

impl<'a> OutputGroupIterator<'a> {
    const fn new(og: &'a OutputGroup) -> Self {
        OutputGroupIterator { og, line: 0 }
    }
}

impl Iterator for OutputGroupIterator<'_> {
    type Item = (String, String);

    fn next(&mut self) -> Option<Self::Item> {
        if self.line >= self.og.lines.len() {
            None
        } else {
            let longest_key = self.og.find_longest_key_string();
            let longest_value = self.og.find_longest_value_string();

            let line = &self.og.lines[self.line];

            // additional spaces between key and value
            let additional_spaces = if self.og.value_alignment == ValueAlignment::Left {
                longest_key - line.key.len()
            } else {
                longest_key - line.key.len() + longest_value - line.value.len()
            };

            let value = format!(
                "{additional_spaces}{value}",
                additional_spaces = " ".repeat(additional_spaces),
                value = line.value,
            );

            self.line += 1;

            Some((line.key.clone(), value))
        }
    }
}

/// Bundles the key and the value for a line inside a [`OutputGroup`].
#[derive(Clone, Debug)]
pub struct OutputLine {
    key: String,
    value: String,
}

impl OutputLine {
    pub fn key(&self) -> &str {
        &self.key
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}

/// Transforms for example "1111000010101010" to "11110000_10101010" to
/// increase readability, but with a 64 bit integer.
fn format_64bit_bin_rust_style(number: u64) -> String {
    let string_fixed_len = format!("{:064b}", number);
    format_num_add_delimiters(&string_fixed_len, 8)
}

/// Transforms for example "1111000010101010" to "11110000_10101010" to
/// increase readability.
fn format_num_add_delimiters(digits: &str, chunksize: usize) -> String {
    let chars = digits.chars().collect::<Vec<char>>();
    assert_eq!(chars.len() % chunksize, 0);
    let formatted_with_delimiters = chars
        .chunks(chunksize)
        .map(|chars| chars.iter().collect::<String>())
        .fold(String::new(), |combined: String, group| {
            format!("{}_{}", combined, group)
        });

    // transform _00000000_00000000 to 00000000_00000000 (remove leading underscore)
    formatted_with_delimiters
        .chars()
        .skip(1) // skip first item
        .collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fold_og_iter_to_string(iter: OutputGroupIterator) -> String {
        iter.fold(String::new(), |mut base, (key, value)| {
            base.push_str(key.as_str());
            base.push_str(value.as_str());
            base.push('\n');
            base
        })
    }

    fn get_output_base_group() -> OutputGroup {
        OutputGroup {
            title: Interpretation::NumeralSystems,
            lines: vec![
                OutputLine {
                    key: "foo".to_string(),
                    value: "foobar".to_string(),
                },
                OutputLine {
                    key: "foo2".to_string(),
                    value: "foobar2".to_string(),
                },
            ],
            value_alignment: ValueAlignment::Left,
        }
    }

    #[test]
    fn test_output_group_iter_align_left() {
        let mut og = get_output_base_group();
        og.value_alignment = ValueAlignment::Left;

        let final_str = fold_og_iter_to_string(og.iter());

        // println!("{}", final_str);
        assert_eq!(
            // it is up to library users to add sth like ": " to the key
            "foo foobar\n\
             foo2foobar2\n\
            ",
            final_str
        );
    }

    #[test]
    fn test_output_group_iter_align_right() {
        let mut og = get_output_base_group();
        og.value_alignment = ValueAlignment::Right;

        let final_str = fold_og_iter_to_string(og.iter());

        // println!("{}", final_str);
        assert_eq!(
            // it is up to library users to add sth like ": " to the key
            "foo  foobar\n\
             foo2foobar2\n\
            ",
            final_str
        );
    }
}
