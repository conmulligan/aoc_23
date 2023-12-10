const VERTICAL: char = '|';
const HORIZONTAL: char = '-';
const BEND_NE: char = 'L';
const BEND_NW: char = 'J';
const BEND_SW: char = '7';
const BEND_SE: char = 'F';
const GROUND: char = '.';
pub const START: char = 'S';

#[derive(Clone, Debug, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Debug, PartialEq)]
pub enum TileType {
    Vertical,
    Horizontal,
    BendNE,
    BendNW,
    BendSW,
    BendSE,
    Ground,
    Start,
}

impl From<char> for TileType {
    fn from(value: char) -> Self {
        match value {
            VERTICAL => Self::Vertical,
            HORIZONTAL => Self::Horizontal,
            BEND_NE => Self::BendNE,
            BEND_NW => Self::BendNW,
            BEND_SW => Self::BendSW,
            BEND_SE => Self::BendSE,
            GROUND => Self::Ground,
            START => Self::Start,
            _ => panic!("Invalid character!"),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Tile {
    pub tile_type: TileType,
    pub location: (usize, usize),
}

#[derive(Debug)]
pub struct Grid {
    pub start: Tile,
    pub tiles: Vec<Vec<Tile>>,
}
