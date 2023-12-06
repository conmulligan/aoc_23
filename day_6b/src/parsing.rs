use core::RunError;

use day_6a::model::Race;

pub fn parse_race(lines: Vec<&str>) -> Result<Race, RunError> {
    let time = parse_number(lines.first().unwrap());
    let distance = parse_number(lines.last().unwrap());

    Ok(Race { time, distance })
}

fn parse_number(line: &str) -> u64 {
    line.split(':')
        .last()
        .unwrap()
        .replace(' ', "")
        .parse::<u64>()
        .unwrap()
}
