// Algorithm taken from https://www.reddit.com/r/adventofcode/comments/18luw6q/2023_day_17_a_longform_tutorial_on_day_17/

use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;

type Grid = Vec<Vec<usize>>;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct State {
    x: isize,
    y: isize,
    dx: isize,
    dy: isize,
    distance: usize,
}

fn main() {
    let grid = read_grid();

    let mut state_queues_by_cost: HashMap<usize, Vec<State>> = HashMap::new();
    let mut seen_states: HashSet<State> = HashSet::new();

    // We don't know which way we'll start, so try both
    // The instructions say to ignore the starting cost
    move_and_add_state(
        0,
        0,
        0,
        1,
        0,
        1,
        &grid,
        &mut state_queues_by_cost,
        &mut seen_states,
    );
    move_and_add_state(
        0,
        0,
        0,
        0,
        1,
        1,
        &grid,
        &mut state_queues_by_cost,
        &mut seen_states,
    );

    // Iterate till we find the exit
    loop {
        // Find the horizon of our search, the states with the lowest cost
        // All future states will have at least this value, so we can just pop
        // Note: this assumes all grid values are positive!

        // Get the lowest cost
        let current_cost = state_queues_by_cost.keys().min().unwrap().clone();

        // Get all states at that cost
        let next_states = state_queues_by_cost.remove(&current_cost).unwrap();

        // Process each state
        for state in next_states {
            let State {
                x,
                y,
                dx,
                dy,
                distance,
            } = state;

            // Perform left and right turns
            move_and_add_state(
                current_cost,
                x,
                y,
                dy,
                -dx,
                1,
                &grid,
                &mut state_queues_by_cost,
                &mut seen_states,
            );
            move_and_add_state(
                current_cost,
                x,
                y,
                -dy,
                dx,
                1,
                &grid,
                &mut state_queues_by_cost,
                &mut seen_states,
            );

            // Go straight, if we haven't gone too far already
            if distance < 3 {
                move_and_add_state(
                    current_cost,
                    x,
                    y,
                    dx,
                    dy,
                    distance + 1,
                    &grid,
                    &mut state_queues_by_cost,
                    &mut seen_states,
                );
            }
        }
    }
}

fn read_grid() -> Grid {
    let mut grid = Vec::new();
    let lines = read_to_string("puzzle_input").unwrap();
    for line in lines.lines() {
        let mut row = Vec::new();
        for c in line.chars() {
            let cost: usize = c.to_digit(10).unwrap() as usize;
            row.push(cost);
        }
        grid.push(row);
    }
    grid
}

fn move_and_add_state(
    cost: usize,
    x: isize,
    y: isize,
    dx: isize,
    dy: isize,
    distance: usize,
    grid: &Grid,
    state_queues_by_cost: &mut HashMap<usize, Vec<State>>,
    seen_states: &mut HashSet<State>,
) {
    // Update the position
    let x = x + dx;
    let y = y + dy;

    // Bounds checking
    if x < 0 || y < 0 {
        return;
    }
    let height = grid.len() as isize;
    let width = grid[0].len() as isize;
    if x >= width || y >= height {
        return;
    }

    // Calculate the cost of stepping on this square
    let new_cost = cost + grid[y as usize][x as usize];

    // Did we find the end?
    let end_x = width - 1;
    let end_y = height - 1;
    if x == end_x && y == end_y {
        println!("Found end with cost {}", new_cost);
        std::process::exit(0);
    }

    // Create the state
    let state = State {
        x,
        y,
        dx,
        dy,
        distance,
    };

    // Have we seen this state before?
    if !seen_states.contains(&state) {
        // Save the state to visit later
        state_queues_by_cost
            .entry(new_cost)
            .or_insert(Vec::new())
            .push(state);

        // Mark the state as seen
        seen_states.insert(state);
    }
}
