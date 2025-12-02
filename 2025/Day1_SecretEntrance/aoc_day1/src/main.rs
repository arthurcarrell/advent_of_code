use std::fs;

fn get_dial_movement(value: &str) -> i32 {
    // calculate movement
    let multiplier: i32;

    match value.chars().nth(0) {
        Some('L') => multiplier = -1,
        Some('R') => multiplier = 1,
        _ => multiplier = 0,
    }

    if multiplier == 0 {
        println!("Something broke")
    }

    // get the rest of the characters
    let num_as_string: String = value.chars().skip(1).collect();
    let num: i32 = num_as_string
        .parse()
        .expect("Failed to parse string to integer");

    // multiply the numbers together
    return num * multiplier;
}

fn main() {
    let mut dial: i32 = 50;

    let contents: String =
        fs::read_to_string("puzzle_input.txt").expect("Should be able to read file!");

    // interate through contents and run get_dial_movement on it
    for number in contents.split('\n').enumerate() {
        // get the dial movement
        dial += get_dial_movement(number.1);

        dial = dial.rem_euclid(100);
    }
}
