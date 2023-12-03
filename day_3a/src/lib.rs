use core::RunError;

use crate::model::{Grid, PartNumber};

pub mod model;
pub mod parsing;

pub static INPUT: &str = include_str!("../input.txt");

pub fn run() -> Result<String, RunError> {
    let lines = core::parse_lines(INPUT);
    let grid = parsing::parse_grid(lines);

    let sum: u32 = parsing::parse_part_numbers(&grid)
        .iter()
        .filter(|p| symbol_is_adjacent(&p, &grid))
        .map(|p| p.value)
        .sum();

    Ok(sum.to_string())
}

pub fn symbol_is_adjacent(part_number: &PartNumber, grid: &Grid) -> bool {
    let row_start = part_number.row_index.checked_sub(1).unwrap_or(0);
    let row_end = part_number.row_index + 1;

    for row in row_start..=row_end {
        let col_start = part_number.col_range.start.checked_sub(1).unwrap_or(0);
        let col_end = part_number.col_range.end + 1;
        for col in col_start..col_end {
            if let Some(char) = character_at(row, col, grid) {
                if is_symbol(char) {
                    return true;
                }
            }
        }
    }
    false
}

pub fn character_at(row_index: usize, col_index: usize, grid: &Grid) -> Option<&char> {
    if row_index > grid.len() - 1 {
        return None;
    }
    let row = &grid[row_index];
    if col_index > row.len() - 1 {
        return None;
    }
    Some(&row[col_index])
}

pub fn is_symbol(char: &char) -> bool {
    !char.is_ascii_digit() && !char.eq(&'.')
}
