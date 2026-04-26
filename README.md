# passgen

Hello groupmates or professor!

This code so far is very minimal and simply outputs one password. A full project would be having options and adding real security methods.
Let me know what ya think!

The output will look something like this:
<img width="506" height="225" alt="f846b5fb09ef3666edd7403e6386b199" src="https://github.com/user-attachments/assets/5f7330a9-4832-4c9e-bafe-e4f686dc2fc6" />

If you would like to see how the prototype was coded go here:
https://github.com/ccstatic/passgen/blob/main/main/src/main.rs

If you want to run it for yourself go here:
https://github.com/ccstatic/passgen/releases/tag/Prototype

The code is documented so if you read it top to bottom you should have a rough idea of what it does and how it does it.
The langauge used is Rust as it's beginner friendly.

IDEAS:

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
