use core::RunError;

use crate::model::Hand;

pub fn parse_hands(lines: Vec<&str>) -> Result<Vec<Hand>, RunError> {
    let mut hands: Vec<Hand> = Vec::new();

    for line in lines {
        let (cards, bid_amount) = line.split_once(' ').unwrap();
        hands.push(Hand {
            cards: cards.chars().map(|c| card_value(c)).collect(),
            bid_amount: bid_amount.parse::<u32>().unwrap(),
        })
    }

    Ok(hands)
}

pub fn card_value(char: char) -> u32 {
    match char {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 11,
        'T' => 10,
        n => n.to_digit(10).unwrap_or(0),
    }
}
