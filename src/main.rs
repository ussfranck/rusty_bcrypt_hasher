use bcrypt::{hash, DEFAULT_COST};
use std::io::{self};
use std::process::exit;

/*
    This is a simple Rust program that uses the bcrypt library to hash a user-provided string.
    The program will continue to run until the user types 'q' to quit.

    To run the program, save it as a .rs file and execute the following command in the terminal:
    $ rustc main.rs && ./main.
    @USS Franck
*/

fn main() {
    loop {
        println!("\n-> Enter a hasher phrase (or press 'q' to exit):");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Error reading user input");
        let input = input.trim();

        if input.eq_ignore_ascii_case("q") {
            println!("OK, stop program...");
            exit(0);
        }

        match hash(input, DEFAULT_COST) {
            Ok(hashed) => println!("\t==> Hash bcrypt: {}", hashed),
            Err(e) => println!("Reject! Error when hashing value: {}", e),
        }
    }
}
