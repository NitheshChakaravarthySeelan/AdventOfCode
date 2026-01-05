use day6::{parse_input_part_two, sol_two};

fn main() {
    let file_path = "/home/nithesh/WindowsDrive/Coding/Rust/AdventOfCode/2025/Day6/src/input.txt";

    // Part Two Logic
    let problems = parse_input_part_two(file_path).expect("Error parsing input for Part Two");
    let grand_total_part_two = sol_two(problems);
    println!("{}", grand_total_part_two);
}
