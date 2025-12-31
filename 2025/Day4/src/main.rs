mod lib;

use lib::{calculate_roll_part_1, calculate_roll_part_2, read_input};

fn main() {
    let file_path = "/home/nithesh/WindowsDrive/Coding/Rust/AdventOfCode/2025/Day4/src/input.txt";
    let mut grid = read_input(file_path).expect("Can't open the file");

    let tot = calculate_roll_part_2(&mut grid);
    println!("{}", tot);
}
