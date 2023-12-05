use core::RunError;
use day_4a::model::ScratchCard;
use day_4a::parsing;

pub fn run() -> Result<String, RunError> {
    let lines = core::parse_lines(day_4a::INPUT);
    let scratchcards = parsing::parse_scratchcards(lines)?;
    let count = count_scratchcards(&scratchcards);

    Ok(count.to_string())
}

fn count_scratchcards(scratchcards: &Vec<ScratchCard>) -> usize {
    let mut copies = vec![1; scratchcards.len()];

    for (i, card) in scratchcards.iter().enumerate() {
        for j in 1..=card.matches {
            if i + j >= copies.len() {
                break;
            }
            copies[i + j] += copies[i];
        }
    }

    copies.iter().sum()
}
