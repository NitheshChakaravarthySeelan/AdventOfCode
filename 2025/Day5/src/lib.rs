use std::{fs, io};

pub fn read_input(file_path: &str) -> Result<(Vec<(i128, i128)>, Vec<i128>), io::Error> {
    let content = fs::read_to_string(file_path)?;
    // Split on blank lines
    let mut sections = content.split("\n\n");

    // Parse range
    let ranges: Vec<(i128, i128)> = sections
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            let mut parts = line.split('-');
            let start = parts.next().unwrap().parse().unwrap();
            let end = parts.next().unwrap().parse().unwrap();

            (start, end)
        })
        .collect();

    // Parse number
    let values: Vec<i128> = sections
        .next()
        .unwrap()
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();

    Ok((ranges, values))
}

pub fn count_fresh_ids(ranges: &mut Vec<(i128, i128)>) -> i128 {
    if ranges.is_empty() {
        return 0;
    }

    // Merge intervals
    ranges.sort_unstable();

    let mut total = 0;
    let mut current_start = ranges[0].0;
    let mut current_end = ranges[0].1;

    for &(start, end) in &ranges[1..] {
        if start <= current_end + 1 {
            current_end = current_end.max(end);
        } else {
            total += current_end - current_start + 1;
            current_start = start;
            current_end = end;
        }
    }

    total += current_end - current_start + 1;
    total
}
