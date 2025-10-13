use core::panic;
use std::fs::File;
use std::io::{BufRead, BufReader};

type Map = Vec<Vec<char>>;

#[derive(Debug)]
struct MirrorInfo {
    after_row: Option<isize>,
    after_col: Option<isize>,
    is_mirror: bool,
    smudge: Option<(isize, isize)>,
}

fn main() {
    let file = File::open("puzzle_input").unwrap();
    let mut reader = BufReader::new(file);
    let mut notes_sum = 0;
    while let Some(map) = read_map(&mut reader) {
        let notes = calculate_map_notes(&map);
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
    let mut mirrors = Vec::new();
    for row in 0..(map.len() as isize) - 1 {
        let mirror_info = is_mirror_after_row(map, row);
        if mirror_info.is_mirror {
            mirrors.push(mirror_info);
        }
    }
    for col in 0..(map[0].len() as isize) - 1 {
        let mirror_info = is_mirror_after_col(map, col);
        if mirror_info.is_mirror {
            mirrors.push(mirror_info);
        }
    }
    // There should be exactly two mirrors
    assert!(mirrors.len() == 2);
    // Exactly one should have a smudge
    let scored_mirror = if mirrors[0].smudge.is_some() {
        assert!(mirrors[1].smudge.is_none());
        &mirrors[0]
    } else {
        assert!(mirrors[1].smudge.is_some());
        &mirrors[1]
    };
    if let Some(after_row) = scored_mirror.after_row {
        return (after_row + 1) * 100;
    }
    if let Some(after_col) = scored_mirror.after_col {
        return after_col + 1;
    }
    panic!("Mirror has neither after_row nor after_col");
}

fn is_mirror_after_row(map: &Map, row: isize) -> MirrorInfo {
    let mut row_1 = row;
    let mut row_2 = row + 1;
    let nr_rows = map.len() as isize;
    let nr_cols = map[0].len() as isize;
    let mut is_mirror = true;
    let mut smudge = None;
    while row_1 >= 0 && row_2 < nr_rows {
        for col in 0..nr_cols {
            if map[row_1 as usize][col as usize] != map[row_2 as usize][col as usize] {
                if smudge.is_none() {
                    smudge = Some((row_1, col));
                } else {
                    is_mirror = false;
                    break;
                }
            }
        }
        row_1 -= 1;
        row_2 += 1;
    }
    return MirrorInfo {
        after_row: Some(row),
        after_col: None,
        is_mirror,
        smudge,
    };
}

fn is_mirror_after_col(map: &Map, col: isize) -> MirrorInfo {
    let mut col_1 = col;
    let mut col_2 = col + 1;
    let nr_rows = map.len() as isize;
    let nr_cols = map[0].len() as isize;
    let mut is_mirror = true;
    let mut smudge = None;
    while col_1 >= 0 && col_2 < nr_cols {
        for row in 0..nr_rows {
            if map[row as usize][col_1 as usize] != map[row as usize][col_2 as usize] {
                if smudge.is_none() {
                    smudge = Some((row, col_1));
                } else {
                    is_mirror = false;
                    break;
                }
            }
        }
        col_1 -= 1;
        col_2 += 1;
    }
    return MirrorInfo {
        after_row: None,
        after_col: Some(col),
        is_mirror,
        smudge,
    };
}
