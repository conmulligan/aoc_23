use core::RunError;
use day_2a::model::GameRound;
use day_2a::parsing::parse_game;

pub fn run() -> Result<String, RunError> {
    let lines = core::parse_lines(day_2a::INPUT);

    let mut sum_power: u32 = 0;

    for line in lines {
        let game = parse_game(line)?;
        sum_power += calculate_power(&game.rounds);
    }

    Ok(sum_power.to_string())
}

fn calculate_power(rounds: &Vec<GameRound>) -> u32 {
    let mut max_red: u32 = 0;
    let mut max_green: u32 = 0;
    let mut max_blue: u32 = 0;

    for round in rounds {
        if round.red_count > max_red {
            max_red = u32::from(round.red_count);
        }
        if round.green_count > max_green {
            max_green = u32::from(round.green_count);
        }
        if round.blue_count > max_blue {
            max_blue = u32::from(round.blue_count);
        }
    }

    max_red * max_green * max_blue
}
