use std::cmp::{Eq, PartialEq, Ord, PartialOrd};
use std::fmt;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Card {
    label: char,
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.label)?;
        Ok(())
    }
}

impl From<u8> for Card {
    fn from(value: u8) -> Self {
        Card { label: value as char }
    }
}

fn card_value(card: &Card) -> u8 {
    match card.label {
        'J' => 1,
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        'T' => 10,
        'Q' => 11,
        'K' => 12,
        'A' => 13,
        _ => panic!("Invalid card: {}", card.label),
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        card_value(self).cmp(&card_value(other))
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Card {

    pub const JOKER: Card = Card { label: 'J' };

    pub fn all_non_joker_cards() -> Vec<Card> {
        let labels = [
            '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A',
        ];
        labels.iter().map(|&label| Card { label }).collect()
    }

}