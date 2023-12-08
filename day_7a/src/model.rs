pub type Card = u32;

#[derive(Debug)]
pub struct Hand {
    pub cards: Vec<Card>,
    pub bid_amount: u32,
}
