use std::io;
use std::string::String;

fn main() -> std::io::Result<()> {
    // Mutable variable that holds input text
    let mut input = String::new();

    // Prints a prompt to the Terminal
    println!("\nEnter your text:");

    // Takes input from Terminal and stores number of bytes
    let bytes = io::stdin().read_line(&mut input).unwrap();

    // Number of total characters
    let characters = bytes - 1;

    let _iter = input.split_whitespace();

    println!("\nNumber of words read: {}", _iter.count());
    println!("Number of characters read: {}", characters);
    Ok(())
}
