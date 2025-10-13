use std::{fs::read_to_string, result};

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
    // TODO: Add a dot (explain why)
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
    println!(
        "Counting arrangements for symbols: {:?} counts: {:?}",
        symbols, counts
    );
    let nr_arrangements = nr_arrangements(0, &symbols, &counts, false);
    println!("Found {} arrangements", nr_arrangements);
    nr_arrangements
}

fn nr_arrangements(level: usize, symbols: &String, counts: &[u32], in_hashes: bool) -> u64 {
    let is = indent_str(level);
    println!(
        "{is}* Nr assignments for symbols: {:?} counts: {:?} in_hashes: {}",
        symbols, counts, in_hashes
    );
    if symbols.is_empty() {
        return if counts.is_empty() {
            // println!("{is}- Symbols is empty and counts is also empty => 1 match");
            1
        } else {
            // println!("{is}- Symbols is empty but counts is not empty => 0 match");
            0
        };
    }
    if counts.is_empty() {
        return if symbols.chars().all(|c| c != '#') {
            // println!("{is}- Counts is empty and remaining symbols are . or ? => 1 match");
            1
        } else {
            // println!("{is}- Counts is empty and remaining symbols have # => 0 match");
            0
        };
    }
    let first_symbol = symbols.chars().next().unwrap();
    let rest_symbols = String::from(&symbols[1..]);
    let first_count = counts[0];
    let result = if first_symbol == '#' {
        // println!("{is}- First symbol is #");
        if in_hashes {
            // println!("{is}- Continuing in group of hashes");
            if first_count == 0 {
                // println!("{is}- Did not expect another hash -> 0 match");
                0
            } else {
                // println!("{is}- Consume a hash");
                let mut new_counts = counts.to_vec();
                new_counts[0] -= 1;
                nr_arrangements(level + 1, &rest_symbols, &new_counts, true)
            }
        } else {
            assert!(first_count > 0);
            // println!("{is}- Starting new group of hashes, consume first hash");
            let mut new_counts = counts.to_vec();
            new_counts[0] -= 1;
            nr_arrangements(level + 1, &rest_symbols, &new_counts, true)
        }
    } else if first_symbol == '.' {
        // println!("{is}- First symbol is .");
        if in_hashes {
            // println!("{is}- Ending group of hashes");
            if first_count == 0 {
                // println!("{is}- Consumed expected number of hashes");
                let new_counts = &counts[1..];
                nr_arrangements(level + 1, &rest_symbols, new_counts, false)
            } else {
                // println!("{is}- Expected more hashes -> 0 match");
                0
            }
        } else {
            // println!("{is}- Continuing in dots");
            nr_arrangements(level + 1, &rest_symbols, counts, false)
        }
    } else if first_symbol == '?' {
        let rest_symbols = String::from(&symbols[1..]);
        let dash_symbol = format!("#{}", &rest_symbols);
        let dot_symbol = format!(".{}", &rest_symbols);
        let hash_arrangements = nr_arrangements(level + 1, &dash_symbol, counts, in_hashes);
        let dot_arrangements = nr_arrangements(level + 1, &dot_symbol, counts, in_hashes);
        let total_arrangements = hash_arrangements + dot_arrangements;
        // println!(
        //     "{is}- First symbol is ?, sum arrangements for # and . => {} + {} = {}",
        //     hash_arrangements, dot_arrangements, total_arrangements
        // );
        // println!(
        //     "{is}- Nr assignments for symbols: {:?} and counts: {:?} => {}",
        //     symbols, counts, total_arrangements
        // );
        total_arrangements
    } else {
        panic!("Unexpected symbol: {}", first_symbol);
    };
    // println!(
    //     "{is}< Nr assignments for symbols: {:?} counts: {:?} in_hashes: {} RESULT: {}",
    //     symbols, counts, in_hashes, result
    // );
    result
}

fn indent_str(level: usize) -> String {
    let mut s = String::new();
    for _ in 0..level * 2 {
        s.push(' ');
    }
    s
}
