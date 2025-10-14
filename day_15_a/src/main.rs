use std::fs::read_to_string;

fn main() {
    let init_seq = read_init_seq();
    let sum = sum_hashes(&init_seq);
    println!("Sum of hashes: {}", sum);
}

fn read_init_seq() -> Vec<String> {
    let contents = read_to_string("puzzle_input").unwrap();
    let mut lines = contents.lines();
    lines
        .next()
        .unwrap()
        .split(',')
        .map(|s| s.to_string())
        .collect()
}

fn sum_hashes(init_seq: &Vec<String>) -> u64 {
    init_seq.iter().fold(0, |acc, vals| acc + hash(vals))
}

fn hash(s: &String) -> u64 {
    let bytes = s.as_bytes();
    let mut hash: u64 = 0;
    for byte in bytes {
        hash += *byte as u64;
        hash *= 17;
        hash %= 256;
    }
    hash
}
