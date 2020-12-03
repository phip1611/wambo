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
mod parse;

use std::process::exit;
use crate::parse::parse_input;


fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() == 1 {
        eprintln!("Please provide an integer input in decimal, hex (0x), or bin (0b) format!");
        eprintln!("Enter -h for help.");
        exit(-1);
    }
    let input = &args[1];
    if input == "-h" || input == "--help" {
        show_help();
        exit(0);
    }

    let number = parse_input(input);
    if let Err(e) = number {
        eprintln!("Illegal input: {}", e);
        exit(-1);
    }
    let parsed = number.unwrap();
    let number = parsed.actual_value();

    let number_f64 = number as f64;
    println!("interpreting input as: unsigned integer");
    println!("decimal: {}", number);
    println!("hex    : {}", format_hex(number));
    println!("bin    : {}", format_bin(number));
    println!();
    println!("interpreting input as: several signed data types");
    println!(" i8 (decimal): {}", number as i8);
    println!("i16 (decimal): {}", number as i16);
    println!("i32 (decimal): {}", number as i32);
    println!("i64 (decimal): {}", number as i64);
    println!("f32 (decimal): {:.7} (bits interpreted as IEEE-754)", f32::from_ne_bytes((number as i32).to_ne_bytes()));
    println!("f64 (decimal): {:.7} (bits interpreted as IEEE-754)", f64::from_ne_bytes(number.to_ne_bytes()));
    println!();
    println!("interpreting input as: file sizes / number of bytes");
    println!(" B     : {}", number);
    println!("KB     : {:.7}", number_f64 / 1E3);
    println!("MB     : {:.7}", number_f64 / 1E6);
    println!("GB     : {:.7}", number_f64 / 1E9);
    println!();
    // TODO is base the right word?!
    println!("interpreting input as: *ibi-bytes (1024 (=multiple of 2) as base instead of 1000)");
    println!("KiB    : {:.7}", number_f64 / 1024_f64);
    println!("MiB    : {:.7}", number_f64 / 1024_f64.powf(2_f64));
    println!("GiB    : {:.7}", number_f64 / 1024_f64.powf(3_f64));
}




fn format_hex(number: usize) -> String {
    let string_fixed_len = format!("{:016x}", number);
    let formatted = format_num_add_delimiters(&string_fixed_len, 2);
    format!("0x{} (64bit)", formatted)
}

fn format_bin(number: usize) -> String {
    let string_fixed_len = format!("{:064b}", number);
    let formatted = format_num_add_delimiters(&string_fixed_len, 8);
    format!("0b{} (64bit)", formatted)
}

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

fn show_help() {
    println!("Wambo - Decimal, Hex, Bin number + byte converter");
    println!("Wambo can easily show you information about a value in decimal, hex, binary and");
    println!("different byte size representations.");
    println!();
    println!("Usage:");
    println!("  Input values can be binary, decimal or hexadecimal.");
    println!("    $ wambo 42");
    println!("    $ wambo 0b10001111");
    println!("    $ wambo 0xdeadbeef");
    println!("  Input values can be seen as number or as byte, whatever way you prefer.");
    println!("    $ wambo 42    => \"I just want to convert this number\"");
    println!("    $ wambo 1024b => \"I want to know how many megabytes 1024 bytes are\"");
    println!("  Input values can have underscores for better readability.");
    println!("    $ wambo 1_000_000");
    println!("    $ wambo 0b1000_1111");
    println!("    $ wambo 0xde_ad_be_ef");
    println!("  Input values can have a unit.");
    println!("    $ wambo 1mib (Mibibyte)");
    println!("    Valid units are: k/kb, m/mb, g/gb");
    println!("                     ki/kib, mi/mib, gi/gib");
}
