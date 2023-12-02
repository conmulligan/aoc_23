use core::RunError;

static INPUT: &str = include_str!("../input.txt");

static MAX_RED: u8 = 12;
static MAX_GREEN: u8 = 13;
static MAX_BLUE: u8 = 14;

#[derive(Debug)]
struct GameRound {
    red_count: u8,
    green_count: u8,
    blue_count: u8,
}

#[derive(Debug)]
struct Game {
    id: u32,
    rounds: Vec<GameRound>,
}

pub fn run() -> Result<String, RunError> {
    let lines = INPUT
        .split('\n')
        .filter(|l| !l.is_empty())
        .collect::<Vec<_>>();

    let mut sum_ids: u32 = 0;

    for line in lines {
        let game = parse_game(line)?;

        if is_qualifying_game(&game.rounds) {
            sum_ids += game.id;
        }
    }

    Ok(sum_ids.to_string())
}

fn is_qualifying_game(rounds: &Vec<GameRound>) -> bool {
    for round in rounds {
        if round.red_count > MAX_RED || round.green_count > MAX_GREEN || round.blue_count > MAX_BLUE
        {
            return false;
        }
    }

    true
}

fn parse_game(string: &str) -> Result<Game, RunError> {
    let components: Vec<&str> = string.split(":").collect();
    if components.len() != 2 {
        return Err(RunError {
            message: String::from("Invalid game string format."),
        });
    }

    let id = parse_id(components.first().unwrap()).unwrap();
    let rounds = parse_rounds(components.last().unwrap())?;

    Ok(Game { id, rounds })
}

fn parse_id(string: &str) -> Option<u32> {
    let components: Vec<&str> = string.trim().split(" ").collect();
    if components.len() < 2 {
        return None;
    }

    components.last().unwrap().parse::<u32>().ok()
}

fn parse_rounds(string: &str) -> Result<Vec<GameRound>, RunError> {
    let components: Vec<&str> = string.trim().split(";").collect();

    let mut rounds: Vec<GameRound> = Vec::new();

    for component in components {
        let round = parse_round(component)?;
        rounds.push(round);
    }

    Ok(rounds)
}

fn parse_round(string: &str) -> Result<GameRound, RunError> {
    let components: Vec<&str> = string.trim().split(",").collect();

    let mut red_count: u8 = 0;
    let mut green_count: u8 = 0;
    let mut blue_count: u8 = 0;

    for component in components {
        let subcomponents: Vec<&str> = component.trim().split(" ").collect();
        let count = subcomponents.first().unwrap().parse::<u8>().unwrap();
        let color = subcomponents.last().unwrap();

        match color.as_ref() {
            "red" => red_count = count,
            "green" => green_count = count,
            "blue" => blue_count = count,
            _ => {
                return Err(RunError {
                    message: String::from("Failed to parse the game round."),
                })
            }
        }
    }

    return Ok(GameRound {
        red_count,
        green_count,
        blue_count,
    });
}
