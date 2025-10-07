use std::collections::HashSet;
use std::fs::read_to_string;

type Schematic = Vec<Vec<char>>;
type Span = (usize, usize, usize);  // row_index, start_col_index, end_col_index

fn main() {
    let schematic = read_schematic();
    let sum = sum_of_gear_ratios(&schematic);
    println!("Sum of gear rations: {sum}");
}

fn read_schematic() -> Schematic {
    let mut schematic = Vec::new();
    for line in read_to_string("puzzle_input").unwrap().lines() {
        let row = line.chars().collect();
        schematic.push(row);
    }
    schematic
}

fn sum_of_gear_ratios(schematic: &Schematic) -> usize {
    let mut sum = 0;
    for row_index in 0..schematic.len() {
        for col_index in 0..schematic[row_index].len() {
            if schematic[row_index][col_index] == '*' {
                let numbers = numbers_around_potential_gear(schematic, row_index, col_index);
                if numbers.len() == 2 {
                    let gear_ratio = numbers[0] * numbers[1];
                    sum += gear_ratio;
                }
            }
        }
    }
    sum
}

fn numbers_around_potential_gear(schematic: &Schematic, row_index: usize, col_index: usize) -> Vec<usize> {
    let mut number_spans = HashSet::new();
    for row_delta in -1..=1 {
        for col_delta in -1..=1 {
            if row_delta == 0 && col_delta == 0 {
                continue;
            }
            if !is_digit_at_delta(schematic, row_index, row_delta, col_index, col_delta) {
                continue;
            }   
            let number_span = number_span_for_digit(schematic, row_index, row_delta, col_index, col_delta);
            number_spans.insert(number_span);
        }
    }
    let mut numbers= Vec::new();
    for number_span in number_spans {
        let number = number_at_span(schematic, number_span);
        numbers.push(number);
    }
    numbers

}

fn is_digit_at_delta(schematic: &Schematic, row_index: usize, row_delta: isize, col_index: usize, col_delta: isize) -> bool {
    match char_at_delta(schematic, row_index, row_delta, col_index, col_delta) {
        Some(c) => c.is_digit(10),
        None => false,
    }
}

fn delta_pos(schematic: &Schematic, row_index: usize, row_delta: isize, col_index: usize, col_delta: isize) -> Option<(usize, usize)> {
    let new_row_index = row_index as isize + row_delta;
    if new_row_index < 0 || new_row_index >= schematic.len() as isize {
        return None;
    }
    let new_row_index = new_row_index as usize;
    let new_col_index = col_index as isize + col_delta;
    if new_col_index < 0 || new_col_index >= schematic[new_row_index].len() as isize {
        return None;
    }
    let new_col_index = new_col_index as usize;
    Some((new_row_index, new_col_index))
}

fn char_at_delta(schematic: &Schematic, row_index: usize, row_delta: isize, col_index: usize, col_delta: isize) -> Option<char> {
    match delta_pos(schematic, row_index, row_delta, col_index, col_delta) {
        None => return None,
        Some((new_row_index, new_col_index)) => return Some(schematic[new_row_index][new_col_index]),
    }
}

fn number_span_for_digit(schematic: &Schematic, row_index: usize, row_delta: isize, col_index: usize, col_delta: isize) -> Span {
    let (row_index, col_index) = delta_pos(schematic, row_index, row_delta, col_index, col_delta).unwrap();
    let mut start_delta = -1;
    loop {
        if is_digit_at_delta(schematic, row_index, 0, col_index, start_delta) {
            start_delta -= 1;
        } else {
            start_delta += 1;
            break
        }
    };
    let start_col_index = (col_index as isize + start_delta) as usize;
    let mut end_delta = 1;
    loop {
        if is_digit_at_delta(schematic, row_index, 0, col_index, end_delta) {
            end_delta += 1;
        } else {
            end_delta -= 1;
            break
        }
    };
    let end_col_index = (col_index as isize + end_delta) as usize;
    (row_index, start_col_index, end_col_index)
}

fn number_at_span(schematic: &Schematic, span: Span) -> usize {
    let (row_index, start_col_index, end_col_index) = span;
    let row = &schematic[row_index];
    let number_str: String = row[start_col_index..=end_col_index].iter().collect();
    number_str.parse().unwrap()
}   
