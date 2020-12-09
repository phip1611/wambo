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
//! TODO

mod parse;
mod print;

use std::process::exit;
use crate::parse::parse_input;
use crate::print::build_output_groups;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() == 1 {
        eprintln!("Please provide an integer input in decimal, hex (0x), octal (0o), or bin (0b) format!");
        eprintln!("Enter -h for help.");
        exit(-1);
    }
    let input = &args[1];
    if input == "-h" || input == "--help" {
        show_help();
        exit(0);
    }

    let parsed = parse_input(input);
    if let Err(e) = parsed {
        eprintln!("Illegal input: {}", e);
        exit(-1);
    }
    let parsed = parsed.unwrap();

    let ogs = build_output_groups(parsed);
    for og in ogs {
        og.pretty_print();
        println!(); // one line to separate the output groups
    }
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
