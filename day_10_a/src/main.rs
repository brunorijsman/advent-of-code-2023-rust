use core::panic;
use std::fs::read_to_string;

fn main() {
    let map = read_map();
    let animal_pos = locate_animal(&map);
    let (pipe_pos_a, pipe_pos_b) = locate_pipe_starts_from_animal(&map, animal_pos);
    let distance = follow_two_pipes_until_they_meet(&map, animal_pos, pipe_pos_a, pipe_pos_b);
    println!("Maximum distance in pipe from animal is {distance}");
}

fn read_map() -> Vec<Vec<char>> {
    let mut map = Vec::new();
    for line in read_to_string("puzzle_input").unwrap().lines() {
        let mut row = Vec::new();
        for c in line.chars() {
            row.push(c);
        }
        map.push(row);
    }
    map
}

fn locate_animal(map: &Vec<Vec<char>>) -> (usize, usize) {
    for (y, row) in map.iter().enumerate() {
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
    map: &Vec<Vec<char>>,
    animal_pos: (usize, usize),
) -> ((usize, usize), (usize, usize)) {
    // This implementation assumes that there are exactly two positions around the animal
    // with pipes that connect to the animal. In general, this is not true, but it is true
    // for the provided inputs. If it was not true, it would make the algorithm more complex.
    let mut pipe_starts = Vec::new();
    for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
        let maybe_pos = ((animal_pos.0 as isize + dx), (animal_pos.1 as isize + dy));
        if !is_valid_pos(map, maybe_pos) {
            continue;
        }
        let pos = (maybe_pos.0 as usize, maybe_pos.1 as usize);
        if map[pos.1][pos.0] == '.' {
            continue;
        }
        if pipe_neighbors(map, pos).contains(&animal_pos) {
            pipe_starts.push(pos);
        }
    }
    assert!(pipe_starts.len() == 2, "Expected exactly two pipe starts");
    let a = pipe_starts[0];
    let b = pipe_starts[1];
    (a, b)
}

fn follow_pipe(
    map: &Vec<Vec<char>>,
    enter_pos: (usize, usize),
    pipe_pos: (usize, usize),
) -> (usize, usize) {
    let neighbors = pipe_neighbors(map, pipe_pos);
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

fn pipe_neighbors(map: &Vec<Vec<char>>, pipe_pos: (usize, usize)) -> Vec<(usize, usize)> {
    let mut neighbors = Vec::new();
    let (x, y) = (pipe_pos.0 as isize, pipe_pos.1 as isize);
    let c = map[pipe_pos.1][pipe_pos.0];
    let (maybe_a, maybe_b) = match c {
        '|' => ((x, y - 1), (x, y + 1)),
        '-' => ((x - 1, y), (x + 1, y)),
        'L' => ((x, y - 1), (x + 1, y)),
        'J' => ((x, y - 1), (x - 1, y)),
        '7' => ((x - 1, y), (x, y + 1)),
        'F' => ((x + 1, y), (x, y + 1)),
        _ => panic!("Unexpected pipe character {c} at {pipe_pos:?}"),
    };
    if is_valid_pos(map, maybe_a) {
        let a = (maybe_a.0 as usize, maybe_a.1 as usize);
        neighbors.push(a);
    }
    if is_valid_pos(map, maybe_b) {
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
    let x = maybe_x as usize;
    let y = maybe_y as usize;
    map[y][x] != ' '
}

fn follow_two_pipes_until_they_meet(
    map: &Vec<Vec<char>>,
    start_pos: (usize, usize),
    pipe_pos_a: (usize, usize),
    pipe_pos_b: (usize, usize),
) -> usize {
    let mut pipe_pos_a = pipe_pos_a;
    let mut pipe_pos_b = pipe_pos_b;
    let mut start_pos_a = start_pos;
    let mut start_pos_b = start_pos;
    let mut distance = 0;
    loop {
        let end_pos_a = follow_pipe(map, start_pos_a, pipe_pos_a);
        (start_pos_a, pipe_pos_a) = (pipe_pos_a, end_pos_a);
        let end_pos_b = follow_pipe(map, start_pos_b, pipe_pos_b);
        (start_pos_b, pipe_pos_b) = (pipe_pos_b, end_pos_b);
        distance += 1;
        if start_pos_a == start_pos_b {
            return distance;
        }
    }
}
