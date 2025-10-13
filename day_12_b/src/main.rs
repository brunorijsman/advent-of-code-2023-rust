use std::fs::read_to_string;

fn main() {
    let mut total = 0;
    for record in read_to_string("puzzle_input").unwrap().lines() {
        total += nr_possible_arrangements(record);
    }
    println!("Total number of possible arrangements: {}", total);
}

fn nr_possible_arrangements(record: &str) -> u32 {
    let (short_symbols, counts) = record.split_once(' ').unwrap();
    let short_symbols = String::from(short_symbols);
    let symbols = format!("{}{}{}", short_symbols, short_symbols, short_symbols);
    let short_counts: Vec<u32> = counts
        .split(',')
        .map(|s| s.parse::<u32>().unwrap())
        .collect();
    let counts = [&short_counts[..], &short_counts[..], &short_counts[..]].concat();
    let total_count = counts.iter().sum::<u32>();
    println!(
        "Counting arrangements for symbols: {} and counts: {:?}",
        symbols, counts
    );
    let nr_arrangements = count_arrangements(&symbols, &counts, total_count);
    println!("Found {} arrangements", nr_arrangements);
    nr_arrangements
}

fn count_arrangements(symbols: &String, counts: &[u32], total_count: u32) -> u32 {
    let is_match = symbols_can_match_counts(symbols, counts, total_count);
    if is_match == Some(false) {
        return 0;
    }
    let pos = symbols.find('?');
    match pos {
        Some(pos) => {
            let mut symbols = symbols.clone();
            symbols.replace_range(pos..=pos, "#");
            let hash_arrangements = count_arrangements(&symbols, counts, total_count);
            symbols.replace_range(pos..=pos, ".");
            let dot_arrangements = count_arrangements(&symbols, counts, total_count);
            hash_arrangements + dot_arrangements
        }
        None => match is_match {
            Some(true) => 1,
            Some(false) => 0,
            None => panic!("Should not happen"),
        },
    }
}

fn symbols_can_match_counts(symbols: &str, counts: &[u32], total_count: u32) -> Option<bool> {
    if !total_chars_count_can_match(symbols, total_count) {
        return Some(false);
    }
    if !total_group_count_can_match(symbols, counts) {
        return Some(false);
    }
    groups_can_match(symbols, counts)
}

fn total_chars_count_can_match(symbols: &str, total_count: u32) -> bool {
    let hash_count = symbols.chars().filter(|&c| c == '#').count() as u32;
    let question_count = symbols.chars().filter(|&c| c == '?').count() as u32;
    let min_count = hash_count;
    let max_count = hash_count + question_count;
    if total_count < min_count || total_count > max_count {
        return false;
    }
    true
}

fn total_group_count_can_match(symbols: &str, counts: &[u32]) -> bool {
    let mut min_nr_groups = 0;
    let mut max_nr_groups = 0;
    let mut in_group = false;
    let mut hashes_in_group = 0;
    let mut questions_in_group = 0;
    for c in symbols.chars() {
        if c == '#' {
            if in_group {
                hashes_in_group += 1;
            } else {
                in_group = true;
                hashes_in_group = 1;
                questions_in_group = 0;
            }
        } else if c == '?' {
            if in_group {
                questions_in_group += 1;
            } else {
                in_group = true;
                hashes_in_group = 0;
                questions_in_group = 1;
            }
        } else if c == '.' {
            if in_group {
                if hashes_in_group > 0 {
                    min_nr_groups += 1;
                    max_nr_groups += 1;
                }
                max_nr_groups += (questions_in_group + 1) / 2;
                in_group = false;
            }
        }
    }
    if in_group {
        if in_group {
            if hashes_in_group > 0 {
                min_nr_groups += 1;
                max_nr_groups += 1;
            }
            max_nr_groups += (questions_in_group + 1) / 2;
        }
    }
    let expected_group_count = counts.len() as u32;
    if expected_group_count < min_nr_groups || expected_group_count > max_nr_groups {
        return false;
    }
    true
}

fn groups_can_match(symbols: &str, counts: &[u32]) -> Option<bool> {
    let mut actual_counts = Vec::new();
    let mut in_group = false;
    let mut min_group_count = 0;
    let mut max_group_count = 0;
    for c in symbols.chars() {
        if c == '#' {
            if in_group {
                min_group_count += 1;
                max_group_count += 1;
            } else {
                in_group = true;
                min_group_count = 1;
                max_group_count = 1;
            }
        } else if c == '?' {
            if in_group {
                max_group_count += 1;
            } else {
                in_group = true;
                min_group_count = 0;
                max_group_count = 1;
            }
        } else if c == '.' {
            if in_group {
                actual_counts.push((min_group_count, max_group_count));
                in_group = false;
            }
        }
    }
    if in_group {
        actual_counts.push((min_group_count, max_group_count));
    }
    for ((min_count, max_count), &expected_count) in actual_counts.iter().zip(counts.iter()) {
        if *min_count == 0 {
            // We ran into a group with only question marks (????). We have not implemented the logic
            // to handle this case, so we return None to indicate that we cannot determine if the
            // symbols can match the counts.
            return None;
        }
        if expected_count < *min_count || expected_count > *max_count {
            return Some(false);
        }
    }
    Some(true)
}
