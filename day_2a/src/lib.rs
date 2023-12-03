use core::RunError;

pub mod model;
pub mod parsing;

pub static INPUT: &str = include_str!("../input.txt");

static MAX_RED: u32 = 12;
static MAX_GREEN: u32 = 13;
static MAX_BLUE: u32 = 14;

pub fn run() -> Result<String, RunError> {
    let lines = core::parse_lines(INPUT);

    let mut sum_ids: u32 = 0;

    for line in lines {
        let game = parsing::parse_game(line)?;

        if is_qualifying_game(&game.rounds) {
            sum_ids += game.id;
        }
    }

    Ok(sum_ids.to_string())
}

fn is_qualifying_game(rounds: &Vec<model::GameRound>) -> bool {
    for round in rounds {
        if round.red_count > MAX_RED || round.green_count > MAX_GREEN || round.blue_count > MAX_BLUE
        {
            return false;
        }
    }

    true
}
