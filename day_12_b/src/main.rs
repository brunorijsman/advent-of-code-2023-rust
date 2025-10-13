use memoize::memoize;
use std::fs::read_to_string;

fn main() {
    let mut total = 0;
    for record in read_to_string("puzzle_input").unwrap().lines() {
        total += nr_arrangements_for_record(record);
    }
    println!("Total number of possible arrangements: {}", total);
}

fn nr_arrangements_for_record(record: &str) -> u64 {
    let (short_symbols, counts) = record.split_once(' ').unwrap();
    let short_symbols = String::from(short_symbols);
    // Quintuple the symbols as requested in the puzzle.
    // Add a trailing . to ensure the last group of #'s is terminated.
    let symbols = format!(
        "{}?{}?{}?{}?{}.",
        short_symbols, short_symbols, short_symbols, short_symbols, short_symbols
    );
    let short_counts: Vec<u32> = counts
        .split(',')
        .map(|s| s.parse::<u32>().unwrap())
        .collect();
    let counts = [
        &short_counts[..],
        &short_counts[..],
        &short_counts[..],
        &short_counts[..],
        &short_counts[..],
    ]
    .concat();
    let nr_arrangements = nr_arrangements(symbols, counts, false);
    nr_arrangements
}

#[memoize] // Memoization is essential for performance
fn nr_arrangements(symbols: String, counts: Vec<u32>, in_hashes: bool) -> u64 {
    if symbols.is_empty() {
        return if counts.is_empty() {
            // Symbols is empty and counts is also empty => 1 match
            1
        } else {
            // Symbols is empty but counts is not empty => 0 match
            0
        };
    }
    if counts.is_empty() {
        return if symbols.chars().all(|c| c != '#') {
            // Counts is empty and remaining symbols are . or ? => 1 match
            1
        } else {
            // Counts is empty and remaining symbols have # => 0 match
            0
        };
    }
    let first_symbol = symbols.chars().next().unwrap();
    let rest_symbols = String::from(&symbols[1..]);
    let first_count = counts[0];
    let result = if first_symbol == '#' {
        // First symbol is #
        if in_hashes {
            // Continuing in group of #'s
            if first_count == 0 {
                // Did not expect another # => 0 match
                0
            } else {
                // Consume the #
                let mut new_counts = counts.to_vec();
                new_counts[0] -= 1;
                nr_arrangements(rest_symbols, new_counts, true)
            }
        } else {
            assert!(first_count > 0);
            // Starting new group of #'s, consume first #
            let mut new_counts = counts.to_vec();
            new_counts[0] -= 1;
            nr_arrangements(rest_symbols, new_counts, true)
        }
    } else if first_symbol == '.' {
        // First symbol is .
        if in_hashes {
            // Ending group of #'s
            if first_count == 0 {
                // Consumed expected number of #'s
                let new_counts = counts[1..].to_vec();
                nr_arrangements(rest_symbols, new_counts, false)
            } else {
                // Expected more #'s => 0 match
                0
            }
        } else {
            // Continuing in .'s
            nr_arrangements(rest_symbols, counts, false)
        }
    } else if first_symbol == '?' {
        // First symbol is ?, try both # and .
        let rest_symbols = String::from(&symbols[1..]);
        let dash_symbol = format!("#{}", &rest_symbols);
        let dot_symbol = format!(".{}", &rest_symbols);
        let hash_arrangements = nr_arrangements(dash_symbol, counts.clone(), in_hashes);
        let dot_arrangements = nr_arrangements(dot_symbol, counts.clone(), in_hashes);
        let total_arrangements = hash_arrangements + dot_arrangements;
        total_arrangements
    } else {
        panic!("Unexpected symbol: {}", first_symbol);
    };
    result
}
