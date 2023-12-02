#[derive(Debug)]
pub struct GameRound {
    pub red_count: u32,
    pub green_count: u32,
    pub blue_count: u32,
}

#[derive(Debug)]
pub struct Game {
    pub id: u32,
    pub rounds: Vec<GameRound>,
}
