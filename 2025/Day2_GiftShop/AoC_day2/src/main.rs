use std::fs;

// Part 1
fn get_repeated_ids(start: &i64, end: &i64) -> i64 {
    let mut sum: i64 = 0;
    for id in *start..=*end {
        let id_string = id.to_string();
        let midpoint = id_string.len() / 2;
        let (left, right) = id_string.split_at(midpoint);
        if left == right {
            sum += id;
        }
    }
    sum
}

// Part 2 - wth did i just write
fn get_repeating_digits(start: &i64, end: &i64) -> i64 {
    let mut sum: i64 = 0;
    for id in *start..=*end {
        let id_string = id.to_string();
        for splits in 1..id_string.len() {
            // split the string into chunks
            let chunks: Vec<String> = id_string
                .chars()
                .collect::<Vec<_>>()
                .chunks(splits)
                .map(|chunk| chunk.iter().collect())
                .collect();

            // check if the chunks match each other
            let prev_string: String = String::from(chunks.first().expect("Cannot access Vector"));
            let mut does_match = true;
            for value in chunks {
                if value != prev_string {
                    does_match = false;
                    break;
                }
            }

            // if every chunk is the same as every other chunk, then this ID is invalid, add it to
            // sum and go to the next one.
            if does_match {
                sum += id;
                break;
            }
        }
    }
    sum
}

fn main() {
    // get input and set up answer variables
    let input = fs::read_to_string("puzzle_input.txt").expect("Cannot open file!");
    let mut part_1_answer: i64 = 0;
    let mut part_2_answer: i64 = 0;

    // split the string by comma
    for id_range in input.split(',') {
        // split the string by the hyphen
        let segment: Vec<_> = id_range.split('-').collect();

        // get the ranges of the ids
        let lower: i64 = segment
            .first()
            .expect("Couldnt access first element")
            .parse()
            .expect("Could not convert string to integer");

        let higher: i64 = segment
            .last()
            .expect("Couldnt access last element")
            .trim_end()
            .parse()
            .expect("Could not convert string to integer");

        part_1_answer += get_repeated_ids(&lower, &higher);
        part_2_answer += get_repeating_digits(&lower, &higher);
    }

    println!("Part 1: {}, Part 2: {}", part_1_answer, part_2_answer);
}
