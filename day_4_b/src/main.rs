use std::fs::read_to_string;
use std::collections::{BTreeMap, HashSet};

struct Card {
    score: usize,
    count: usize,
}

type Cards = BTreeMap<usize, Card>; // key is card id

fn main() {
    let mut cards = read_original_cards();
    collect_won_cards(&mut cards);
    let total_cards = cards.values().map(|c| c.count).sum::<usize>();
    println!("Total cards: {}", total_cards);
}

fn read_original_cards() -> Cards {
    let mut cards: Cards = BTreeMap::new();
    for line in read_to_string("puzzle_input").unwrap().lines() {
        let (id, score) = game_id_and_score(line);
        let card = Card { score, count: 1 };
        cards.insert(id, card);
    }
    cards
}

fn game_id_and_score(game: &str) -> (usize, usize) {
    let (intro, numbers) = game.trim().split_once(':').unwrap();
    assert!(intro.starts_with("Card "));
    let id = intro[5..].trim().parse::<usize>().unwrap();
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
    let score = winning_numbers_count;
    (id, score)
}

fn collect_won_cards(cards: &mut Cards) {
    let total_nr_cards = cards.len();
    let ids: Vec<usize> = cards.keys().cloned().collect();
    for id in ids {
        let score = cards.get(&id).unwrap().score;
        if score > 0 {
            let nr_won_cards = cards.get(&id).unwrap().count;
            let first_won_car_id = total_nr_cards.min(id + 1);
            let last_won_card_id = total_nr_cards.min(first_won_car_id + score - 1);
            for won_card_id in first_won_car_id..=last_won_card_id {
                cards.get_mut(&won_card_id).unwrap().count += nr_won_cards;
            }
        }
    }
}