use core::RunError;

use crate::model::ScratchCard;

pub fn parse_scratchcards(lines: Vec<&str>) -> Result<Vec<ScratchCard>, RunError> {
    let mut scratchcards: Vec<ScratchCard> = Vec::new();

    for line in lines {
        let scratchcard = parse_scratchcard(line)?;
        scratchcards.push(scratchcard);
    }

    Ok(scratchcards)
}

pub fn parse_scratchcard(string: &str) -> Result<ScratchCard, RunError> {
    let components: Vec<&str> = string.split(':').collect();
    if components.len() != 2 {
        return Err(RunError {
            message: "Invalid card string.".to_string(),
        });
    }

    let card_number = components.first().unwrap().trim().to_string();

    let num_components: Vec<&str> = components.last().unwrap().split('|').collect();
    if num_components.len() != 2 {
        return Err(RunError {
            message: "Invalid number strings.".to_string(),
        });
    }

    let winning_numbers = parse_numbers(num_components.first().unwrap().trim());
    let numbers = parse_numbers(num_components.last().unwrap().trim());

    Ok(ScratchCard {
        card_number,
        winning_numbers,
        numbers,
    })
}

pub fn parse_numbers(string: &str) -> Vec<u8> {
    let (numbers, _): (Vec<_>, Vec<_>) = string
        .split(' ')
        .map(|s| s.trim().parse::<u8>())
        .partition(Result::is_ok);

    numbers.into_iter().map(Result::unwrap).collect()
}
