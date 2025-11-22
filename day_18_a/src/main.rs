use std::fs::read_to_string;

type Plan = Vec<Step>;

#[derive(Debug, Hash)]
struct Step {
    direction: Direction,
    length: usize,
    color: String,
}

#[derive(Debug, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
struct Extent {
    min_x: isize,
    max_x: isize,
    min_y: isize,
    max_y: isize,
}

type Map = Vec<Vec<bool>>;

fn main() {
    let plan = read_plan();
    let mut map = make_map(&plan);
    flood_fill_interior(&mut map);
    let volume = measure_volume(&map);
    println!("Volume: {}", volume);
}

fn read_plan() -> Plan {
    let mut plan = Vec::new();
    let lines = read_to_string("puzzle_input").unwrap();
    for line in lines.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        assert!(parts.len() == 3);
        let direction = match parts[0] {
            "U" => Direction::Up,
            "D" => Direction::Down,
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => panic!("Unknown direction"),
        };
        let length: usize = parts[1].parse().unwrap();
        let color = parts[2].to_string();
        let step = Step {
            direction,
            length,
            color,
        };
        plan.push(step);
    }
    plan
}

fn make_map(plan: &Plan) -> Map {
    let extent = determine_extent(plan);
    let size_x = extent.max_x - extent.min_x + 1;
    let size_y = extent.max_y - extent.min_y + 1;
    let mut map = empty_map(size_x as usize, size_y as usize);
    let mut x = -extent.min_x;
    let mut y = -extent.min_y;
    for step in plan {
        match step.direction {
            Direction::Up => {
                for _ in 0..step.length {
                    y += 1;
                    map[y as usize][x as usize] = true;
                }
            }
            Direction::Down => {
                for _ in 0..step.length {
                    y -= 1;
                    map[y as usize][x as usize] = true;
                }
            }
            Direction::Left => {
                for _ in 0..step.length {
                    x -= 1;
                    map[y as usize][x as usize] = true;
                }
            }
            Direction::Right => {
                for _ in 0..step.length {
                    x += 1;
                    map[y as usize][x as usize] = true;
                }
            }
        }
    }
    map
}

fn determine_extent(plan: &Plan) -> Extent {
    let mut x: isize = 0;
    let mut y: isize = 0;
    let mut min_x: isize = 0;
    let mut max_x: isize = 0;
    let mut min_y: isize = 0;
    let mut max_y: isize = 0;
    for step in plan {
        match step.direction {
            Direction::Up => {
                y += step.length as isize;
                if y > max_y {
                    max_y = y;
                }
            }
            Direction::Down => {
                y -= step.length as isize;
                if y < min_y {
                    min_y = y;
                }
            }
            Direction::Left => {
                x -= step.length as isize;
                if x < min_x {
                    min_x = x;
                }
            }
            Direction::Right => {
                x += step.length as isize;
                if x > max_x {
                    max_x = x;
                }
            }
        }
    }
    Extent {
        min_x,
        max_x,
        min_y,
        max_y,
    }
}

fn empty_map(size_x: usize, size_y: usize) -> Map {
    vec![vec![false; size_x]; size_y]
}

fn flood_fill_interior(map: &mut Map) {
    let exterior = flood_fill_exterior(map);
    let size_y = map.len();
    let size_x = map[0].len();
    for y in 0..size_y {
        for x in 0..size_x {
            if !exterior[y][x] && !map[y][x] {
                map[y][x] = true;
            }
        }
    }
}

fn flood_fill_exterior(map: &Map) -> Map {
    let size_y = map.len();
    let size_x = map[0].len();
    let mut exterior = empty_map(size_x, size_y);
    let mut stack = Vec::new();
    for x in 0..size_x {
        stack.push((x, 0usize));
        stack.push((x, size_y - 1));
    }
    for y in 0..size_y {
        stack.push((0usize, y));
        stack.push((size_x - 1, y));
    }
    while let Some((x, y)) = stack.pop() {
        if x >= size_x || y >= size_y {
            continue;
        }
        if exterior[y][x] {
            continue;
        }
        if map[y][x] {
            continue;
        }
        exterior[y][x] = true;
        if x > 0 {
            stack.push((x - 1, y));
        }
        if x + 1 < size_x {
            stack.push((x + 1, y));
        }
        if y > 0 {
            stack.push((x, y - 1));
        }
        if y + 1 < size_y {
            stack.push((x, y + 1));
        }
    }
    exterior
}

fn measure_volume(map: &Map) -> usize {
    let mut volume = 0;
    for row in map {
        for &cell in row {
            if cell {
                volume += 1;
            }
        }
    }
    volume
}

#[allow(dead_code)]
fn print_map(map: &Map) {
    for row in map {
        for &cell in row {
            if cell {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}
