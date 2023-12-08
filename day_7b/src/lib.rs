use core::RunError;
use std::collections::HashMap;

mod parsing;

use day_7a::model::Hand;

pub fn run() -> Result<String, RunError> {
    let lines = core::parse_lines(day_7a::INPUT);
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
    let mut joker_count: u32 = 0;

    for card in &hand.cards {
        if card != &1 {
            *map.entry(card).or_insert(0) += 1;
        } else {
            joker_count += 1;
        }
    }

    let mut counts: Vec<_> = map.into_values().collect();
    counts.sort_by(|a, b| b.cmp(a));

    match counts.get_mut(0) {
        Some(head) => *head += joker_count,
        None => counts.push(joker_count),
    }

    counts
}
