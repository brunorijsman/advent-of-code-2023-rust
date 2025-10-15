use std::fs::read_to_string;

type Map = Vec<Vec<char>>;

// A position on the map is represented as a tuple (x, y).
// (0, 0) is the top-left (North-West) corner of the map.
// The coordinates are isize instead of usize to allow for negative positions which occur
// temporarily before we realize that we have moved off the map.
type Pos = (isize, isize);

// A direction is represented as a tuple (dx, dy).
// In this representation, (0, 1) is North, (0, -1) is South, (1, 0) is East, and (-1, 0) is West.
type Dir = (isize, isize);

// The visited map is a 2D grid, where each cell contains a list of directions.
// Each direction indicates that the cell has been visited coming from that direction.
type VisitedMap = Vec<Vec<Vec<Dir>>>;

fn main() {
    let map = read_map();
    // print_map(&map);
    let max_energized_count = try_all_starting_points(&map);
    println!("Max energized cells count: {}", max_energized_count);
}

fn try_all_starting_points(map: &Map) -> usize {
    let mut max_energized_count = 0;
    // Northern edge (going South)
    for x in 0..map[0].len() {
        let energized_count = try_one_starting_point(map, (x as isize, 0), (0, 1));
        max_energized_count = max_energized_count.max(energized_count);
    }
    // Southern edge (going North)
    for x in 0..map[0].len() {
        let energized_count =
            try_one_starting_point(map, (x as isize, (map.len() - 1) as isize), (0, -1));
        max_energized_count = max_energized_count.max(energized_count);
    }
    // Western edge (going East)
    for y in 0..map.len() {
        let energized_count = try_one_starting_point(map, (0, y as isize), (1, 0));
        max_energized_count = max_energized_count.max(energized_count);
    }
    // Eastern edge (going West)
    for y in 0..map.len() {
        let energized_count =
            try_one_starting_point(map, ((map[0].len() - 1) as isize, y as isize), (-1, 0));
        max_energized_count = max_energized_count.max(energized_count);
    }
    max_energized_count
}

fn try_one_starting_point(map: &Map, start_pos: Pos, start_dir: Dir) -> usize {
    let mut visited_map = new_visited_map(map);
    follow_laser(map, &mut visited_map, &start_pos, &start_dir);
    let energized_count = energized_cells_count(&visited_map);
    energized_count
}

fn read_map() -> Map {
    read_to_string("puzzle_input")
        .unwrap()
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}

#[allow(dead_code)]
fn print_map(map: &Map) {
    for row in map {
        for &cell in row {
            print!("{}", cell);
        }
        println!();
    }
}

#[allow(dead_code)]
fn print_energized_map(visited_map: &VisitedMap) {
    for row in visited_map {
        for cell in row {
            if cell.is_empty() {
                print!(".");
            } else {
                print!("#");
            }
        }
        println!();
    }
}

fn energized_cells_count(visited_map: &VisitedMap) -> usize {
    visited_map
        .iter()
        .map(|row| row.iter().filter(|cell| !cell.is_empty()).count())
        .sum()
}

#[allow(dead_code)]
fn to_str(dir: &Dir) -> &str {
    match dir {
        (0, 1) => "S",
        (0, -1) => "N",
        (1, 0) => "E",
        (-1, 0) => "W",
        _ => panic!("Invalid direction"),
    }
}

fn new_visited_map(for_map: &Map) -> VisitedMap {
    let height = for_map.len();
    let width = for_map[0].len();
    vec![vec![Vec::new(); width]; height]
}

fn follow_laser(map: &Map, visited_map: &mut VisitedMap, pos: &Pos, dir: &Dir) {
    // println!(
    //     "Following laser at pos {:?} going in direction {}",
    //     pos,
    //     to_str(dir)
    // );
    // Do off map check here to avoid doing it in every follow_laser_in_* function
    if !pos_is_on_map(map, pos) {
        return;
    }
    if has_already_been_visited(visited_map, pos, dir) {
        return;
    }
    mark_as_visited(visited_map, pos, dir);
    let (x, y) = *pos;
    match map[y as usize][x as usize] {
        '.' => follow_laser_in_empty_space(map, visited_map, pos, dir),
        '/' => follow_laser_in_bottom_left_to_top_right_mirror(map, visited_map, pos, dir),
        '\\' => follow_laser_in_top_left_to_bottom_right_mirror(map, visited_map, pos, dir),
        '|' => follow_laser_in_vertical_splitter(map, visited_map, pos, dir),
        '-' => follow_laser_in_horizontal_splitter(map, visited_map, pos, dir),
        _ => panic!("Invalid map character"),
    }
}

