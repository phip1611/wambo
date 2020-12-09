use derive_more::Display;
use crate::parse::Parsed;
use std::fmt::Display;
use regex::Regex;
use crate::print::fraction_fmt::fmt_align_fraction_strings;

mod fraction_fmt;

pub fn build_output_groups(parsed: Parsed) -> Vec<OutputGroup> {
    vec![
        build_numeral_systems_og(&parsed),
        build_bits_og(&parsed),
        build_un_and_signed_integers_og(&parsed),
        build_ieee754_og(&parsed),
    ]
}

fn build_numeral_systems_og(parsed: &Parsed) -> OutputGroup {
    OutputGroup {
        title: Interpretation::NumeralSystems,
        value_alignment: ValueAlignment::Left,
        interpretations: vec![
            OutputLine {
                key: "Decimal".to_string(),
                value: format!("{}", parsed.value())
            },
            OutputLine {
                key: "Binary".to_string(),
                value: format!("{:b}", parsed.value())
            },
            OutputLine {
                key: "Octal".to_string(),
                value: format!("{:o}", parsed.value())
            },
            OutputLine {
                key: "Hexadecimal".to_string(),
                value: format!("{:x}", parsed.value())
            },
        ]
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
        ]
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
        ]
    }
}

fn build_ieee754_og(parsed: &Parsed) -> OutputGroup {
    const PRECISION: usize = 20;
    // maximum 15 digits fractional precision
    // also rounds the number at the 15'th place/digit
    let f32_num = f32::from_ne_bytes((parsed.value() as i32).to_ne_bytes());
    let f64_num = f64::from_ne_bytes(parsed.value().to_ne_bytes());
    let f32_rust_fmt = format!("{:.1$}", f32_num, PRECISION);
    let f64_rust_fmt = format!("{:.1$}", f64_num, PRECISION);
    let (f32_fmt , f64_fmt) = fmt_align_fraction_strings(&f32_rust_fmt, &f64_rust_fmt);
    OutputGroup {
        title: Interpretation::IEEE754,
        value_alignment: ValueAlignment::Right,
        interpretations: vec![
            OutputLine {
                key: " f32".to_string(),
                // value: format!("'{}'", f32_fmt),
                value: f32_fmt,
            },
            OutputLine {
                key: " f64".to_string(),
                // value: format!("'{}'", f64_fmt),
                value: f64_fmt,
            },
        ]
    }
}


#[derive(Debug, Display, Copy, Clone)]
enum Interpretation {
    #[display(fmt = "Different numeral systems.")]
    NumeralSystems,
    #[display(fmt = "64bit in memory (big endian byte representation).")]
    Bit64BigEndian,
    #[display(fmt = "Several signed and unsigned integers (decimal).")]
    UnAndSignedIntegers,
    #[display(fmt = "Integer bits as IEEE754 (floating point numbers/fractions).")]
    IEEE754,
    #[display(fmt = "File size in bytes (factor 1000).")]
    Bytes,
    #[display(fmt = "File size in *ibibytes (factor 1024).")]
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
        self.interpretations.iter()
            .map(|i| i.value().len())
            .max().unwrap()
    }

    pub fn find_longest_key_string(&self) -> usize {
        self.interpretations.iter()
            .map(|i| i.key().len())
            .max().unwrap()
    }

    pub fn title(&self) -> Interpretation {
        self.title
    }
    pub fn interpretations(&self) -> &Vec<OutputLine> {
        &self.interpretations
    }

    pub fn pretty_print(&self) {
        // print heading
        let fmt = format!("### Interpreted as: {} ###", self.title)
            .to_uppercase();
        println!("{}", fmt);

        let longest_key = self.find_longest_key_string();
        let longest_value = self.find_longest_value_string();

        // align like this:
        // 123
        // 1
        // 1234
        if self.value_alignment == ValueAlignment::Left {
            for i in &self.interpretations {
                let spaces = longest_key - i.key.len();
                print!("{}: ", i.key);
                for _ in 0..spaces { print!(" ") }
                print!("{}", i.value);
                println!(); //print \n
            }
        }
        // align like this:
        //  123
        //    1
        // 1234
        else {
            for i in &self.interpretations {
                let spaces = longest_key - i.key.len()
                    + longest_value - i.value.len();
                print!("{}: ", i.key);
                for _ in 0..spaces { print!(" ") }
                print!("{}", i.value);
                println!(); //print \n
            }
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
    let formatted_with_delimiters = chars.chunks(chunksize)
        .map(|chars| chars.iter().collect::<String>())
        .fold(String::new(), |combined: String, group|
            format!("{}_{}", combined, group),
        );

    // transform _00000000_00000000 to 00000000_00000000 (remove leading underscore)
    formatted_with_delimiters.chars().into_iter()
        .skip(1) // skip first item
        .collect::<String>()
}
