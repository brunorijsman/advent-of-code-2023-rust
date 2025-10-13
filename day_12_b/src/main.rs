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
    let short_counts: Vec<u32> = counts
        .split(',')
        .map(|s| s.parse::<u32>().unwrap())
        .collect();
    // let counts = [&short_counts[..], &short_counts[..], &short_counts[..]].concat();
    let counts = [&short_counts[..]].concat();
    println!(
        "Counting arrangements for symbols: {:?} and counts: {:?}",
        symbols, counts
    );
    let nr_arrangements = nr_assignments(&symbols, &counts);
    println!("Found {} arrangements", nr_arrangements);
    nr_arrangements
}

fn nr_assignments(symbols: &String, counts: &[u32]) -> u64 {
    println!(
        "Nr assignments for symbols: {:?} and counts: {:?}",
        symbols, counts
    );
    if symbols.is_empty() {
        return if counts.is_empty() {
            println!(" - Symbols is empty and counts is also empty => 1 match");
            1
        } else {
            println!(" - Symbols is empty but counts is not empty => 0 match");
            0
        };
    }
    if counts.is_empty() {
        return if symbols.chars().all(|c| c != '#') {
            println!(" - Counts is empty and remaining symbols are . or ? => 1 match");
            1
        } else {
            println!(" - Counts is empty and remaining symbols have # => 0 match");
            0
        };
    }
    let first_symbol = symbols.chars().next().unwrap();
    if first_symbol == '#' {
        let mut nr_expected_hashes = counts[0];
        assert!(nr_expected_hashes > 0);
        println!(
            " - First symbol is # and nr expected dashes is {}, consume expected #'s",
            nr_expected_hashes
        );
        if symbols.len() < nr_expected_hashes as usize {
            println!("   - Not enough symbols left to consume all # => 0 match");
            return 0;
        }
        let expected_hash_symbols = String::from(&symbols[0..nr_expected_hashes as usize]);
        if !expected_hash_symbols.chars().all(|c| c == '#') {
            println!("   - Not enough # symbols at start to consume all # => 0 match");
            return 0;
        }
        let mut rest_symbols = String::from(&symbols[nr_expected_hashes as usize..]);
        println!(
            "   - Rest symbols after consuming expected #'s: {:?}",
            rest_symbols
        );
        let first_rest_symbol = rest_symbols.chars().next();
        if first_rest_symbol == Some('#') {
            println!("   - Next symbol after consuming # is also # => invalid, 0 match");
            return 0;
        }
        if first_rest_symbol == Some('?') {
            rest_symbols = String::from(&rest_symbols[1..]);
            println!("   - Consume terminating ? after contiguous string of #");
        }
        let rest_counts = &counts[1..];
        return nr_assignments(&rest_symbols, &rest_counts);
    }
    if first_symbol == '.' {
        println!(" - First symbol is ., continue with rest symbols and same counts");
        let rest_symbols = String::from(&symbols[1..]);
        return nr_assignments(&rest_symbols, counts);
    }
    if first_symbol == '?' {
        let rest_symbols = String::from(&symbols[1..]);
        let dash_symbol = format!("#{}", &rest_symbols);
        let dot_symbol = format!(".{}", &rest_symbols);
        let hash_arrangements = nr_assignments(&dash_symbol, counts);
        let dot_arrangements = nr_assignments(&dot_symbol, counts);
        let total_arrangements = hash_arrangements + dot_arrangements;
        println!(
            " - First symbol is ?, sum arrangements for # and . => {} + {} = {}",
            hash_arrangements, dot_arrangements, total_arrangements
        );
        println!(
            " * Nr assignments for symbols: {:?} and counts: {:?} => {}",
            symbols, counts, total_arrangements
        );
        return total_arrangements;
    }
    panic!("Unexpected symbol: {}", first_symbol);
}
