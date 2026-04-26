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

/*  IDEA -> WHY
    CSPRNG (rand::rngs::OsRng) -> unpredictable, seedless, backtracking resistant, amazing entropy
    BYTES TO GIVEN CHARSET -> no modulo bias
    DEFAULT CONFIG -> strong defaults for instant and easy use for nontechnical people
    CONFIG -> length, charset presets, copy to clipboard, passphrase
    MEMORY -> scramble memory after use, clear clipboard after 30s, no string cloning
    OUTPUT -> confirm print before printing, no logging, autoclear after 30s
    SIDE CHANNEL -> avoid branching on sensitive data
    ANALYTICS -> no network calls at all, no telemetry, no analytics
    OTHER -> disable core dumps, dont allow swap writing, mark memory as dont dump ever
    EXTRA -> profiles, entropy estimate display
*/

/*
    draft for report

    COVER
    intro
    background on pass security
    threat model
    program overview
    requirements / design
    algorithms
    randomness
    implementation
    testing
    uses
    limitations
    conclusion
    REFERENCES
*/
