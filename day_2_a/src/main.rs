use std::fs::read_to_string;

const MAX_RED: u32 = 12;
const MAX_GREEN: u32 = 13;
const MAX_BLUE: u32 = 14;

fn is_draw_possible(draw: &str) -> bool {
    let mut red: u32 = 0;
    let mut green: u32 = 0;
    let mut blue: u32 = 0;
    for count_color in draw.split(',') {
        let (count, color) = count_color.trim().split_once(' ').unwrap();
        let count: u32 = count.parse().unwrap();
        match color {
            "red" => red += count,
            "green" => green += count,
            "blue" => blue += count,
            _ => panic!("Unknown color: {color}"),
        }
    }
    red <= MAX_RED && green <= MAX_GREEN && blue <= MAX_BLUE
}

fn are_all_draws_possible(draws: &str) -> bool {
        for draw in draws.split(';') {
            if !is_draw_possible(draw) {
                return false;
            }
        }
    true
}

fn main() {
    let mut sum = 0;
    for line in read_to_string("puzzle_input").unwrap().lines() {
        let (game, draws) = line.split_once(':').unwrap();
        assert!(game.starts_with("Game "));
        let game_id: u32 = game[5..].parse().unwrap();
        if are_all_draws_possible(draws) {
            sum += game_id;
        }
    }
    println!("Sum of game IDs: {sum}");
}