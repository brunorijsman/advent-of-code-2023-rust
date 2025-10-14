use std::collections::HashMap;
use std::fs::read_to_string;

type Map = Vec<Vec<char>>;

#[derive(Clone, Copy, Eq, PartialEq)]
enum Direction {
    North,
    South,
    East,
    West,
}

fn main() {
    let mut map = read_map();
    map = roll_rocks_many_cycles(map, 1_000_000_000);
    let load = compute_load_of_map(&map);
    println!("\nLoad of map: {}", load);
}

fn read_map() -> Map {
    let input = read_to_string("puzzle_input").unwrap();
    let map: Map = input.lines().map(|line| line.chars().collect()).collect();
    map
}

fn roll_rocks_cycle(map: Map) -> Map {
    let mut map = map.clone();
    roll_rocks_in_direction(&mut map, Direction::North);
    roll_rocks_in_direction(&mut map, Direction::West);
    roll_rocks_in_direction(&mut map, Direction::South);
    roll_rocks_in_direction(&mut map, Direction::East);
    map
}

fn roll_rocks_in_direction(map: &mut Map, direction: Direction) {
    match direction {
        Direction::North | Direction::South => roll_rocks_north_or_south(map, direction),
        Direction::East | Direction::West => roll_rocks_east_or_west(map, direction),
    }
}

fn roll_rocks_north_or_south(map: &mut Map, direction: Direction) {
    let nr_columns = map[0].len();
    for column_index in 0..nr_columns {
        match direction {
            Direction::North => roll_rocks_north_in_column(map, column_index),
            Direction::South => roll_rocks_south_in_column(map, column_index),
            _ => panic!("Unexpected direction"),
        }
    }
}

fn roll_rocks_north_in_column(map: &mut Map, column_index: usize) {
    let nr_rows = map[0].len();
    let mut roll_to_row_index = 0;
    for row_index in 0..nr_rows {
        match map[row_index][column_index] {
            '#' => roll_to_row_index = row_index + 1,
            'O' => {
                if row_index != roll_to_row_index {
                    map[roll_to_row_index][column_index] = 'O';
                    map[row_index][column_index] = '.';
                }
                roll_to_row_index = roll_to_row_index + 1;
            }
            '.' => (),
            _ => panic!("Unexpected character in map"),
        }
    }
}

fn roll_rocks_south_in_column(map: &mut Map, column_index: usize) {
    let nr_rows = map[0].len();
    let mut roll_to_row_index = nr_rows - 1;
    for row_index in (0..nr_rows).rev() {
        match map[row_index][column_index] {
            '#' => {
                if row_index > 0 {
                    roll_to_row_index = row_index - 1
                }
            }
            'O' => {
                if row_index != roll_to_row_index {
                    map[roll_to_row_index][column_index] = 'O';
                    map[row_index][column_index] = '.';
                }
                if roll_to_row_index > 0 {
                    roll_to_row_index = roll_to_row_index - 1;
                }
            }
            '.' => (),
            _ => panic!("Unexpected character in map"),
        }
    }
}

fn roll_rocks_east_or_west(map: &mut Map, direction: Direction) {
    let nr_rows = map.len();
    for row_index in 0..nr_rows {
        match direction {
            Direction::East => roll_rocks_east_in_row(map, row_index),
            Direction::West => roll_rocks_west_in_row(map, row_index),
            _ => panic!("Unexpected direction"),
        }
    }
}

fn roll_rocks_east_in_row(map: &mut Map, row_index: usize) {
    let nr_columns = map.len();
    let mut roll_to_column_index = nr_columns - 1;
    for column_index in (0..nr_columns).rev() {
        match map[row_index][column_index] {
            '#' => {
                if column_index > 0 {
                    roll_to_column_index = column_index - 1
                }
            }
            'O' => {
                if column_index != roll_to_column_index {
                    map[row_index][roll_to_column_index] = 'O';
                    map[row_index][column_index] = '.';
                }
                if roll_to_column_index > 0 {
                    roll_to_column_index = roll_to_column_index - 1;
                }
            }
            '.' => (),
            _ => panic!("Unexpected character in map"),
        }
    }
}

fn roll_rocks_west_in_row(map: &mut Map, row_index: usize) {
    let nr_columns = map.len();
    let mut roll_to_column_index = 0;
    for column_index in 0..nr_columns {
        match map[row_index][column_index] {
            '#' => roll_to_column_index = column_index + 1,
            'O' => {
                if column_index != roll_to_column_index {
                    map[row_index][roll_to_column_index] = 'O';
                    map[row_index][column_index] = '.';
                }
                roll_to_column_index = roll_to_column_index + 1;
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

fn roll_rocks_many_cycles(map: Map, nr_cycles: usize) -> Map {
    let mut map = map.clone();
    let mut seen_maps = HashMap::<Map, Vec<usize>>::new();
    let mut cycle_nr = 0;
    while cycle_nr < nr_cycles {
        match seen_maps.get_mut(&map) {
            Some(seen_in_cycles) => {
                seen_in_cycles.push(cycle_nr);
                for previous_cycle_nr in seen_in_cycles {
                    let cycle_length = cycle_nr - *previous_cycle_nr;
                    if cycle_nr + cycle_length < nr_cycles - 1 {
                        cycle_nr += cycle_length;
                        break;
                    }
                }
            }
            None => {
                let seen_in_cycles = vec![cycle_nr];
                seen_maps.insert(map.clone(), seen_in_cycles);
            }
        }
        map = roll_rocks_cycle(map);
        cycle_nr += 1;
    }
    map
}
