use core::{parse_lines, RunError};
use std::collections::HashMap;

use model::Node;

pub static INPUT: &str = include_str!("../input.txt");

pub mod model;
pub mod parsing;

pub fn run() -> Result<String, RunError> {
    let mut lines = parse_lines(INPUT);

    let directions = lines.remove(0);
    let nodes = parsing::parse_nodes(lines);
    let start = &nodes["AAA"];
    let destination = &nodes["ZZZ"];

    let steps = calculate_steps(&start, &destination, &directions, &nodes);

    Ok(steps.to_string())
}

fn calculate_steps(
    start: &Node,
    destination: &Node,
    directions: &str,
    nodes: &HashMap<String, Node>,
) -> u32 {
    let directions: Vec<char> = directions.chars().collect();
    let mut direction_index: usize = 0;

    let mut node: &Node = start;
    let mut steps: u32 = 0;

    while node.id != destination.id {
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
