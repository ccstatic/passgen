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

    for _ in 0..amount {}

    // Keeps terminal alive so you can see the password output
    loop {}

    // use std::io;

    // // Constant character sets

    // // Secure default options
    // let mut length: usize = 16;
    // let mut amount: usize = 1;

    // let mut use_uppercase = true;
    // let mut use_lowercase = true;
    // let mut use_numbers = true;
    // let mut use_symbols = true;

    // println!("Enter password length:");

    // //gets input of first line! gets the number the user enters
    // let mut input = String::new();
    // io::stdin()
    //     .read_line(&mut input)
    //     .expect("Failed to read input");
    // let length: usize = input.trim().parse().expect("Please enter a valid number");

    // // Records/asks user if they want nums
    // println!("Do you want numbers? y/n");

    // let mut numbers_input = String::new();
    // io::stdin()
    //     .read_line(&mut numbers_input)
    //     .expect("Failed to read input");

    // // Records/asks user if they want symbols
    // println!("Do you want symbols? y/n");

    // let mut symbols_input = String::new();
    // io::stdin()
    //     .read_line(&mut symbols_input)
    //     .expect("Failed to read input");

    // if numbers_input.trim().to_lowercase() != "y" {
    //     use_numbers = false;
    // }
    // if symbols_input.trim().to_lowercase() != "y" {
    //     use_symbols = false;
    // }

    // // Builds set of characters based on their options
    // for (enabled, characters) in [
    //     (use_lowercase, LOWERCASE),
    //     (use_uppercase, UPPERCASE),
    //     (use_numbers, NUMBERS),
    //     (use_symbols, SYMBOLS),
    // ] {
    //     if enabled {
    //         charset.extend_from_slice(characters);
    //     }
    // }

    // // Creates an RNG thread based on your OS timestamp
    // // Means 'new random' every time you run the program.
    // use rand::Rng;
    // let mut rng = rand::thread_rng();

    // let password: String = (0..length.clamp(2, 32))
    //     .map(|_| {
    //         let idx = rng.gen_range(0..charset.len()); // Gets a random character from the charset
    //         charset[idx] as char
    //     })
    //     .collect(); // Builds the string

    // println!("Password output: {}", password); // Prints it to the terminal
    // //clear();
    // loop {} // Temporary! Keeps program from closing so you can see the password printed on your screen
}

// This will be used to clear the screan, works on windows/mac/linux
fn clear() {
    use std::process::Command;
    if cfg!(target_os = "windows") {
        Command::new("cmd").args(["/C", "cls"]).status().unwrap(); //If on windows, clear screen specific way
    } else {
        Command::new("clear").status().unwrap(); //Other better systems allow this automatically
    }
}
