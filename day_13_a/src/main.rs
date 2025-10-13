use std::fs::File;
use std::io::{BufRead, BufReader};

type Map = Vec<Vec<char>>;

fn main() {
    let file = File::open("example_input").unwrap();
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

fn calculate_map_notes(map: &Map) -> usize {
    0
}
