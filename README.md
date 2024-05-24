# Rusty Bcrypt Hasher
Rust Bcrypt Hasher is a simple and efficient command-line application written in Rust that generates bcrypt hashes from a given input. This tool was created to address a specific need in a web application where I had to implement an authentication system without a register feature.

## Key Features

- <strong>Bcrypt Hash Generation:</strong> Securely generate bcrypt hashes from input strings.
- <strong>Command-Line Interface:</strong> Easy-to-use CLI for generating hashes on the fly.
- <strong>Security Enhancement:</strong> Allows pre-creation of base passwords for employees to use initially and then change to their personal passwords upon first login.

## Use Case

In a web application, I needed a way to securely handle passwords for employees without providing a registration feature. The solution was to generate base passwords that employees could use to log in for the first time and then update to their own passwords. Rust Bcrypt Hasher was created to facilitate this need.

## Technology Used

- <strong>Rust:</strong> Programming language used for its performance and safety features. PS: the best🍷
- <strong>Bcrypt:</strong> Library used for generating secure password hashes.

## How to Use



1. <strong>Clone the Repository:</strong>
``
git clone https://github.com/yourusername/rust-bcrypt-hasher.git
``
2. <strong>Navigate to the Project Directory:</strong>
``
cd rust-bcrypt-hasher
``
3. <strong>Build the Application:</strong>
``
cargo build --release
``
4. <strong>Run the Application:</strong>
``
./target/release/rust-bcrypt-hasher "your_password_here"
``

## Example
To generate a bcrypt hash for the password "my_secure_password":
``./target/release/rust-bcrypt-hasher "my_secure_password"``

<br/><br/>

## Disclaimer
This application was developed to solve a specific problem within a web application project. It is a simple tool and may require further enhancements for broader use cases.

Feel free to explore, contribute, and enhance Rusty Bcrypt Hasher!