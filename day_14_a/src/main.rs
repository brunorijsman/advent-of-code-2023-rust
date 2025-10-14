use std::fs::read_to_string;

type Map = Vec<Vec<char>>;

fn main() {
    let mut map = read_map();
    roll_rocks_in_map(&mut map);
    let load = compute_load_of_map(&map);
    println!("Load of map: {}", load);
}

fn read_map() -> Map {
    let input = read_to_string("puzzle_input").unwrap();
    let map: Map = input.lines().map(|line| line.chars().collect()).collect();
    map
}

fn roll_rocks_in_map(map: &mut Map) {
    let cols = map[0].len();
    for col in 0..cols {
        roll_rocks_in_col(map, col);
    }
}

fn roll_rocks_in_col(map: &mut Map, col: usize) {
    let rows = map[0].len();
    let mut roll_to_row = 0;
    for row in 0..rows {
        match map[row][col] {
            '#' => roll_to_row = row + 1,
            'O' => {
                if row != roll_to_row {
                    map[roll_to_row][col] = 'O';
                    map[row][col] = '.';
                }
                roll_to_row = roll_to_row + 1;
            }
            '.' => (),
            _ => panic!("Unexpected character in map"),
        }
    }
}

fn compute_load_of_map(map: &Map) -> usize {
    let mut load = 0;
    let nr_rows = map.len();
    for (row_nr, row) in map.iter().enumerate() {
        for c in row {
            if *c == 'O' {
                load += (nr_rows - row_nr) as usize;
            }
        }
    }
    load
}
