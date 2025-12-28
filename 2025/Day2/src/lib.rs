/// This file contains all the necessary import for the day 2 part 1.
use std::fs;
use std::io;

pub fn read_input(file_location: &str) -> Result<Vec<String>, io::Error> {
    let content = fs::read_to_string(file_location)?;

    let values = content.split(',').map(|s| s.trim().to_string()).collect();

    Ok(values)
}

pub fn get_range(content: &str) -> Vec<String> {
    let x: Vec<String> = content.split('-').map(|s| s.to_string()).collect();

    x
}

pub fn is_valid_pattern(id: &str) -> bool {
    let n = id.len();
    if n == 0 || n % 2 != 0 {
        return false;
    }

    let mid = n / 2;
    let first_half = &id[..mid];
    let second_half = &id[mid..];

    first_half == second_half
}

/// for part 2
pub fn is_invalid(id: &str) -> bool {}
