use std::io;
use std::string::String;

fn main(){
    // Mutable variable that holds input text
    let mut input = String::new();

    // Prints a prompt to the Terminal
    println!("Enter your text:");

    // Takes input from Terminal and stores number of bytes
    let bytes = io::stdin().read_line(&mut input).unwrap();

    // Number of total characters
    let characters = bytes - 1;

    let _iter = input.split_whitespace();

    // println!("You typed: {}", input.trim());
    println!("Number of words read: {}", _iter.count());
    println!("Number of characters read: {}", characters);
    // println!("Number of bytes read: {}", bytes)
}
