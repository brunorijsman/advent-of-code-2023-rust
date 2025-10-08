// A bit over the top with all the types, but hey, the point is to learn Rust, not to solve the problem asap

mod hand;
mod card;

use std::fs::read_to_string;
use std::str::FromStr;
use crate::hand::Hand;

fn main() {
    // Read all plays
    let mut plays: Vec<(Hand, u64)> = vec![];
    for line in read_to_string("puzzle_input").unwrap().lines() {
        let (hand, bid) = parse_line(line);
        plays.push((hand, bid));
    }
    // Sort plays by hand strength, weakest first
    plays.sort_by(|(hand_a, _), (hand_b, _)| hand_a.cmp(hand_b));
    // Compute winnings
    let mut winnings: u64 = 0;
    for (index, play) in plays.iter().enumerate() {
        let rank = index + 1;
        winnings += play.1 * rank as u64;
    }
    println!("Winnings are {winnings}");

}

fn parse_line(line: &str) -> (Hand, u64) {
    let mut iter = line.split(' ');
    let hand_str = iter.next().unwrap();
    let hand = Hand::from_str(hand_str).unwrap();
    let bid_str = iter.next().unwrap();
    let bid = bid_str.parse().unwrap();
    (hand, bid)
}

