mod lib;

use lib::{read_input, total_splitting};

fn main() {
    let file_path = "/home/nithesh/WindowsDrive/Coding/Rust/AdventOfCode/2025/Day7/src/input.txt";
    let grid = read_input(file_path).expect("Cant find the file");
    let total = total_splitting(&grid);
    print!("{}", total);
}
