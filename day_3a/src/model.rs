use std::ops::Range;

pub type Grid = Vec<Vec<char>>;

#[derive(Debug)]
pub struct PartNumber {
    pub row_index: usize,
    pub col_range: Range<usize>,
    pub value: u32,
}
