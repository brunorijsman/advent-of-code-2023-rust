use std::cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd};
use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;
use crate::card::Card;


#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub enum Rank {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct Hand {
    cards: [Card; 5],
    rank: Rank,
}

impl fmt::Display for Hand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for card in self.cards.iter() {
            write!(f, "{}", card)?;
        }
        write!(f, " {:?}", self.rank)?;
        Ok(())
    }
}

impl FromStr for Hand {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bytes = s.as_bytes();
        if bytes.len() != 5 {
            return Err(());
        }
        let cards = bytes
            .iter()
            .map(|&b| Card::from(b))
            .collect::<Vec<Card>>()
            .try_into()
            .unwrap();
        let rank = rank_of_cards(&cards);
        Ok(Hand { cards, rank })
    }
}

impl Ord for Hand {
    fn cmp(&self, _other: &Self) -> std::cmp::Ordering {
        let rank_ordering = self.rank.cmp(&_other.rank);
        if rank_ordering != Ordering::Equal {
            return rank_ordering;
        }
        for (card_self, card_other) in self.cards.iter().zip(_other.cards.iter()) {
            let card_ordering = card_self.cmp(card_other);
            if card_ordering != Ordering::Equal {
                return card_ordering;
            }
        }
        Ordering::Equal
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn rank_of_cards(cards: &[Card; 5]) -> Rank {
    // Count how many of each card we have
    let mut frequencies: HashMap<Card, u8> = HashMap::new();
    for card in cards.iter() {
        frequencies.entry(*card).and_modify(|c| *c += 1).or_insert(1);
    }
    // Sort by count descending (and discard card value)
    let mut frequencies: Vec<_> = frequencies
        .iter()
        .map(|(_, &count)| count)
        .collect();
    frequencies.sort_by(|a, b| b.cmp(a));
    // Get top 2 counts
    let top1 = frequencies[0];
    let top2 = frequencies.get(1).unwrap_or(&0);
    let rank = match (top1, top2) {
        (5, _) => Rank::FiveOfAKind,
        (4, _) => Rank::FourOfAKind,
        (3, 2) => Rank::FullHouse,
        (3, _) => Rank::ThreeOfAKind,
        (2, 2) => Rank::TwoPair,
        (2, _) => Rank::OnePair,
        (_, _) => Rank::HighCard,
    };
    rank
}