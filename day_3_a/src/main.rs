use std::fs::read_to_string;

type Schematic = Vec<Vec<char>>;
type Span = (usize, usize);


fn read_schematic() -> Schematic {
    let mut schematic = Vec::new();
    for line in read_to_string("puzzle_input").unwrap().lines() {
        let row = line.chars().collect();
        schematic.push(row);
    }
    schematic
}

fn next_number_start(row: &Vec<char>, start_col_index: usize) -> Option<usize> {
    for col_index in start_col_index..row.len() {
        if row[col_index].is_digit(10) {
            return Some(col_index);
        }
    }
    None
}

fn number_end(row: &Vec<char>, start_col_index: usize) -> usize {
    for col_index in start_col_index+1..row.len() {
        if !row[col_index].is_digit(10) {
            return col_index-1;
        }
    }
    row.len()-1
}

fn extract_number_col_spans_from_row(row: &Vec<char>) -> Vec<Span> {
    let mut positions = Vec::new();
    let mut col_index = 0;
    while let Some(start_col_index) = next_number_start(row, col_index) {
        let end_col_index = number_end(row, start_col_index);
        positions.push((start_col_index, end_col_index));
        col_index = end_col_index+1;
    }
    positions
}

fn is_symbol_at(schematic: &Schematic, row_index: usize, col_index: usize) -> bool {
    assert!(row_index < schematic.len());
    assert!(col_index < schematic[row_index].len());
    schematic[row_index][col_index] != '.'
}

fn is_symbol_at_delta(schematic: &Schematic, row_index: usize, row_delta: isize, col_index: usize, col_delta: isize) -> bool {
    if col_index == 0 && col_delta < 0 {
        return false;
    }
    if col_index == schematic[row_index].len() - 1 && col_delta > 0 {
        return false;
    }
    if row_index == 0 && row_delta < 0 {
        return false;
    }
    if row_index == schematic.len() - 1 && row_delta > 0 {
        return false;
    }
    let new_row_index = (row_index as isize + row_delta) as usize;
    let new_col_index = (col_index as isize + col_delta) as usize;
    is_symbol_at(schematic, new_row_index, new_col_index)
}

fn is_adjacent_to_symbol(schematic: &Schematic, row_index: usize, col_span: Span) -> bool {
    let (start_col_index, end_col_index) = col_span;
    for row_delta in -1..=1 {
        if is_symbol_at_delta(schematic, row_index, row_delta, start_col_index, -1) {
            return true;
        }
        if is_symbol_at_delta(schematic, row_index, row_delta, end_col_index, 1) {
            return true;
        }
    }
    for col_index in start_col_index..=end_col_index {
        if is_symbol_at_delta(schematic, row_index, -1, col_index, 0) {
            return true;
        }
        if is_symbol_at_delta(schematic, row_index, 1, col_index, 0) {
            return true;
        }
    }
    false
}

fn number_at_span(schematic: &Schematic, row_index: usize, col_span: Span) -> usize {
    let (start_col_index, end_col_index) = col_span;
    let row = &schematic[row_index];
    let number_str: String = row[start_col_index..=end_col_index].iter().collect();
    number_str.parse().unwrap()
}   

fn sum_of_part_numbers_in_row(schematic: &Schematic, row_index: usize) -> usize {
    let mut sum = 0;
    let row = schematic[row_index].as_ref();
    let col_spans = extract_number_col_spans_from_row(&row);
    for col_span in col_spans {
        if is_adjacent_to_symbol(schematic, row_index, col_span) {
            let part_nr = number_at_span(schematic, row_index, col_span);
            sum += part_nr;
        }
    }
    sum
}


fn sum_of_part_numbers_in_schematic(schematic: &Schematic) -> usize {
    let mut sum = 0;
    for row_index in 0..schematic.len() {
        sum += sum_of_part_numbers_in_row(schematic, row_index);
    }
    sum
}

fn main() {
    let schematic = read_schematic();
    let sum = sum_of_part_numbers_in_schematic(&schematic);
    println!("Sum of part numbers: {sum}");
}