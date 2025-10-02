/*
MIT License

Copyright (c) 2024 Philipp Schuster

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

//! **Wambo** - Helper tool for converting decimal/bin/oct/hex + interpreting
//! data as i8-i64, u8-u64, and f32/f64.

#![deny(clippy::all, clippy::cargo, clippy::nursery)]
#![allow(clippy::multiple_crate_versions)]
// required because the produced code by `derive_more` doesn't follow this rule
#![allow(clippy::use_self)]
#![deny(missing_docs)]
#![deny(missing_debug_implementations)]
#![deny(rustdoc::all)]

mod layout;
mod parse;
mod print;

use crate::parse::{ParsedUserInput, parse_input};
use layout::*;
use std::process::exit;
use std::sync::atomic::AtomicBool;

/// Turned to true if SIGINT or SIGTERM are received. Supports a graceful shutdown.
pub static SIGNAL_STOP: AtomicBool = AtomicBool::new(false);

/// **Wambo** is a binary and so far no library.
/// It's an all-in-one binary to convert decimal/bin/oct/hex + interpret data as i8-i64, u8-u64,
/// and f32/f64.
fn main() {
    let parsed = validate_args_and_parse_input();

    let mut tui = tui_prepare().unwrap();
    run_tui(&mut tui, &parsed).unwrap();
    tui_cleanup(tui).unwrap();
}

/// Validates the user input and parses it. Terminates the program, if the args are invalid.
fn validate_args_and_parse_input() -> ParsedUserInput {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() == 1 {
        eprintln!(
            "Please provide an integer input in decimal, hex (0x), octal (0o), or bin (0b) format!"
        );
        eprintln!("Enter -h for help.");
        exit(-1);
    }
    let input = &args[1];
    if input == "-h" || input == "--help" {
        show_help();
        exit(0);
    }

    let parsed = parse_input(input);
    match parsed {
        Ok(parsed) => parsed,
        Err(e) => {
            eprintln!("Illegal input: {}", e);
            exit(-1);
        }
    }
}

fn show_help() {
    println!("Wambo - Decimal, Hex, Bin number + byte converter");
    println!("Wambo can easily show you information about a value in decimal, hex, binary and");
    println!("different byte size representations.");
    println!();
    println!("Usage:");
    println!("  Input value can be a number in binary, octal, decimal or hexadecimal.");
    println!("  Negative numbers are supported. Only integers, no decimals.");
    println!("    $ wambo 42");
    println!("    $ wambo 0b10001111");
    println!("    $ wambo 0xdeadbeef");
    println!();
    println!("  Input value can be seen as number or as byte, whatever way you prefer.");
    println!("    $ wambo 42    => \"I just want to convert this number\"");
    println!("    $ wambo 1024b => \"I want to know how many megabytes 1024 bytes are\"");
    println!();
    println!("  Input values can have underscores for better readability.");
    println!("    $ wambo 1_000_000");
    println!("    $ wambo 0b1000_1111");
    println!("    $ wambo 0xde_ad_be_ef");
    println!();
    println!("  Input values can have a unit.");
    println!("    $ wambo 1mib (Mibibyte)");
    println!("    Valid units are: k/kb, m/mb, g/gb, t/tb");
    println!("                     ki/kib, mi/mib, gi/gib, ti/tib");
}
