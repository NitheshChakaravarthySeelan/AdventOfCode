use std::collections::HashSet;
use std::{fs, io, usize};

pub fn read_input(file_path: &str) -> Result<Vec<Vec<char>>, io::Error> {
    let content = fs::read_to_string(file_path)?;
    let grid: Vec<Vec<char>> = content.lines().map(|line| line.chars().collect()).collect();
    Ok(grid)
}

pub fn total_splitting(grid: &Vec<Vec<char>>) -> i32 {
    let m = grid.len();
    let n = grid[0].len();

    let start_col = grid[0].iter().position(|&c| c == 'S').expect("No S found");

    let mut active_cols: HashSet<usize> = HashSet::new();
    active_cols.insert(start_col);

    let mut split_count = 0;

    // Start from row 1
    for row in 1..m {
        if active_cols.is_empty() {
            break;
        }
        let mut next_active_cols = HashSet::new();

        for col in active_cols {
            match grid[row][col] {
                '.' => {
                    next_active_cols.insert(col);
                }
                '^' => {
                    split_count += 1;

                    if col > 0 {
                        next_active_cols.insert(col - 1);
                    }
                    if col + 1 < n {
                        next_active_cols.insert(col + 1);
                    }
                }
                _ => {}
            }
        }
        active_cols = next_active_cols;
    }
    split_count
}
