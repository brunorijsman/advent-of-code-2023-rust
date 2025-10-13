use std::fs::read_to_string;

struct Universe {
    max_x: usize,
    max_y: usize,
    galaxies: Vec<(usize, usize)>,
}

fn main() {
    let mut universe = read_universe();
    expand_universe(&mut universe);
    let total = sum_distances(&universe);
    println!("Total distance: {}", total);
}

fn read_universe() -> Universe {
    let mut galaxies = Vec::new();
    let mut y = 0;
    let mut max_x = 0;
    for line in read_to_string("puzzle_input").unwrap().lines() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                galaxies.push((x, y));
                max_x = std::cmp::max(x, max_x);
            }
        }
        y += 1;
    }
    Universe {
        galaxies,
        max_x,
        max_y: y,
    }
}

fn expand_universe(universe: &mut Universe) {
    let ec = empty_columns(universe);
    let er = empty_rows(universe);
    let grow = 999_999;
    for x in ec.iter().rev() {
        for galaxy in universe.galaxies.iter_mut() {
            if galaxy.0 >= *x {
                galaxy.0 += grow;
            }
        }
        universe.max_x += grow;
    }
    for y in er.iter().rev() {
        for galaxy in universe.galaxies.iter_mut() {
            if galaxy.1 >= *y {
                galaxy.1 += grow;
            }
        }
        universe.max_y += grow;
    }
}

fn empty_columns(universe: &Universe) -> Vec<usize> {
    let mut empty_cols = Vec::new();
    for x in 0..=universe.max_x {
        let mut is_empty = true;
        for y in 0..universe.max_y {
            if universe.galaxies.contains(&(x, y)) {
                is_empty = false;
                break;
            }
        }
        if is_empty {
            empty_cols.push(x);
        }
    }
    empty_cols
}

fn empty_rows(universe: &Universe) -> Vec<usize> {
    let mut empty_rows = Vec::new();
    for y in 0..universe.max_y {
        let mut is_empty = true;
        for x in 0..=universe.max_x {
            if universe.galaxies.contains(&(x, y)) {
                is_empty = false;
                break;
            }
        }
        if is_empty {
            empty_rows.push(y);
        }
    }
    empty_rows
}

fn sum_distances(universe: &Universe) -> usize {
    let mut total = 0;
    let nr_galaxies = universe.galaxies.len();
    for i in 0..nr_galaxies {
        for j in (i + 1)..nr_galaxies {
            let (x1, y1) = universe.galaxies[i];
            let (x2, y2) = universe.galaxies[j];
            total += (x1 as isize - x2 as isize).abs() as usize;
            total += (y1 as isize - y2 as isize).abs() as usize;
        }
    }
    total
}
