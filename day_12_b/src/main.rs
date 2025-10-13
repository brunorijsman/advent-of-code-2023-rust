use std::fs::read_to_string;

fn main() {
    let mut total = 0;
    for record in read_to_string("example_input").unwrap().lines() {
        total += nr_possible_arrangements(record);
    }
    println!("Total number of possible arrangements: {}", total);
}

fn nr_possible_arrangements(record: &str) -> u64 {
    let (short_symbols, counts) = record.split_once(' ').unwrap();
    let short_symbols = String::from(short_symbols);
    // let symbols = format!("{}{}{}", short_symbols, short_symbols, short_symbols);
    let symbols = format!("{}", short_symbols);
    let symbol_groups = symbols
        .split('.')
        .filter(|s| !s.is_empty())
        .map(|s| String::from(s))
        .collect();
    let short_counts: Vec<u32> = counts
        .split(',')
        .map(|s| s.parse::<u32>().unwrap())
        .collect();
    // let counts = [&short_counts[..], &short_counts[..], &short_counts[..]].concat();
    let counts = [&short_counts[..]].concat();
    println!(
        "Counting arrangements for symbol groups: {:?} and counts: {:?}",
        symbol_groups, counts
    );
    let nr_arrangements = total_nr_assignments(&symbol_groups, &counts);
    println!("Found {} arrangements", nr_arrangements);
    nr_arrangements
}

fn total_nr_assignments(symbol_groups: &Vec<String>, counts: &[u32]) -> u64 {
    if symbol_groups.is_empty() {
        if counts.is_empty() {
            return 1;
        } else {
            return 0;
        }
    }
    let first_symbol_group = symbol_groups[0].clone();
    let rest_symbol_groups = &symbol_groups[1..].to_vec();
    let mut total_nr_assignements = 0;
    for split_pos in 0..counts.len() {
        let match_counts = counts[0..split_pos].to_vec();
        let rest_counts = counts[split_pos..].to_vec();
        let first_nr_assignments = symbol_group_nr_assignments(&first_symbol_group, &match_counts);
        if first_nr_assignments > 0 {
            let rest_nr_assignments = total_nr_assignments(rest_symbol_groups, &rest_counts);
            println!(
                "first_nr_assignments: {}, rest_nr_assignments: {}",
                first_nr_assignments, rest_nr_assignments
            );
            total_nr_assignements += first_nr_assignments * rest_nr_assignments;
        }
    }
    total_nr_assignements
}

fn symbol_group_nr_assignments(symbol_group: &String, counts: &[u32]) -> u64 {
    println!(
        "Assignments for group {:?} counts {:?}",
        symbol_group, counts
    );
    // Empty counts list => only match if symbol group is all '?'
    if counts.is_empty() {
        if symbol_group.chars().all(|c| c == '?') {
            println!(
                "- Assignments for symbol group: {} and counts: {:?} => ONE",
                symbol_group, counts
            );
            return 1;
        } else {
            println!(
                "- Assignments for symbol group: {} and counts: {:?} => ZERO",
                symbol_group, counts
            );
            return 0;
        }
    }
    // Consume the number of hashes needed to satisfy the first count in the count vector.
    // Remember that the symbol group (by definition) consists of only '#' and '?'
    let need_hashes: u32 = counts[0];
    if symbol_group.len() < need_hashes as usize {
        return 0;
    }
    let rest_symbol_group = String::from(&symbol_group[need_hashes as usize..]);
    let rest_counts = &counts[1..];
    let nr_assignments = symbol_group_nr_assignments(&rest_symbol_group, rest_counts);
    println!(
        "- Assignments for symbol group: {} and counts: {:?} => {}",
        symbol_group, counts, nr_assignments
    );
    nr_assignments
}
