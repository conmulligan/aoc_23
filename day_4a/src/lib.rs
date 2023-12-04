use core::RunError;

use model::ScratchCard;

pub mod model;
pub mod parsing;

pub static INPUT: &str = include_str!("../input.txt");

pub fn run() -> Result<String, RunError> {
    let lines = core::parse_lines(INPUT);
    let scratchcards = parsing::parse_scratchcards(lines)?;
    let score = calculate_score(&scratchcards);

    Ok(score.to_string())
}

fn calculate_score(scratchcards: &Vec<ScratchCard>) -> u32 {
    let mut total_score: u32 = 0;

    for scratchcard in scratchcards {
        let mut multiplier = 0;

        for number in &scratchcard.numbers {
            if scratchcard.winning_numbers.contains(number) {
                if multiplier == 0 {
                    multiplier = 1;
                } else {
                    multiplier = multiplier * 2;
                }
            }
        }

        total_score += multiplier;
    }

    total_score
}
