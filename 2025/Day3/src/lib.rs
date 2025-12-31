use std::fs;
use std::io;

pub fn maxi_two_digit(banks: &Vec<i32>) -> i32 {
    let mut best_value = 0;
    let mut best_first = None;
    /// We use Some not to accidentally use the None while assigning or doing operations.
    for &c in banks {
        if let Some(f) = best_first {
            let candidate = f * 10 + c;

            if candidate > best_value {
                best_value = candidate;
            }
        }

        match best_first {
            Some(f) if f >= c => {}
            _ => best_first = Some(c),
        }
    }

    best_value
}

pub fn read_input(file_path: &str) -> Result<Vec<Vec<i32>>, io::Error> {
    let content = fs::read_to_string(file_path).expect("Failed to read file");

    let data: Vec<Vec<i32>> = content
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).expect("Not a digit") as i32)
                .collect()
        })
        .collect();

    Ok(data)
}

pub fn digits_to_i128(digits: &Vec<i32>) -> i128 {
    digits.iter().fold(0i128, |acc, &d| acc * 10 + d as i128)
}

pub fn maxi_twelve_digit(banks: &Vec<i32>) -> Vec<i32> {
    let k: usize = 12;
    let mut stack = Vec::new();
    let mut to_remove = banks.len().saturating_sub(12);

    for &d in banks {
        while to_remove > 0 && !stack.is_empty() && *stack.last().unwrap() < d {
            stack.pop();
            to_remove -= 1;
        }
        stack.push(d);
    }
    stack.truncate(k);
    stack
}
