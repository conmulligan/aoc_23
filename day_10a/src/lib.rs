use core::{parse_lines, RunError};

use model::Grid;
use parsing::{adjacent_tiles, parse_grid};

use crate::model::TileType;

pub mod model;
pub mod parsing;

pub static INPUT: &str = include_str!("../input.txt");

pub fn run() -> Result<String, RunError> {
    let lines = parse_lines(INPUT);
    let grid = parse_grid(lines);
    let count = traverse_loop(&grid);

    Ok(count.to_string())
}

fn traverse_loop(grid: &Grid) -> usize {
    let mut current = grid.start.clone();
    let mut previous = grid.start.clone();
    let mut count = 0;

    loop {
        let adjacent = adjacent_tiles(&current, grid);
        let filtered: Vec<_> = adjacent
            .into_iter()
            .filter(|t| t.tile_type != TileType::Ground)
            .filter(|t| t.location != previous.location)
            .collect();

        previous = current;
        current = filtered.first().unwrap().clone();

        count += 1;
        if filtered.iter().any(|t| t.tile_type == TileType::Start) {
            break;
        }
    }

    count / 2
}
