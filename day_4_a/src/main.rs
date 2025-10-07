use std::fs::read_to_string;
use std::collections::HashSet;

fn main() {
    let sum = sum_all_game_scores();
    println!("Sum of all game scores: {sum}");
}

fn sum_all_game_scores() -> usize {
    let mut sum = 0;
    for line in read_to_string("puzzle_input").unwrap().lines() {
        let game_score = game_score(line);
        sum += game_score;
    }
    sum
}

fn game_score(game: &str) -> usize {
    let (_game, numbers) = game.trim().split_once(':').unwrap();
    let (winning_numbers, draw_numbers) = numbers.trim().split_once('|').unwrap();
    let winning_numbers = winning_numbers
        .trim()
        .split_whitespace()
        .map(|n| n.trim().parse::<usize>().unwrap())
        .collect::<HashSet<usize>>();
    let draw_numbers = draw_numbers
        .trim()
        .split_whitespace()
        .map(|n| n.trim().parse::<usize>().unwrap())
        .collect::<HashSet<usize>>();
    let mut winning_numbers_count: usize = 0;
    for number in draw_numbers {
        if winning_numbers.contains(&number) {
            winning_numbers_count += 1;
        }
    }
    if winning_numbers_count > 0 {
        1 << (winning_numbers_count - 1)
    } else {
        0
    }
}
