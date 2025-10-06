use std::fs::read_to_string;

fn digit_at_pos(s: &str, pos: usize) -> u32 {
    s.chars().nth(pos).unwrap().to_digit(10).unwrap()
}

fn main() {
    let mut sum = 0;
    for line in read_to_string("puzzle_input").unwrap().lines() {
        let is_digit = |c: char| c.is_ascii_digit();
        let first_digit_pos = line.find(is_digit).unwrap();
        let first_digit = digit_at_pos(line, first_digit_pos);
        let last_digit_pos = line.rfind(is_digit).unwrap();
        let last_digit = digit_at_pos(line, last_digit_pos);
        let calibration_value = first_digit * 10 + last_digit;
        sum += calibration_value;
    }
    println!("Sum of calibration values: {sum}");
}
