// Parse the instruction
fn parse_instruction(instruction: &str) -> i32 {
    let (_, digit) = instruction.split_at(1);
    // Converts from str to int32
    digit.parse().unwrap()
}

// Move the pointer and count zeros click by click
fn dial(pointer: &mut i32, instruction: &str, count: &mut i32) {
    let (dir, _) = instruction.split_at(1);
    let value = parse_instruction(instruction);

    for _ in 0..value {
        match dir {
            "L" => *pointer = (*pointer - 1).rem_euclid(100),
            "R" => *pointer = (*pointer + 1).rem_euclid(100),
            _ => unreachable!(),
        }

        if *pointer == 0 {
            *count += 1;
        }
    }
}

// Count the number of zeros
pub fn decoration(instructions: &[&str]) -> i32 {
    let mut count: i32 = 0;
    let mut pointer: i32 = 50;

    for instruction in instructions {
        dial(&mut pointer, instruction, &mut count);
    }

    count
}