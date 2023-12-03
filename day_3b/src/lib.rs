use core::RunError;

use day_3a::model::{Grid, PartNumber};

static GEAR_CHAR: &'static char = &'*';

pub fn run() -> Result<String, RunError> {
    let lines = core::parse_lines(day_3a::INPUT);
    let grid = day_3a::parsing::parse_grid(lines);
    let part_numbers = day_3a::parsing::parse_part_numbers(&grid);
    let gear_ratio = enumerate_gears(&part_numbers, &grid);

    Ok(gear_ratio.to_string())
}

fn enumerate_gears(part_numbers: &Vec<PartNumber>, grid: &Grid) -> u32 {
    let mut sum_gear_ratio: u32 = 0;

    for (row_index, row) in grid.iter().enumerate() {
        for (col_index, char) in row.iter().enumerate() {
            if char.eq(GEAR_CHAR) {
                let matches = adjacent_part_numbers(row_index, col_index, &part_numbers);
                if matches.len() == 2 {
                    let ratio: u32 = matches.iter().product();
                    sum_gear_ratio += ratio;
                }
            }
        }
    }

    sum_gear_ratio
}

fn adjacent_part_numbers(row: usize, col: usize, part_numbers: &Vec<PartNumber>) -> Vec<u32> {
    let mut matches: Vec<u32> = Vec::new();

    for part_number in part_numbers {
        let row_start = row.checked_sub(1).unwrap_or(0);
        let col_start = part_number.col_range.start;
        let col_start = col_start.checked_sub(1).unwrap_or(col_start);
        let range = col_start..part_number.col_range.end + 1;

        if (row_start..=row + 1).contains(&part_number.row_index) && range.contains(&col) {
            matches.push(part_number.value.to_owned());
        }
    }

    matches
}
