# Passgen
A secure and easy-to-use password generator application. Generate passwords and passphrases with many customizable options right from your computer.

# Read here!
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

# Features
* Creates secure random passwords purely from the command line
* Allows password options like length, amount, and character sets
* Copies the last generated password to the clipboard by default and clears it safely after 30 seconds
* Prevents unnecessary terminal output by limiting maximum password length and amount
* Uses memory zeroization to reduce leftover memory

# Use
<img width="972" height="505" alt="image" src="https://github.com/user-attachments/assets/a0fc0a33-68dc-45ec-8532-711157d8a036" />

When you run the main.exe program, you'll be met with an output like this by default. If you paste into a text editor or a text box you'll have a, by default, a strongly generated password locally and securely generated.

With this passgen.exe however, you can go to the path of the .exe inside your terminal and run it with some configurations.
Some examples:
`passgen --show` to see it in the terminal.
<img width="557" height="36" alt="image" src="https://github.com/user-attachments/assets/4e88fa38-7688-4c2d-938e-e1029b207106" />

`passgen --length 32 --show` or `passgen -l 32 --show` to change the length of the password.
<img width="587" height="59" alt="image" src="https://github.com/user-attachments/assets/ddb84a6a-9129-48b5-b289-177e7855befb" />

`passgen --amount 5 --show` to show five passwords
<img width="599" height="215" alt="image" src="https://github.com/user-attachments/assets/60ce6a3d-2c21-483f-9872-008f631a6305" />

`passgen --show --no-symbols` to prevent symbols from being generated.
<img width="571" height="69" alt="image" src="https://github.com/user-attachments/assets/6096b40d-b9b8-43e5-866f-c7353274c08f" />

`passgen --show --no-numbers` to prevent numbers from being generated.
<img width="589" height="58" alt="image" src="https://github.com/user-attachments/assets/d3ff6085-f2db-4961-8328-269bb60b16e5" />

`passgen --show --no-uppercase` to prevent uppercase from being generated.
<img width="602" height="60" alt="image" src="https://github.com/user-attachments/assets/b5d9d4f9-555f-4722-8d5c-ce62c993bdba" />

`passgen --show --no-lowercase` to prevent lowercase from being generated.
<img width="628" height="56" alt="image" src="https://github.com/user-attachments/assets/e6a25e07-3418-4a17-ab67-f2584e0944ea" />

`passgen --show --no-uppercase --no-lowercase` for numbers and symbols only.
<img width="588" height="58" alt="image" src="https://github.com/user-attachments/assets/1d55d31c-2178-4bc4-bbcf-7a1fd7a531ad" />

`passgen --show --no-symbols --no-numbers` for letters only.
<img width="579" height="61" alt="image" src="https://github.com/user-attachments/assets/1b3eca87-c149-409e-8ac3-652be0567925" />

Some examples that will rightfully error:
`passgen --no-clipboard` errors because by default we don't show it in the terminal and you didn't allow copying, meaning the program is useless with these settings.
<img width="743" height="55" alt="image" src="https://github.com/user-attachments/assets/fb611d6d-4bb4-4565-849c-45e741c0e70d" />
