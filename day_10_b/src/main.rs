use core::panic;
use std::fs::read_to_string;

// Different types of maps:
// Symbol map : the original map read from the input file with symbols such as | - L J 7 F
// Pipe map: a map that only contains the symbols for the pipes (same resolution, same symbols)
// Hires map: a map that has true or false for dots present, at triple the resolution of the symbol map

fn main() {
    let mut symbol_map = read_symbol_map();
    let animal_pos = locate_animal(&symbol_map);
    let (pipe_start_pos, pipe_end_pos) = locate_pipe_starts_from_animal(&symbol_map, animal_pos);
    replace_animal_with_pipe(&mut symbol_map, animal_pos, pipe_start_pos, pipe_end_pos);
    let pipe_map = make_pipe_map(&symbol_map, animal_pos, pipe_start_pos, pipe_end_pos);
    let mut hires_map = make_hires_map(&pipe_map);
    flood_outside(&mut hires_map);
    let inside_map = make_inside_map(&hires_map);
    let count = count_inside(&inside_map);
    println!("Count of inside positions: {count}");
}

fn read_symbol_map() -> Vec<Vec<char>> {
    let mut symbol_map = Vec::new();
    for line in read_to_string("puzzle_input").unwrap().lines() {
        let mut row = Vec::new();
        for c in line.chars() {
            row.push(c);
        }
        symbol_map.push(row);
    }
    symbol_map
}

fn locate_animal(symbol_map: &Vec<Vec<char>>) -> (usize, usize) {
    for (y, row) in symbol_map.iter().enumerate() {
        for (x, &c) in row.iter().enumerate() {
            if c == 'S' {
                return (x as usize, y as usize);
            }
        }
    }
    assert!(false, "Animal not found");
    (0, 0)
}

fn locate_pipe_starts_from_animal(
    symbol_map: &Vec<Vec<char>>,
    animal_pos: (usize, usize),
) -> ((usize, usize), (usize, usize)) {
    // This implementation assumes that there are exactly two positions around the animal
    // with pipes that connect to the animal. In general, this is not true, but it is true
    // for the provided inputs. If it was not true, it would make the algorithm more complex.
    let mut pipe_starts = Vec::new();
    for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
        let maybe_pos = ((animal_pos.0 as isize + dx), (animal_pos.1 as isize + dy));
        if !is_valid_pos(symbol_map, maybe_pos) {
            continue;
        }
        let pos = (maybe_pos.0 as usize, maybe_pos.1 as usize);
        if symbol_map[pos.1][pos.0] == '.' {
            continue;
        }
        if pipe_neighbors(symbol_map, pos).contains(&animal_pos) {
            pipe_starts.push(pos);
        }
    }
    assert!(pipe_starts.len() == 2, "Expected exactly two pipe starts");
    let pipe_start_pos = pipe_starts[0];
    let pipe_end_pos = pipe_starts[1];
    (pipe_start_pos, pipe_end_pos)
}

fn follow_pipe(
    symbol_map: &Vec<Vec<char>>,
    enter_pos: (usize, usize),
    pipe_pos: (usize, usize),
) -> (usize, usize) {
    let neighbors = pipe_neighbors(symbol_map, pipe_pos);
    assert!(neighbors.len() == 2, "Pipe goes off-map");
    let exit_pos = if enter_pos == neighbors[0] {
        neighbors[1]
    } else if enter_pos == neighbors[1] {
        neighbors[0]
    } else {
        panic!("Pipe does not connect to start position")
    };
    exit_pos
}

fn pipe_neighbors(symbol_map: &Vec<Vec<char>>, pipe_pos: (usize, usize)) -> Vec<(usize, usize)> {
    let mut neighbors = Vec::new();
    let (x, y) = (pipe_pos.0 as isize, pipe_pos.1 as isize);
    let c = symbol_map[pipe_pos.1][pipe_pos.0];
    let (maybe_a, maybe_b) = match c {
        '|' => ((x, y - 1), (x, y + 1)),
        '-' => ((x - 1, y), (x + 1, y)),
        'L' => ((x, y - 1), (x + 1, y)),
        'J' => ((x, y - 1), (x - 1, y)),
        '7' => ((x - 1, y), (x, y + 1)),
        'F' => ((x + 1, y), (x, y + 1)),
        _ => panic!("Unexpected pipe character {c} at {pipe_pos:?}"),
    };
    if is_valid_pos(symbol_map, maybe_a) {
        let a = (maybe_a.0 as usize, maybe_a.1 as usize);
        neighbors.push(a);
    }
    if is_valid_pos(symbol_map, maybe_b) {
        let b = (maybe_b.0 as usize, maybe_b.1 as usize);
        neighbors.push(b);
    }
    neighbors
}

fn is_valid_pos(map: &Vec<Vec<char>>, maybe_pos: (isize, isize)) -> bool {
    let (maybe_x, maybe_y) = maybe_pos;
    if maybe_y < 0 || maybe_y >= map.len() as isize {
        return false;
    }
    if maybe_x < 0 || maybe_x >= map[maybe_y as usize].len() as isize {
        return false;
    }
    true
}

