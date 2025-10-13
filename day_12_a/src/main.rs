use std::fs::read_to_string;

fn main() {
    let mut total = 0;
    for record in read_to_string("puzzle_input").unwrap().lines() {
        total += nr_possible_arrangements(record);
    }
    println!("Total number of possible arrangements: {}", total);
}

fn nr_possible_arrangements(record: &str) -> u32 {
    let (symbols, counts) = record.split_once(' ').unwrap();
    let symbols = String::from(symbols);
    let counts: Vec<u32> = counts
        .split(',')
        .map(|s| s.parse::<u32>().unwrap())
        .collect();
    let nr_arrangements = count_arrangements(&symbols, &counts);
    nr_arrangements
}

fn count_arrangements(symbols: &String, counts: &[u32]) -> u32 {
    let pos = symbols.find('?');
    match pos {
        Some(pos) => {
            let mut symbols = symbols.clone();
            symbols.replace_range(pos..=pos, "#");
            let hash_arrangements = count_arrangements(&symbols, counts);
            symbols.replace_range(pos..=pos, ".");
            let dot_arrangements = count_arrangements(&symbols, counts);
            hash_arrangements + dot_arrangements
        }
        None => {
            if symbols_match_counts(symbols, counts) {
                1
            } else {
                0
            }
        }
    }
}

fn symbols_match_counts(symbols: &str, counts: &[u32]) -> bool {
    let mut actual_counts = Vec::new();
    let mut in_group = false;
    let mut group_count = 0;
    for c in symbols.chars() {
        if c == '#' {
            if in_group {
                group_count += 1;
            } else {
                in_group = true;
                group_count = 1;
            }
        } else if c == '.' {
            if in_group {
                actual_counts.push(group_count);
                in_group = false;
            }
        }
    }
    if in_group {
        actual_counts.push(group_count);
    }
    return actual_counts == counts;
}
