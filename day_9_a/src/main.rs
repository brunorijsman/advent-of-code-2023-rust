use std::fs::read_to_string;

fn main() {
    let mut sum = 0;
    for line in read_to_string("puzzle_input").unwrap().lines() {
        let numbers: Vec<i64> = line
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();
        sum += next_number(&numbers);
    }
    println!("Sum of next numbers is {sum}");
}

fn next_number(numbers: &Vec<i64>) -> i64 {
    if numbers.iter().all(|&x| x == 0) {
        return 0
    }
    let mut diffs = Vec::new();
    for i in 1..numbers.len() {
        diffs.push(numbers[i] - numbers[i - 1]);
    }
    numbers[numbers.len() - 1] + next_number(&diffs)
}
