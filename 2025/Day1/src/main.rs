mod decoration;

use decoration::decoration;

use std::env;
use std::fs;

fn main() {
    let file_path = env::args().nth(1).expect("Please Provide a file path");

    let contents: String =
        fs::read_to_string(file_path).expect("Should have been able to read the file");

    // Borrow the contents as a &str
    let text: &str = &contents;

    let instructions: Vec<&str> = text.split_whitespace().collect();

    let result = decoration(&instructions);

    println!("Result: {}", result);
}
