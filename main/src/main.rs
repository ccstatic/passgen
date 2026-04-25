// The main fn runs first when you call the program.
// For example, this runs when you double click the .exe file or do cargo run
fn main() {
    // This is the possible characters the program can produce
    // In the future you would want uppercase, lowercase, numbers, and specials.
    let charset = b"abcdefhijklmnopqrstuvwxyz";

    // Creates an RNG thread based on your OS timestamp
    // Means 'new random' every time you run the program.
    use rand::Rng;
    let mut rng = rand::thread_rng();

    // 12 is the pass length, in the future you'd wanna ask the user
    // how long it should be.
    let length = 12;
    let password: String = (0..length)
        .map(|_| {
            let idx = rng.gen_range(0..charset.len()); // Gets a random character from the charset
            charset[idx] as char
        })
        .collect(); // Builds the string

    println!("Password output: {}", password); // Prints it to the terminal
    loop {} // Temporary! Keeps program from closing so you can see the password printed on your screen
}
