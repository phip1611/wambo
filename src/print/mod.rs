use crate::parse::unit::Unit;
use crate::parse::Parsed;
use crossterm::style::{Attribute, Color, Print, ResetColor, SetAttribute, SetForegroundColor};
use crossterm::tty::IsTty;
use crossterm::ExecutableCommand;
use derive_more::Display;
use fraction_list_fmt_align::{fmt_align_fractions, FractionNumber};
use std::io::stdout;

const MAX_PRECISION: u8 = 20;

pub fn build_output_groups(parsed: Parsed) -> Vec<OutputGroup> {
    vec![
        build_numeral_systems_og(&parsed),
        build_bits_og(&parsed),
        build_un_and_signed_integers_og(&parsed),
        build_ieee754_og(&parsed),
        build_bytes(&parsed),
        build_ibi_bytes(&parsed),
    ]
}

fn build_numeral_systems_og(parsed: &Parsed) -> OutputGroup {
    OutputGroup {
        title: Interpretation::NumeralSystems,
        value_alignment: ValueAlignment::Left,
        interpretations: vec![
            OutputLine {
                key: "Decimal".to_string(),
                value: format!("{}", parsed.value()),
            },
            OutputLine {
                key: "Binary".to_string(),
                value: format!("{:b}", parsed.value()),
            },
            OutputLine {
                key: "Octal".to_string(),
                value: format!("{:o}", parsed.value()),
            },
            OutputLine {
                key: "Hexadecimal".to_string(),
                value: format!("{:x}", parsed.value()),
            },
        ],
    }
}

