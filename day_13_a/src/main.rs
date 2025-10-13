use std::fs::File;
use std::io::{BufRead, BufReader};

type Map = Vec<Vec<char>>;

fn main() {
    let file = File::open("puzzle_input").unwrap();
    let mut reader = BufReader::new(file);
    let mut notes_sum = 0;
    while let Some(map) = read_map(&mut reader) {
        println!("Map\n===\n{:?}\n", map);
        let notes = calculate_map_notes(&map);
        println!("Notes for map: {}\n", notes);
        notes_sum += notes;
    }
    println!("Total notes sum: {}", notes_sum);
}

fn read_map(reader: &mut BufReader<File>) -> Option<Map> {
    let mut map = Vec::new();
    while let Some(line) = next_line(reader) {
        if line.is_empty() {
            break;
        }
        let mut row = Vec::new();
        for c in line.chars() {
            row.push(c)
        }
        map.push(row);
    }
    if map.is_empty() {
        return None;
    }
    Some(map)
}

fn next_line(reader: &mut BufReader<File>) -> Option<String> {
    let mut line = String::new();
    let bytes_read = reader.read_line(&mut line).unwrap();
    if bytes_read == 0 {
        return None;
    }
    line = line.trim().to_string();
    Some(line)
}

fn calculate_map_notes(map: &Map) -> isize {
    for row in 0..(map.len() as isize) - 1 {
        if is_mirror_below_row(map, row) {
            return 100 * (row + 1); // +1 our row indexes are 0-based and the puzzle expects 1-based
        }
    }
    for col in 0..(map[0].len() as isize) - 1 {
        if is_mirror_right_of_col(map, col) {
            return col + 1; // +1 our col indexes are 0-based and the puzzle expects 1-based
        }
    }
    panic!("No mirror found in map");
}

fn is_mirror_below_row(map: &Map, row: isize) -> bool {
    let mut row_1 = row;
    let mut row_2 = row + 1;
    let nr_rows = map.len() as isize;
    while row_1 >= 0 && row_2 < nr_rows {
        if map[row_1 as usize] != map[row_2 as usize] {
            return false;
        }
        row_1 -= 1;
        row_2 += 1;
    }
    true
}

fn is_mirror_right_of_col(map: &Map, col: isize) -> bool {
    let mut col_1 = col;
    let mut col_2 = col + 1;
    let nr_rows = map.len() as isize;
    while col_1 >= 0 && col_2 < map[0].len() as isize {
        for row in 0..nr_rows {
            if map[row as usize][col_1 as usize] != map[row as usize][col_2 as usize] {
                return false;
            }
        }
        col_1 -= 1;
        col_2 += 1;
    }
    true
}
