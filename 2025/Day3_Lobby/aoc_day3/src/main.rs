use std::fs;

fn get_value(line: &str) -> u32 {
    let mut prev_highest = 0;
    let mut value = 0;
    for (i, chr) in line.trim().chars().enumerate() {
        let digit = chr.to_digit(10).expect("Cannot convert to digit");

        if digit > prev_highest {
            for (j, chr2) in line.trim().chars().enumerate().skip(i) {
                let digit2 = chr2.to_digit(10).expect("Cannot conver to digit");

                // make sure same index is not being checked
                if j == i {
                    continue;
                }
                if (digit * 10 + digit2) > value {
                    value = digit * 10 + digit2;
                    prev_highest = digit;
                }
            }
        }
    }
    value
}

fn get_max_joltage(line: &str, digits: usize) -> u64 {
    let input: Vec<u8> = line
        .trim()
        .chars()
        .map(|chr| chr.to_digit(10).unwrap() as u8)
        .collect();

    let length = input.len();
    let mut digits_to_drop: u32 = (length - digits) as u32;

    // create a stack
    let mut stack: Vec<u8> = Vec::with_capacity(digits);

    for &num in &input {
        // while we can drop numbers and the prev digit is smaller then remove
        while digits_to_drop > 0 && !stack.is_empty() && stack.last().unwrap() < &num {
            stack.pop();
            digits_to_drop -= 1;
        }
        stack.push(num);
    }

    // truncate and then fold it into an integer and return it
    stack.truncate(digits);
    stack.iter().fold(0u64, |acc, &d| acc * 10 + d as u64)
}

fn main() {
    // get puzzle input and split it by lines
    let input = fs::read_to_string("puzzle_input.txt").expect("Failed to open file");
    let mut part1: u32 = 0;
    let mut part2: u64 = 0;
    for line in input.lines() {
        part1 += get_value(line);
        part2 += get_max_joltage(line, 12);
    }
    println!("Part 1: {}, Part 2: {}", part1, part2);
}