fn build_bits_og(parsed: &Parsed) -> OutputGroup {
    OutputGroup {
        title: Interpretation::Bit64BigEndian,
        value_alignment: ValueAlignment::Left,
        interpretations: vec![
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

fn build_un_and_signed_integers_og(parsed: &Parsed) -> OutputGroup {
    OutputGroup {
        title: Interpretation::UnAndSignedIntegers,
        value_alignment: ValueAlignment::Right,
        interpretations: vec![
            OutputLine {
                key: " i8".to_string(),
                value: format!("{}", parsed.value() as i8),
            },
            OutputLine {
                key: " u8".to_string(),
                value: format!("{}", parsed.value() as u8),
            },
            OutputLine {
                key: "i16".to_string(),
                value: format!("{}", parsed.value() as i16),
            },
            OutputLine {
                key: "u16".to_string(),
                value: format!("{}", parsed.value() as u16),
            },
            OutputLine {
                key: "i32".to_string(),
                value: format!("{}", parsed.value() as i32),
            },
            OutputLine {
                key: "u32".to_string(),
                value: format!("{}", parsed.value() as u32),
            },
            OutputLine {
                key: "i64".to_string(),
                value: format!("{}", parsed.value() as i64),
            },
            OutputLine {
                key: "u64".to_string(),
                value: format!("{}", parsed.value() as u64),
            },
        ],
    }
}

fn build_ieee754_og(parsed: &Parsed) -> OutputGroup {
    // maximum 15 digits fractional precision
    // also rounds the number at the 15'th place/digit
    let f32_num = f32::from_ne_bytes((parsed.value() as i32).to_ne_bytes());
    let f64_num = f64::from_ne_bytes(parsed.value().to_ne_bytes());
    let fmt_vec = fmt_align_fractions(
        &[FractionNumber::F32(f32_num), FractionNumber::F64(f64_num)],
        MAX_PRECISION as u8,
    );
    OutputGroup {
        title: Interpretation::IEEE754,
        // not important here if left or right because the formatting
        // utility already makes sure that all values are same length (via spaces)
        value_alignment: ValueAlignment::Left,
        interpretations: vec![
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

fn build_bytes(parsed: &Parsed) -> OutputGroup {
    let base_value_f64 = parsed.unit().value_to_base_f64(parsed.value() as f64);

    let fmt_vec = fmt_align_fractions(
        &[
            FractionNumber::F64(base_value_f64),
            FractionNumber::F64(Unit::base_to_target(Unit::Kilo, base_value_f64)),
            FractionNumber::F64(Unit::base_to_target(Unit::Mega, base_value_f64)),
            FractionNumber::F64(Unit::base_to_target(Unit::Giga, base_value_f64)),
        ],
        MAX_PRECISION,
    );
    OutputGroup {
        title: Interpretation::Bytes,
        // not important here if left or right because the formatting
        // utility already makes sure that all values are same length (via spaces)
        value_alignment: ValueAlignment::Left,
        interpretations: vec![
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
        ],
    }
}

fn build_ibi_bytes(parsed: &Parsed) -> OutputGroup {
    let base_value_f64 = parsed.unit().value_to_base_f64(parsed.value() as f64);
    let fmt_vec = fmt_align_fractions(
        &[
            FractionNumber::F64(base_value_f64),
            FractionNumber::F64(Unit::base_to_target(Unit::Kibi, base_value_f64)),
            FractionNumber::F64(Unit::base_to_target(Unit::Mibi, base_value_f64)),
            FractionNumber::F64(Unit::base_to_target(Unit::Gibi, base_value_f64)),
        ],
        MAX_PRECISION,
    );
    OutputGroup {
        title: Interpretation::Ibibytes,
        // because they are already aligned Left and not right
        value_alignment: ValueAlignment::Left,
        interpretations: vec![
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
        ],
    }
}

/// Describes the kind of an output group that is dedicated to
/// a specific class of interpretations.
#[derive(Debug, Display, Copy, Clone)]
pub enum Interpretation {
    #[display(fmt = "Different numeral systems.")]
    NumeralSystems,
    #[display(fmt = "64bit in memory (big endian byte representation).")]
    Bit64BigEndian,
    #[display(fmt = "Several signed and unsigned integers (decimal).")]
    UnAndSignedIntegers,
    #[display(fmt = "Integer bits as IEEE754 (floating point numbers/fractions).")]
    IEEE754,
    #[display(fmt = "File size in bytes (factor 1000) (using f64).")]
    Bytes,
    #[display(fmt = "File size in *ibibytes (factor 1024) (using f64).")]
    Ibibytes,
}

/// Alignment of the value against the other values
/// of the same group.
#[derive(Debug, PartialEq)]
enum ValueAlignment {
    /// align like this:
    /// ```
    /// 123
    /// 1
    /// 1234
    /// ```
    Left,
    /// align like this:
    /// ```
    ///  123
    ///    1
    /// 1234
    /// ```
    Right,
}

#[derive(Debug)]
pub struct OutputGroup {
    title: Interpretation,
    interpretations: Vec<OutputLine>,
    value_alignment: ValueAlignment,
}

impl OutputGroup {
    pub fn find_longest_value_string(&self) -> usize {
        self.interpretations
            .iter()
            .map(|i| i.value().len())
            .max()
            .unwrap()
    }

    pub fn find_longest_key_string(&self) -> usize {
        self.interpretations
            .iter()
            .map(|i| i.key().len())
            .max()
            .unwrap()
    }

    #[allow(dead_code)]
    pub fn title(&self) -> Interpretation {
        self.title
    }

    #[allow(dead_code)]
    pub fn interpretations(&self) -> &Vec<OutputLine> {
        &self.interpretations
    }

    pub fn pretty_print(&self) {
        // print heading
        self.print_heading();

        let longest_key = self.find_longest_key_string();
        let longest_value = self.find_longest_value_string();

        for i in &self.interpretations {
            let additional_left_spaces = if self.value_alignment == ValueAlignment::Left {
                longest_key - i.key.len()
            } else {
                longest_key - i.key.len() + longest_value - i.value.len()
            };

            // print key in color
            self.print_key(i.key());
            // print!("{}:  ", i.key);
            for _ in 0..additional_left_spaces {
                print!(" ")
            }
            // print value in color
            self.print_value(i.value());
            println!(); //print \n
        }
    }

    /// Prints the heading with the leading newline.
    fn print_heading(&self) {
        // in "IntelliJ/Clion > run" this is false
        let is_tty = stdout().is_tty();
        let fmt = format!("### Interpreted as: {} ###", self.title).to_uppercase();
        if is_tty {
            stdout()
                .execute(SetAttribute(Attribute::Bold))
                .unwrap()
                .execute(SetForegroundColor(Color::Blue))
                .unwrap()
                .execute(Print(fmt))
                .unwrap()
                .execute(ResetColor)
                .unwrap()
                .execute(SetAttribute(Attribute::Reset))
                .unwrap();
        } else {
            print!("{}", fmt)
        }
        println!(); // newline
    }

    /// Prints the key without newline at the end.
    fn print_key(&self, key: &str) {
        // in "IntelliJ/Clion > run" this is false
        let is_tty = stdout().is_tty();
        let key_fmt = format!("{}:  ", key);
        if is_tty {
            stdout()
                .execute(SetAttribute(Attribute::Bold))
                .unwrap()
                .execute(SetForegroundColor(Color::Red))
                .unwrap()
                .execute(Print(key_fmt))
                .unwrap()
                .execute(ResetColor)
                .unwrap()
                .execute(SetAttribute(Attribute::Reset))
                .unwrap();
        } else {
            print!("{}", key_fmt);
        }
    }

    /// Prints the value without newline at the end.
    fn print_value(&self, value: &str) {
        // in "IntelliJ/Clion > run" this is false
        let is_tty = stdout().is_tty();
        let value_fmt = value.to_string();
        if is_tty {
            stdout()
                // .execute(SetAttribute(Attribute::Bold)).unwrap()
                .execute(SetForegroundColor(Color::Green))
                .unwrap()
                .execute(Print(value_fmt))
                .unwrap()
                .execute(ResetColor)
                .unwrap();
            // .execute(SetAttribute(Attribute::Reset)).unwrap();
        } else {
            print!("{}", value_fmt);
        }
    }
}

#[derive(Debug)]
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
        .into_iter()
        .skip(1) // skip first item
        .collect::<String>()
}
