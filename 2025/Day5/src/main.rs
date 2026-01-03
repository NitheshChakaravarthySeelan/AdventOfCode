mod lib;

use lib::count_fresh_ids;
use lib::read_input;

fn main() {
    let file_path = "/home/nithesh/WindowsDrive/Coding/Rust/AdventOfCode/2025/Day5/src/input.txt";
    let (mut ranges, values) = read_input(file_path).expect("No file is found");

    let fresh_count = values
        .iter()
        .filter(|&&val| ranges.iter().any(|&(a, b)| a <= val && val <= b))
        .count();

    let fresh_ids = count_fresh_ids(&mut ranges);
    println!("{}", fresh_ids);
}
