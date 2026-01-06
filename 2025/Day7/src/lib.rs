use std::collections::{HashMap, HashSet};
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

pub fn quantum_splitting(grid: &Vec<Vec<char>>) -> u128 {
    let m = grid.len();
    let n = grid[0].len();

    let start_col = grid[0].iter().position(|&c| c == 'S').expect("No S found");
    let mut memo: HashMap<(usize, usize), u128> = HashMap::new();
    let result = dfs(0, start_col, grid, m, n, &mut memo);

    result
}

fn dfs(
    r: usize,
    c: usize,
    grid: &Vec<Vec<char>>,
    rows: usize,
    cols: usize,
    memo: &mut HashMap<(usize, usize), u128>,
) -> u128 {
    if let Some(&v) = memo.get(&(r, c)) {
        return v;
    }

    // Base case: Reached the end of the grid
    if r >= rows {
        return 1; // One timeline successfully completed
    }

    let tile = grid[r][c];
    let mut count = 0;

    match tile {
        'S' | '|' | '.' => {
            // Particle moves vertically through S, |, or .
            let nr = r + 1;
            // The particle always tries to move down.
            // If nr >= rows, it's handled by the base case above.
            count = dfs(nr, c, grid, rows, cols, memo);
        }
        '^' => {
            // Splitter: particle goes left and right
            let nr = r + 1;
            // Particle attempts to move to (nr, c-1) and (nr, c+1)
            // Need to check bounds for c-1 and c+1
            if c > 0 {
                // Check left path
                count += dfs(nr, c - 1, grid, rows, cols, memo);
            }
            if c + 1 < cols {
                // Check right path
                count += dfs(nr, c + 1, grid, rows, cols, memo);
            }
        }
        _ => {}
    }

    memo.insert((r, c), count);
    count
}
