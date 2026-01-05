use std::error::Error;
use std::fs;

pub fn read_input(
    file_path: &str,
) -> Result<(Vec<Vec<i32>>, Vec<char>), Box<dyn std::error::Error>> {
    let content = fs::read_to_string(file_path)?;

    let lines: Vec<&str> = content.lines().filter(|l| !l.trim().is_empty()).collect();

    let (values, operation_lines) = lines.split_at(lines.len() - 1);

    let nums = divide_as_cols(values)?;
    let operations = parse_operations(operation_lines[0]);

    Ok((nums, operations))
}

pub fn sol_one(value: Vec<i32>, ope: char) -> i64 {
    match ope {
        '*' => {
            let mut tot: i64 = 1;

            for val in value {
                tot *= val as i64;
            }
            tot
        }
        '+' => {
            let mut tot: i64 = 0;
            for val in value {
                tot += val as i64;
            }
            tot
        }
        _ => panic!("Unsupported operations: {}", ope),
    }
}

fn divide_as_cols(lines: &[&str]) -> Result<Vec<Vec<i32>>, Box<dyn Error>> {
    // Parse rows first
    let rows: Vec<Vec<i32>> = lines
        .iter()
        .map(|line| {
            line.split_whitespace()
                .map(|v| v.parse::<i32>())
                .collect::<Result<Vec<_>, _>>()
        })
        .collect::<Result<_, _>>()?;

    if rows.is_empty() {
        return Ok(Vec::new());
    }

    let col_count = rows[0].len();

    // Enforce rectangular matrix
    if rows.iter().any(|r| r.len() != col_count) {
        return Err("Inconsistent column count".into());
    }

    let mut cols = vec![Vec::with_capacity(rows.len()); col_count];

    for row in rows {
        for (c, value) in row.into_iter().enumerate() {
            cols[c].push(value);
        }
    }

    Ok(cols)
}

fn parse_operations(line: &str) -> Vec<char> {
    line.split_whitespace()
        .map(|op| op.chars().next().unwrap())
        .collect()
}

pub struct Problem {
    pub operands: Vec<i64>,
    pub operator: char,
}

pub fn parse_input_part_two(file_path: &str) -> Result<Vec<Problem>, Box<dyn Error>> {
    let content = fs::read_to_string(file_path)?;
    let lines: Vec<&str> = content.lines().filter(|l| !l.trim().is_empty()).collect();

    if lines.len() < 2 {
        return Err("Input must contain numbers and an operator line".into());
    }

    let (number_lines, op_line) = lines.split_at(lines.len() - 1);
    let op_line = op_line[0];

    let max_width = number_lines.iter().map(|l| l.len()).max().unwrap_or(0);

    // 1. Identify problem column ranges
    let mut ranges = Vec::new();
    let mut in_problem = false;
    let mut start = 0;

    for col in 0..max_width {
        let is_space_col = number_lines
            .iter()
            .all(|line| line.chars().nth(col).map_or(true, |c| c.is_whitespace()));

        if !is_space_col && !in_problem {
            start = col;
            in_problem = true;
        } else if is_space_col && in_problem {
            ranges.push((start, col - 1));
            in_problem = false;
        }
    }

    if in_problem {
        ranges.push((start, max_width - 1));
    }

    // Problems are evaluated right-to-left
    ranges.reverse();

    // 2. Parse each problem column-wise
    let mut problems = Vec::new();

    for (start, end) in ranges {
        let mut operands = Vec::new();

        for col in start..=end {
            let mut value: i64 = 0;
            let mut has_digit = false;

            for line in number_lines {
                if let Some(c) = line.chars().nth(col) {
                    if c.is_ascii_digit() {
                        value = value * 10 + (c as i64 - '0' as i64);
                        has_digit = true;
                    }
                }
            }

            if has_digit {
                operands.push(value);
            }
        }

        let operator = op_line[start..=end]
            .chars()
            .find(|c| !c.is_whitespace())
            .ok_or("Missing operator")?;

        problems.push(Problem { operands, operator });
    }

    Ok(problems)
}

pub fn sol_two(problems: Vec<Problem>) -> i64 {
    let mut grand_total: i64 = 0;

    for problem in problems {
        let mut problem_result: i64 = match problem.operator {
            '*' => 1,
            '+' => 0,
            _ => panic!("Unsupported operation: {}", problem.operator),
        };

        match problem.operator {
            '*' => {
                for operand in problem.operands {
                    problem_result *= operand;
                }
            }
            '+' => {
                for operand in problem.operands {
                    problem_result += operand;
                }
            }
            _ => panic!("Unsupported operation: {}", problem.operator),
        }
        grand_total += problem_result;
    }

    grand_total
}
