#[derive(Debug)]
pub struct ScratchCard {
    pub winning_numbers: Vec<u8>,
    pub numbers: Vec<u8>,
    pub matches: usize,
}
