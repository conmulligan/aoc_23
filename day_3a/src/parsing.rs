use crate::model::{Grid, PartNumber};

pub fn parse_grid(lines: Vec<&str>) -> Grid {
    let mut rows: Grid = Vec::new();
    lines.iter().for_each(|l| rows.push(l.chars().collect()));
    rows
}

pub fn parse_part_numbers(grid: &Grid) -> Vec<PartNumber> {
    let mut part_numbers: Vec<PartNumber> = Vec::new();

    for (row_index, row) in grid.iter().enumerate() {
        let mut start_index: Option<usize> = None;
        let mut current_digit: Option<u32> = None;

        for (col_index, char) in row.iter().enumerate() {
            if char.is_ascii_digit() {
                let digit = char.to_digit(10).unwrap();

                if let Some(current) = current_digit {
                    current_digit = Some((current * 10) + digit);
                } else {
                    start_index = Some(col_index);
                    current_digit = Some(digit);
                }
            }

            if !char.is_ascii_digit() || col_index == row.len() - 1 {
                if start_index.is_some() && current_digit.is_some() {
                    part_numbers.push(PartNumber {
                        row_index,
                        col_range: start_index.unwrap()..col_index,
                        value: current_digit.unwrap(),
                    });
                    start_index = None;
                    current_digit = None;
                }
            }
        }
    }

    part_numbers
}