fn follow_laser_in_empty_space(map: &Map, visited_map: &mut VisitedMap, pos: &Pos, dir: &Dir) {
    let (dx, dy) = *dir;
    let new_pos = (pos.0 + dx, pos.1 + dy);
    follow_laser(map, visited_map, &new_pos, dir);
}

fn follow_laser_in_bottom_left_to_top_right_mirror(
    map: &Map,
    visited_map: &mut VisitedMap,
    pos: &Pos,
    dir: &Dir,
) {
    let (dx, dy) = *dir;
    let new_dir = (-dy, -dx); // Reflect the direction
    let new_pos = (pos.0 + new_dir.0, pos.1 + new_dir.1);
    follow_laser(map, visited_map, &new_pos, &new_dir);
}

fn follow_laser_in_top_left_to_bottom_right_mirror(
    map: &Map,
    visited_map: &mut VisitedMap,
    pos: &Pos,
    dir: &Dir,
) {
    let (dx, dy) = *dir;
    let new_dir = (dy, dx); // Reflect the direction
    let new_pos = (pos.0 + new_dir.0, pos.1 + new_dir.1);
    follow_laser(map, visited_map, &new_pos, &new_dir);
}

fn follow_laser_in_vertical_splitter(
    map: &Map,
    visited_map: &mut VisitedMap,
    pos: &Pos,
    dir: &Dir,
) {
    let (dx, dy) = *dir;
    if dy == 0 {
        // Coming from East or West, split into North and South
        let new_pos_north = (pos.0, pos.1 + 1);
        let dir_north = (0, 1);
        follow_laser(map, visited_map, &new_pos_north, &dir_north);
        let new_pos_south = (pos.0, pos.1 - 1);
        let dir_south = (0, -1);
        follow_laser(map, visited_map, &new_pos_south, &dir_south);
    } else {
        // Coming from North or South, continue in the same direction
        let new_pos = (pos.0 + dx, pos.1 + dy);
        follow_laser(map, visited_map, &new_pos, dir);
    }
}

fn follow_laser_in_horizontal_splitter(
    map: &Map,
    visited_map: &mut VisitedMap,
    pos: &Pos,
    dir: &Dir,
) {
    let (dx, dy) = *dir;
    if dx == 0 {
        // Coming from North or South, split into East and West
        let new_pos_east = (pos.0 + 1, pos.1);
        let dir_east = (1, 0);
        follow_laser(map, visited_map, &new_pos_east, &dir_east);
        let new_pos_west = (pos.0 - 1, pos.1);
        let dir_west = (-1, 0);
        follow_laser(map, visited_map, &new_pos_west, &dir_west);
    } else {
        // Coming from East or West, continue in the same direction
        let new_pos = (pos.0 + dx, pos.1 + dy);
        follow_laser(map, visited_map, &new_pos, dir);
    }
}

fn has_already_been_visited(visited_map: &VisitedMap, pos: &Pos, dir: &Dir) -> bool {
    let (x, y) = *pos;
    visited_map[y as usize][x as usize].contains(dir)
}

fn mark_as_visited(visited_map: &mut VisitedMap, pos: &Pos, dir: &Dir) {
    let (x, y) = *pos;
    let x = x as usize;
    let y = y as usize;
    assert!(!visited_map[y][x].contains(dir));
    visited_map[y][x].push(*dir);
}

fn pos_is_on_map(map: &Map, pos: &Pos) -> bool {
    let (x, y) = *pos;
    let height = map.len() as isize;
    let width = map[0].len() as isize;
    x >= 0 && x < width && y >= 0 && y < height
}
