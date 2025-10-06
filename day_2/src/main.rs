use std::fs::read_to_string;

const DIGIT_WORDS: [(&str, u32); 20] = [
    ("0", 0),
    ("1", 1),
    ("2", 2),
    ("3", 3),
    ("4", 4),
    ("5", 5),
    ("6", 6),
    ("7", 7),
    ("8", 8),
    ("9", 9),
    ("zero", 0),
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
];

enum Direction {
    FromStart,
    FromEnd
}

fn find_digit(line: &str, direction: Direction) -> Option<u32> {
    let range: Vec<usize> = match direction {
        Direction::FromStart => (0..line.len()).collect(),
        Direction::FromEnd => (0..line.len()).rev().collect(),
    };
    for pos in range {
        let line_slice = &line[pos..];
        for (digit_str, digit_value) in DIGIT_WORDS.iter() {
            if line_slice.starts_with(digit_str) {
                return Some(*digit_value);
            }
        }
    }
    None
}   

fn main() {
    let mut sum = 0;
    for line in read_to_string("puzzle_input").unwrap().lines() {
        let first_digit = find_digit(line, Direction::FromStart).unwrap();
        let last_digit = find_digit(line, Direction::FromEnd).unwrap();
        let calibration_value = first_digit * 10 + last_digit;
        sum += calibration_value;
    }
    println!("Sum of calibration values: {sum}");
}
