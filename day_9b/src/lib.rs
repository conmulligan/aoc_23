use core::{parse_lines, RunError};

use day_9a::{differences_for_sequence, parsing};

pub fn run() -> Result<String, RunError> {
    let lines = parse_lines(day_9a::INPUT);
    let values: Vec<_> = lines.iter().map(|v| parsing::parse_values(v)).collect();
    let total: i32 = values.iter().map(|v| enumerate_differences(&v)).sum();

    Ok(total.to_string())
}

fn enumerate_differences(sequence: &Vec<i32>) -> i32 {
    let mut current: Vec<i32> = sequence.clone();
    let mut differences: Vec<Vec<i32>> = vec![sequence.to_owned()];
    let mut last = false;

    while !last {
        current = differences_for_sequence(&current);
        differences.push(current.clone());
        last = current.iter().all(|v| v.eq(&0));
    }

    differences.reverse();

    let mut amended = differences.clone();

    for (index, _) in differences.iter().enumerate() {
        if index == differences.len() - 1 {
            break;
        }
        let mut next = differences[index + 1].clone();
        next.insert(0, next.first().unwrap() - amended[index].first().unwrap());
        amended[index + 1] = next;
    }

    *amended.last().unwrap().first().unwrap()
}