fn replace_animal_with_pipe(
    symbol_map: &mut Vec<Vec<char>>,
    animal_pos: (usize, usize),
    pipe_start_pos: (usize, usize),
    pipe_end_pos: (usize, usize),
) {
    let (ax, ay) = (animal_pos.0 as isize, animal_pos.1 as isize);
    let (sx, sy) = (pipe_start_pos.0 as isize, pipe_start_pos.1 as isize);
    let (ex, ey) = (pipe_end_pos.0 as isize, pipe_end_pos.1 as isize);
    let pipe_under_animal_char = if (sx == ax && sy == ay - 1 && ex == ax && ey == ay + 1)
        || (ex == ax && ey == ay - 1 && sx == ax && sy == ay + 1)
    {
        '|'
    } else if (sx == ax - 1 && sy == ay && ex == ax + 1 && ey == ay)
        || (ex == ax - 1 && ey == ay && sx == ax + 1 && sy == ay)
    {
        '-'
    } else if (sx == ax && sy == ay - 1 && ex == ax + 1 && ey == ay)
        || (ex == ax && ey == ay - 1 && sx == ax + 1 && sy == ay)
    {
        'L'
    } else if (sx == ax && sy == ay - 1 && ex == ax - 1 && ey == ay)
        || (ex == ax && ey == ay - 1 && sx == ax - 1 && sy == ay)
    {
        'J'
    } else if (sx == ax - 1 && sy == ay && ex == ax && ey == ay + 1)
        || (ex == ax - 1 && ey == ay && sx == ax && sy == ay + 1)
    {
        '7'
    } else if (sx == ax + 1 && sy == ay && ex == ax && ey == ay + 1)
        || (ex == ax + 1 && ey == ay && sx == ax && sy == ay + 1)
    {
        'F'
    } else {
        panic!("Could not determine pipe at animal position");
    };
    symbol_map[ay as usize][ax as usize] = pipe_under_animal_char;
}

fn make_pipe_map(
    symbol_map: &Vec<Vec<char>>,
    start_pos: (usize, usize),
    pipe_start_pos: (usize, usize),
    pipe_end_pos: (usize, usize),
) -> Vec<Vec<char>> {
    let mut pipe_map = make_empty_map(symbol_map.len());
    let mut pipe_pos = pipe_start_pos;
    let mut start_pos = start_pos;
    loop {
        pipe_map[pipe_pos.1][pipe_pos.0] = symbol_map[pipe_pos.1][pipe_pos.0];
        let next_pos = follow_pipe(symbol_map, start_pos, pipe_pos);
        (start_pos, pipe_pos) = (pipe_pos, next_pos);
        if start_pos == pipe_end_pos {
            pipe_map[pipe_pos.1][pipe_pos.0] = symbol_map[pipe_pos.1][pipe_pos.0];
            break;
        }
    }
    pipe_map
}

fn make_hires_map(pipe_map: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    // Create an empty hires map with the three times the dimensions as the pipe map
    let mut hires_map = Vec::new();
    for row in pipe_map {
        let mut hires_row = [Vec::new(), Vec::new(), Vec::new()];
        for c in row {
            let hires_box = match c {
                '|' => [['.', 'X', '.'], ['.', 'X', '.'], ['.', 'X', '.']],
                '-' => [['.', '.', '.'], ['X', 'X', 'X'], ['.', '.', '.']],
                'L' => [['.', 'X', '.'], ['.', 'X', 'X'], ['.', '.', '.']],
                'J' => [['.', 'X', '.'], ['X', 'X', '.'], ['.', '.', '.']],
                '7' => [['.', '.', '.'], ['X', 'X', '.'], ['.', 'X', '.']],
                'F' => [['.', '.', '.'], ['.', 'X', 'X'], ['.', 'X', '.']],
                '.' => [['.', '.', '.'], ['.', '.', '.'], ['.', '.', '.']],
                _ => panic!("Unexpected pipe character {c}"),
            };
            for y in 0..3 {
                for x in 0..3 {
                    hires_row[y].push(hires_box[y][x]);
                }
            }
        }
        for y in 0..3 {
            hires_map.push(hires_row[y].clone());
        }
    }
    hires_map
}

#[allow(dead_code)]
fn print_map(map: &Vec<Vec<char>>) {
    for row in map {
        for &c in row {
            print!("{c}");
        }
        println!();
    }
}

fn flood_outside(map: &mut Vec<Vec<char>>) {
    let mut flood_front = Vec::new();
    flood_front.push((0, 0));
    while !flood_front.is_empty() {
        let pos = flood_front.pop().unwrap();
        let (x, y) = (pos.0 as isize, pos.1 as isize);
        if !is_valid_pos(&map, (x, y)) {
            continue;
        }
        if map[y as usize][x as usize] == 'X' || map[y as usize][x as usize] == 'o' {
            continue;
        }
        map[y as usize][x as usize] = 'o';
        flood_front.push((x - 1, y));
        flood_front.push((x + 1, y));
        flood_front.push((x, y - 1));
        flood_front.push((x, y + 1));
    }
}

fn make_inside_map(hires_map: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let hires_size = hires_map.len();
    assert!(hires_size % 3 == 0);
    let inside_size = hires_size / 3;
    let mut inside_map = make_empty_map(inside_size);
    for inside_y in 0..inside_size {
        for inside_x in 0..inside_size {
            let hires_base_x = inside_x * 3;
            let hires_base_y = inside_y * 3;
            let mut all_inside = true;
            for dy in 0..3 {
                for dx in 0..3 {
                    if hires_map[hires_base_y + dy][hires_base_x + dx] != '.' {
                        all_inside = false;
                    }
                }
            }
            if all_inside {
                inside_map[inside_y][inside_x] = 'I';
            }
        }
    }
    inside_map
}

fn make_empty_map(size: usize) -> Vec<Vec<char>> {
    let mut map = Vec::new();
    for _ in 0..size {
        let mut row = Vec::new();
        for _ in 0..size {
            row.push('.');
        }
        map.push(row);
    }
    map
}

fn count_inside(inside_map: &Vec<Vec<char>>) -> usize {
    let mut count = 0;
    for row in inside_map {
        for &c in row {
            if c == 'I' {
                count += 1;
            }
        }
    }
    count
}
