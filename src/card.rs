use std::fmt;

use crate::{rank::Rank, suit::Suit};

#[derive(PartialEq, Eq)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.rank, self.suit)
    }
}
