use std::fs::read_to_string;

fn main() {
    let mut sum = 0;
    for line in read_to_string("puzzle_input").unwrap().lines() {
        let numbers: Vec<i64> = line
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();
        let (prev, _next) = prev_and_next_number(&numbers);
        sum += prev;
    }
    println!("Sum of previous numbers is {sum}");
}

fn prev_and_next_number(numbers: &Vec<i64>) -> (i64, i64) {
    if numbers.iter().all(|&x| x == 0) {
        return (0, 0)
    }
    let mut diffs = Vec::new();
    for i in 1..numbers.len() {
        diffs.push(numbers[i] - numbers[i - 1]);
    }
    let (prev_diff, next_diff) = prev_and_next_number(&diffs);
    let next = numbers[numbers.len() - 1] + next_diff;
    let prev = numbers[0] - prev_diff;
    (prev, next)
}
