fn main() {
    use std::io;

    // Constant character sets
    const LOWERCASE: &[u8] = b"abcdefghijklmnopqrstuvwxyz";
    const UPPERCASE: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    const NUMBERS: &[u8] = b"0123456789";
    const SYMBOLS: &[u8] = b"!@#$%^&*()-_=+[]{};:,.<>?/";

    // By default, to be secure, we use everything
    let mut use_uppercase = true;
    let mut use_lowercase = true;
    let mut use_numbers = true;
    let mut use_symbols = true;

    println!("Enter password length:");

    //gets input of first line! gets the number the user enters
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    let length: usize = input.trim().parse().expect("Please enter a valid number");

    // Records/asks user if they want nums
    println!("Do you want numbers? y/n");

    let mut numbers_input = String::new();
    io::stdin()
        .read_line(&mut numbers_input)
        .expect("Failed to read input");

    // Records/asks user if they want symbols
    println!("Do you want symbols? y/n");

    let mut symbols_input = String::new();
    io::stdin()
        .read_line(&mut symbols_input)
        .expect("Failed to read input");

    if numbers_input.trim().to_lowercase() != "y" {
        use_numbers = false;
    }
    if symbols_input.trim().to_lowercase() != "y" {
        use_symbols = false;
    }

    // Builds set of characters based on their options
    let mut charset = Vec::new();
    for (enabled, characters) in [
        (use_lowercase, LOWERCASE),
        (use_uppercase, UPPERCASE),
        (use_numbers, NUMBERS),
        (use_symbols, SYMBOLS),
    ] {
        if enabled {
            charset.extend_from_slice(characters);
        }
    }

    // Creates an RNG thread based on your OS timestamp
    // Means 'new random' every time you run the program.
    use rand::Rng;
    let mut rng = rand::thread_rng();

    let password: String = (0..length.clamp(2, 32))
        .map(|_| {
            let idx = rng.gen_range(0..charset.len()); // Gets a random character from the charset
            charset[idx] as char
        })
        .collect(); // Builds the string

    println!("Password output: {}", password); // Prints it to the terminal
    //clear();
    loop {} // Temporary! Keeps program from closing so you can see the password printed on your screen
}

// This will be used to clear the screan, works on windows/mac/linux
#[allow(unused)] // Tells compiler that the function isnt being used
fn clear() {
    use std::process::Command;
    if cfg!(target_os = "windows") {
        Command::new("cmd").args(["/C", "cls"]).status().unwrap(); //If on windows, clear screen specific way
    } else {
        Command::new("clear").status().unwrap(); //Other better systems allow this automatically
    }
}
