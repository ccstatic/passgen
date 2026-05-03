#![allow(unused)]

const LOWERCASE: &[u8] = b"abcdefghijklmnopqrstuvwxyz";
const UPPERCASE: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const NUMBERS: &[u8] = b"0123456789";
const SYMBOLS: &[u8] = b"!@#$%^&*()-_=+[]{};:,.<>?/";

const MAX_LENGTH: usize = 2048;
const MAX_AMOUNT: usize = 32;

use clap::{ArgAction, Parser};
#[derive(Parser)]
struct Args {
    // By default, a passsword is 16 characters long
    #[arg(short, long, default_value_t = 16)]
    length: usize,

    // By default, only one password is generated
    #[arg(short, long, default_value_t = 1)]
    amount: usize,

    // By default, passwords can have lowercase characters
    #[arg(long = "no-lowercase", action = ArgAction::SetFalse)]
    lowercase: bool,

    // By default, passwords can have uppercase characters
    #[arg(long = "no-uppercase", action = ArgAction::SetFalse)]
    uppercase: bool,

    // By default, passwords can have numbers
    #[arg(long = "no-numbers", action = ArgAction::SetFalse)]
    numbers: bool,

    // By default, passwords can have symbols
    #[arg(long = "no-symbols", action = ArgAction::SetFalse)]
    symbols: bool,
}

fn main() {
    let args = Args::parse();
    let mut charset = Vec::new();

    // Adds characters based on arguments
    if args.lowercase {
        charset.extend_from_slice(LOWERCASE);
    }
    if args.uppercase {
        charset.extend_from_slice(UPPERCASE);
    }
    if args.numbers {
        charset.extend_from_slice(NUMBERS);
    }
    if args.symbols {
        charset.extend_from_slice(SYMBOLS);
    }

    // Error if the user requests no character sets, which would be an impossible password generation
    assert!(!charset.is_empty(), "One character set has to be enabled!");

    // We don't want terminal-breaking amount of characters being pushed to the screen
    let length = args.length.clamp(2, MAX_LENGTH);
    let amount = args.amount.clamp(1, MAX_AMOUNT);

    // An RNG thread based off of unique seeds.
    // It uses system time + CPU cycles + number of inputs, meaning without a compromised system it's not possible to replicate naturally.
    use rand::rngs::OsRng;
    let mut rng = OsRng;

    use rand::seq::SliceRandom;
    for _ in 0..amount {
        // Creates the string and prints it
        let password: String = (0..length)
            .map(|_| *charset.choose(&mut rng).unwrap() as char)
            .collect();
        println!("{password}");
        // Beautifies the output if you wanted multiple passwords
        if amount > 1 {
            println!();
        }
    }

    // Stalls terminal so you can see your output
    use std::thread::sleep;
    use std::time::Duration;
    sleep(Duration::from_secs(30));

    // Clears the terminal in-case you forgot to close the program
    use std::process::Command;
    if cfg!(target_os = "windows") {
        // If on windows, clear screen specific way
        Command::new("cmd").args(["/C", "cls"]).status().unwrap();
    } else {
        // Other better systems allow this automatically
        Command::new("clear").status().unwrap();
    }
}
