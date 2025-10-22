// Algorithm taken from https://www.reddit.com/r/adventofcode/comments/18luw6q/2023_day_17_a_longform_tutorial_on_day_17/

use std::collections::HashMap;
use std::fs::read_to_string;

type Grid = Vec<Vec<usize>>;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct State {
    x: isize,
    y: isize,
}

fn main() {
    let grid = read_grid();

    let mut state_queues_by_cost: HashMap<usize, Vec<State>> = HashMap::new();
    let mut seen_cost_by_state: HashMap<State, usize> = HashMap::new();

    // Add initial state
    add_state(
        0,
        0,
        0,
        &grid,
        &mut state_queues_by_cost,
        &mut seen_cost_by_state,
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
            let x = state.x as isize;
            let y = state.y as isize;

            add_state(
                current_cost,
                x + 1,
                y,
                &grid,
                &mut state_queues_by_cost,
                &mut seen_cost_by_state,
            );
            add_state(
                current_cost,
                x - 1,
                y,
                &grid,
                &mut state_queues_by_cost,
                &mut seen_cost_by_state,
            );
            add_state(
                current_cost,
                x,
                y + 1,
                &grid,
                &mut state_queues_by_cost,
                &mut seen_cost_by_state,
            );
            add_state(
                current_cost,
                x,
                y - 1,
                &grid,
                &mut state_queues_by_cost,
                &mut seen_cost_by_state,
            );
        }
    }
}

fn read_grid() -> Grid {
    let mut grid = Vec::new();
    let lines = read_to_string("example_input").unwrap();
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

fn add_state(
    cost: usize,
    x: isize,
    y: isize,
    grid: &Grid,
    state_queues_by_cost: &mut HashMap<usize, Vec<State>>,
    seen_cost_by_state: &mut HashMap<State, usize>,
) {
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
    let state = State { x, y };

    // Have we seen this state before?
    if !seen_cost_by_state.contains_key(&state) {
        // Save the state to visit later
        state_queues_by_cost
            .entry(new_cost)
            .or_insert(Vec::new())
            .push(state);

        // Mark the state as seen
        seen_cost_by_state.insert(state, new_cost);
    }
}
