mod lib;

use lib::{digits_to_i128, maxi_twelve_digit, maxi_two_digit, read_input};

fn main() {
    let file_path = "/home/nithesh/WindowsDrive/Coding/Rust/AdventOfCode/2025/Day3/src/input.txt";

    let content = read_input(file_path).expect("Can't read the file");

    let mut tot: i128 = 0;

    for c in &content {
        let val = maxi_twelve_digit(c);
        let fi = digits_to_i128(&val);
        tot += fi;
    }
    println!("{}", tot);
}
