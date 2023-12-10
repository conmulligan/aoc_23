use crate::model::{self, Direction, Grid, Tile, TileType};

pub fn parse_grid(lines: Vec<&str>) -> Grid {
    let mut start_tile: Option<Tile> = None;
    let mut tiles: Vec<Vec<Tile>> = Vec::new();

    lines.iter().enumerate().for_each(|(index, line)| {
        if let Some(col_index) = line.find(model::START) {
            start_tile = Some(Tile {
                tile_type: TileType::Start,
                location: (index, col_index),
            });
        }
        tiles.push(
            line.chars()
                .enumerate()
                .map(|(j, c)| Tile {
                    tile_type: c.into(),
                    location: (index, j),
                })
                .collect(),
        );
    });

    Grid {
        start: start_tile.unwrap(),
        tiles,
    }
}

pub fn adjacent_tiles(tile: &Tile, grid: &Grid) -> Vec<Tile> {
    let directions = directions_for_tile(tile);
    let mut tiles: Vec<Option<Tile>> = Vec::new();

    if directions.contains(&Direction::Up) {
        if let Some(row) = tile.location.0.checked_sub(1) {
            tiles.push(tile_at(row, tile.location.1, grid));
        }
    }

    if directions.contains(&Direction::Left) {
        if let Some(col) = tile.location.1.checked_sub(1) {
            tiles.push(tile_at(tile.location.0, col, grid));
        }
    }

    if directions.contains(&Direction::Right) {
        tiles.push(tile_at(tile.location.0, tile.location.1 + 1, grid));
    }

    if directions.contains(&Direction::Down) {
        tiles.push(tile_at(tile.location.0 + 1, tile.location.1, grid));
    }

    tiles.into_iter().flatten().collect()
}

pub fn tile_at(row_index: usize, col_index: usize, grid: &Grid) -> Option<Tile> {
    if row_index > grid.tiles.len() - 1 {
        return None;
    }
    let row = &grid.tiles[row_index];
    if col_index > row.len() - 1 {
        return None;
    }
    Some(row[col_index].clone())
}

pub fn directions_for_tile(tile: &Tile) -> Vec<Direction> {
    match &tile.tile_type {
        TileType::Vertical => vec![Direction::Up, Direction::Down],
        TileType::Horizontal => vec![Direction::Left, Direction::Right],
        TileType::BendNE => vec![Direction::Up, Direction::Right],
        TileType::BendNW => vec![Direction::Up, Direction::Left],
        TileType::BendSW => vec![Direction::Down, Direction::Left],
        TileType::BendSE => vec![Direction::Down, Direction::Right],
        TileType::Ground => vec![],
        TileType::Start => vec![
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ],
    }
}
