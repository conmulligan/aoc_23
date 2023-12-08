use core::{lcm, parse_lines, RunError};
use std::collections::HashMap;

use day_8a::{model::Node, parsing, INPUT};

pub fn run() -> Result<String, RunError> {
    let mut lines = parse_lines(INPUT);

    let directions = lines.remove(0);
    let nodes = parsing::parse_nodes(lines);

    let totals: Vec<usize> = nodes
        .iter()
        .filter(|(k, _)| k.ends_with('A'))
        .map(|(_, v)| calculate_steps(v, &directions, &nodes))
        .collect();

    Ok(lcm(totals.as_slice()).to_string())
}

fn calculate_steps(start: &Node, directions: &str, nodes: &HashMap<String, Node>) -> usize {
    let directions: Vec<char> = directions.chars().collect();
    let mut direction_index: usize = 0;

    let mut node: &Node = start;
    let mut steps: usize = 0;

    while !node.id.ends_with("Z") {
        let direction = directions[direction_index];
        let next_id = if direction == 'L' {
            &node.left_id
        } else {
            &node.right_id
        };

        node = &nodes[next_id];

        direction_index = if direction_index < directions.len() - 1 {
            direction_index + 1
        } else {
            0
        };

        steps += 1;
    }

    steps
}
