use std::fmt;

use crate::rank;
use crate::suit;

#[derive(PartialEq, Eq)]
pub struct Card {
    pub suit: suit::Suit,
    pub rank: rank::Rank,
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.rank, self.suit)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display() {
        for suit in suit::values() {
            for rank in rank::values() {
                let val = format!("{}{}", rank, suit);
                assert_eq!(val, Card { suit, rank }.to_string());
            }
        }
    }
}
