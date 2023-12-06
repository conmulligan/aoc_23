use core::RunError;

use day_6a::model::Race;

pub mod parsing;

pub fn run() -> Result<String, RunError> {
    let lines = core::parse_lines(day_6a::INPUT);
    let race = parsing::parse_race(lines)?;
    let count: u64 = calculate_win_permuations(&race);

    Ok(count.to_string())
}

fn calculate_win_permuations(race: &Race) -> u64 {
    let mut count: u64 = 0;

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
