use std::fs;
use std::io;

pub fn read_input(file_path: &str) -> Result<Vec<Vec<String>>, io::Error> {
    let content = fs::read_to_string(file_path)?;
    let data = content
        .lines()
        .map(|line| line.chars().map(|c| c.to_string()).collect::<Vec<String>>())
        .collect::<Vec<Vec<String>>>();
    Ok(data)
}

pub fn calculate_roll_part_1(grid: Vec<Vec<String>>) -> i32 {
    const DIRS: [(isize, isize); 8] = [
        (1, 0),
        (0, 1),
        (1, 1),
        (-1, 0),
        (0, -1),
        (-1, -1),
        (-1, 1),
        (1, -1),
    ];

    let m = grid.len();
    let n = grid[0].len();
    let mut tot = 0;

    for r in 0..m {
        for c in 0..n {
            if grid[r][c] != "@" {
                continue;
            }
            let mut count = 0;

            for (dr, dc) in DIRS {
                let nr = r as isize + dr;
                let nc = c as isize + dc;

                if nr >= 0 && nr < m as isize && nc >= 0 && nc < n as isize {
                    let (nr, nc) = (nr as usize, nc as usize);

                    if grid[nr][nc] == "@" {
                        count += 1;
                    }
                }
            }

            if count < 4 {
                tot += 1;
            }
        }
    }
    tot
}

pub fn calculate_roll_part_2(grid: &mut Vec<Vec<String>>) -> i32 {
    const DIRS: [(isize, isize); 8] = [
        (1, 0),
        (0, 1),
        (1, 1),
        (-1, 0),
        (0, -1),
        (-1, -1),
        (-1, 1),
        (1, -1),
    ];

    let m = grid.len();
    let n = grid[0].len();
    let mut total_removed = 0;

    loop {
        let mut to_remove = Vec::new();

        for r in 0..m {
            for c in 0..n {
                if grid[r][c] != "@" {
                    continue;
                }

                let mut count = 0;

                for (dr, dc) in DIRS {
                    let nr = r as isize + dr;
                    let nc = c as isize + dc;

                    if nr >= 0 && nc >= 0 && nr < m as isize && nc < n as isize {
                        if grid[nr as usize][nc as usize] == "@" {
                            count += 1;
                        }
                    }
                }

                if count < 4 {
                    to_remove.push((r, c));
                }
            }
        }

        if to_remove.is_empty() {
            break;
        }

        for (r, c) in &to_remove {
            grid[*r][*c] = ".".to_string();
        }

        total_removed += to_remove.len() as i32;
    }

    total_removed
}
