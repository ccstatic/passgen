const LOWERCASE: &[u8] = b"abcdefghijklmnopqrstuvwxyz";
const UPPERCASE: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const NUMBERS: &[u8] = b"0123456789";
const SYMBOLS: &[u8] = b"!@#$%^&*()-_=+[]{};:,.<>?/";

const MAX_LENGTH: usize = 2048;
const MAX_AMOUNT: usize = 32;
const CLIPBOARD_CLEAR_SECONDS: u64 = 30;

use arboard::Clipboard;
use clap::{ArgAction, Parser};
use rand::rngs::OsRng;
use rand::seq::SliceRandom;
use std::process::Command;
use std::thread::sleep;
use std::time::Duration;
use zeroize::{Zeroize, Zeroizing};

#[derive(Parser)]
struct Args {
    // By default, we do not show the password in the terminal
    #[arg(long)]
    show: bool,

    // By default, we copy the last password to the clipboard
    #[arg(long = "no-clipboard", action = ArgAction::SetFalse)]
    clipboard: bool,

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
    if charset.is_empty() {
        eprintln!("At least one character set has to be enabled.");
        std::process::exit(1);
    }

    if !args.show && !args.clipboard {
        eprintln!("At least have to show password or copy it to clipboard.");
        std::process::exit(1);
    }

    // We don't want terminal-breaking amount of characters being pushed to the screen
    let length = args.length.clamp(2, MAX_LENGTH);
    let amount = args.amount.clamp(1, MAX_AMOUNT);

    // Uses the operating systems crypto secure random number generator
    let mut rng = OsRng;
    let mut last_password = Zeroizing::new(String::new());

    for _ in 0..amount {
        let password = Zeroizing::new(
            (0..length)
                .map(|_| *charset.choose(&mut rng).unwrap() as char)
                .collect::<String>(),
        );

        if args.show {
            println!("{}", password.as_str());
            if amount > 1 {
                println!();
            }
        }

        last_password = password;
    }

    if args.clipboard {
        let mut clipboard = Clipboard::new().expect("Failed to access clipboard");
        clipboard
            .set_text(last_password.as_str())
            .expect("Failed to copy password");

        println!("Password(s) securely generated and last generated was copied to the clipboard");
        sleep(Duration::from_secs(CLIPBOARD_CLEAR_SECONDS));
        if let Ok(mut current_clipboard) = clipboard.get_text() {
            if current_clipboard == last_password.as_str() {
                clipboard.set_text("").expect("Failed to clear clipboard");
            }
            // Clipboard string is also zeroized
            current_clipboard.zeroize();
        }
    } else {
        println!("Password(s) securely generated");

        // Stalls terminal so you can see your output
        sleep(Duration::from_secs(CLIPBOARD_CLEAR_SECONDS));
    }

    // Wipes the password buffer
    last_password.zeroize();

    // Clears output in-case you left the program open
    if cfg!(target_os = "windows") {
        Command::new("cmd").args(["/C", "cls"]).status().unwrap();
    } else {
        Command::new("clear").status().unwrap();
    }

    if args.clipboard {
        println!("Clipboard cleared and password memory wiped");
    } else {
        println!("Password memory wiped");
    }
}

#[cfg(test)]
mod tests;
