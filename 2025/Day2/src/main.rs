mod lib;
use lib::{get_range, is_valid_pattern, read_input};
use std::env;

fn main() {
    let file_path = env::args().nth(1).expect("Please Provide a file path");

    let content = read_input(&file_path).unwrap();

    let mut total_sum = 0;
    for x in &content {
        let result: Vec<String> = get_range(x);
        let start = &result[0];
        let end = &result[1];
        let s: i64 = start.parse().unwrap();
        let e: i64 = end.parse().unwrap();

        for i in s..=e {
            let val = i.to_string();
            if is_valid_pattern(&val) {
                total_sum += i;
            }
        }
    }
    println!("{}", total_sum);
}
