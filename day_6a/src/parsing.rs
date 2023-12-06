use core::RunError;

use crate::model::Race;

pub fn parse_races(lines: Vec<&str>) -> Result<Vec<Race>, RunError> {
    let mut races: Vec<Race> = Vec::new();

    let times = parse_numbers(lines.first().unwrap());
    let distances = parse_numbers(lines.last().unwrap());

    for (i, time) in times.iter().enumerate() {
        races.push(Race {
            time: *time,
            distance: distances[i],
        });
    }

    Ok(races)
}

fn parse_numbers(line: &str) -> Vec<u32> {
    let (numbers, _): (Vec<_>, Vec<_>) = line
        .split(':')
        .last()
        .unwrap()
        .trim()
        .split(' ')
        .map(|s| s.trim().parse::<u32>())
        .partition(Result::is_ok);

    numbers.iter().map(|n| n.clone().unwrap()).collect()
}
