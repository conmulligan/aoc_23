use std::ops::Range;

use crate::model::{Grid, PartNumber};

pub fn parse_grid(lines: Vec<&str>) -> Grid {
    let mut rows: Grid = Vec::new();
    lines.iter().for_each(|l| rows.push(l.chars().collect()));
    rows
}

pub fn make_part_number(
    grid: &Grid,
    row_index: usize,
    col_start_index: usize,
    col_end_index: usize,
) -> PartNumber {
    let col_range = Range {
        start: col_start_index,
        end: col_end_index,
    };
    let value = value_of_range_grid(row_index, &col_range, grid);
    PartNumber {
        row_index,
        col_range,
        value,
    }
}

pub fn parse_part_numbers(grid: &Grid) -> Vec<PartNumber> {
    let mut part_numbers: Vec<PartNumber> = Vec::new();

    for (row_index, row) in grid.iter().enumerate() {
        let mut start_idx: Option<usize> = None;
        for (col_index, char) in row.iter().enumerate() {
            if char.is_ascii_digit() && start_idx.is_none() {
                start_idx = Some(col_index);
            } else if !char.is_ascii_digit() {
                if let Some(start) = start_idx {
                    part_numbers.push(make_part_number(grid, row_index, start, col_index));
                    start_idx = None;
                }
            } else if col_index == row.len() - 1 {
                if let Some(start) = start_idx {
                    part_numbers.push(make_part_number(grid, row_index, start, col_index + 1));
                }
            }
        }
    }

    part_numbers
}

fn value_of_range_grid(row_index: usize, range: &Range<usize>, grid: &Grid) -> u32 {
    let row = &grid[row_index];
    row[range.to_owned()]
        .iter()
        .collect::<String>()
        .parse::<u32>()
        .unwrap()
}
