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
    scratchcards
        .iter()
        .filter(|c| c.matches > 0)
        .map(|c| 2_u32.pow(c.matches as u32 - 1))
        .sum()
}
