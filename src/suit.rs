use std::fmt::{self, Display};

#[derive(PartialEq, Clone, Copy, Debug)]
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
    vec![Suit::Club, Suit::Diamond, Suit::Heart, Suit::Spade]
}

#[cfg(test)]
mod tests {
    use super::*;
    // TODO Test that Suit display implementation returns correct string for each suit.

    #[test]
    fn test_values() {
        let vals = values();
        assert_eq!(4, vals.len());
        assert_eq!(vals[0], Suit::Club);
        assert_eq!(vals[1], Suit::Diamond);
        assert_eq!(vals[2], Suit::Heart);
        assert_eq!(vals[3], Suit::Spade);
    }
}
