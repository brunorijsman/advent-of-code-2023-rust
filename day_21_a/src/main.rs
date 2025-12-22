use std::fs::read_to_string;

// A 2D map. Depending on the context, true means "there is a rock" or "the cell is reachable in N steps".
type Map = Vec<Vec<bool>>;

fn main() {
    let (rock_map, mut reachable_map) = read_map();
    let nr_steps = 64;
    for _step in 1..=nr_steps {
        reachable_map = next_reachable(&reachable_map, &rock_map);
    }
    let nr_reachable = reachable_map.iter().flatten().filter(|&&cell| cell).count();
    println!(
        "Number of reachable cells after {} steps: {}",
        nr_steps, nr_reachable
    );
}

fn read_map() -> (Map, Map) {
    let string = read_to_string("puzzle_input").unwrap();
    let lines = string.lines();
    let mut rock_map = Vec::new();
    let mut reachable_map = Vec::new();
    for line in lines {
        rock_map.push(line.chars().map(|c| c == '#').collect());
        reachable_map.push(line.chars().map(|c| c == 'S').collect());
    }
    (rock_map, reachable_map)
}

#[allow(dead_code)]
fn print_map(map: &Map) {
    for row in map {
        for &cell in row {
            print!("{}", if cell { 'X' } else { '.' });
        }
        println!();
    }
}

fn next_reachable(current_reachable: &Map, rock_map: &Map) -> Map {
    let height = current_reachable.len();
    let width = current_reachable[0].len();
    let directions: Vec<(isize, isize)> = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];
    let mut next_reachable = vec![vec![false; width]; height];
    for y in 0..height {
        for x in 0..width {
            if current_reachable[y][x] {
                for dir in &directions {
                    let nx = x as isize + dir.0;
                    if nx < 0 || nx > width as isize {
                        continue;
                    }
                    let ny = y as isize + dir.1;
                    if ny < 0 || ny > height as isize {
                        continue;
                    }
                    if rock_map[ny as usize][nx as usize] {
                        continue;
                    }
                    next_reachable[ny as usize][nx as usize] = true;
                }
            }
        }
    }
    next_reachable
}
