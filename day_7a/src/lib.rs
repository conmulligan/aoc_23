use core::RunError;
use std::collections::HashMap;

use model::Hand;

pub mod model;
mod parsing;

pub static INPUT: &str = include_str!("../input.txt");

pub fn run() -> Result<String, RunError> {
    let lines = core::parse_lines(INPUT);
    let hands = parsing::parse_hands(lines)?;

    let mut weighted_hands: Vec<_> = hands
        .iter()
        .map(|h| ((hand_weight(&h), &h.cards), h.bid_amount))
        .collect();
    weighted_hands.sort();

    let total: u32 = weighted_hands
        .iter()
        .enumerate()
        .map(|(i, (_, amount))| (i as u32 + 1) * amount)
        .sum();

    Ok(total.to_string())
}

fn hand_weight(hand: &Hand) -> Vec<u32> {
    let mut map = HashMap::new();

    for card in &hand.cards {
        *map.entry(card).or_insert(0) += 1;
    }

    let mut counts: Vec<_> = map.into_values().collect();
    counts.sort_by(|a, b| b.cmp(a));
    counts
}
