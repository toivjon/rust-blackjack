use std::fmt::{self, Display};

#[derive(PartialEq, Clone, Copy)]
pub enum Suit {
    Club,
    Diamond,
    Heart,
    Spade,
}

impl Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Suit::Club => write!(f, "♣"),
            Suit::Diamond => write!(f, "♦"),
            Suit::Heart => write!(f, "♥"),
            Suit::Spade => write!(f, "♠"),
        }
    }
}

pub fn values() -> Vec<Suit> {
    return vec![Suit::Club, Suit::Diamond, Suit::Heart, Suit::Spade];
}

#[cfg(test)]
mod tests {

    // TODO Test that Suit display implementation returns correct string for each suit.
    // TODO Test that values implementation returns all values.
}
