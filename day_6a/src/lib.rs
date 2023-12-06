use core::RunError;

use model::Race;

pub mod model;
pub mod parsing;

pub static INPUT: &str = include_str!("../input.txt");

pub fn run() -> Result<String, RunError> {
    let lines = core::parse_lines(INPUT);

    let count: u32 = parsing::parse_races(lines)?
        .iter()
        .map(|r| calculate_win_permuations(r))
        .product();

    Ok(count.to_string())
}

fn calculate_win_permuations(race: &Race) -> u32 {
    let mut count: u32 = 0;

    for i in 0..race.distance {
        if race.time < i {
            break;
        }
        let distance = i * (race.time - i);

        if distance > race.distance {
            count += 1;
        }
    }

    count
}
