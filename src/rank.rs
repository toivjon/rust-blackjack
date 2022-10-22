use std::fmt::{self, Display};

#[derive(PartialEq)]
pub enum Rank {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
}

impl Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Rank::Ace => write!(f, "A"),
            Rank::Two => write!(f, "2"),
            Rank::Three => write!(f, "3"),
            Rank::Four => write!(f, "4"),
            Rank::Five => write!(f, "5"),
            Rank::Six => write!(f, "6"),
            Rank::Seven => write!(f, "7"),
            Rank::Eight => write!(f, "8"),
            Rank::Nine => write!(f, "9"),
            Rank::Ten => write!(f, "10"),
            Rank::Jack => write!(f, "J"),
            Rank::Queen => write!(f, "Q"),
            Rank::King => write!(f, "K"),
        }
    }
}

impl Rank {
    pub fn points(&self) -> (usize, usize) {
        match self {
            Rank::Ace => (1, 14),
            Rank::Two => (2, 2),
            Rank::Three => (3, 3),
            Rank::Four => (4, 4),
            Rank::Five => (5, 5),
            Rank::Six => (6, 6),
            Rank::Seven => (7, 7),
            Rank::Eight => (8, 8),
            Rank::Nine => (9, 9),
            Rank::Ten => (10, 10),
            Rank::Jack => (10, 10),
            Rank::Queen => (10, 10),
            Rank::King => (10, 10),
        }
    }
}

pub fn values() -> Vec<Rank> {
    vec![
        Rank::Ace,
        Rank::Two,
        Rank::Three,
        Rank::Four,
        Rank::Five,
        Rank::Six,
        Rank::Seven,
        Rank::Eight,
        Rank::Nine,
        Rank::Ten,
        Rank::Jack,
        Rank::Queen,
        Rank::King,
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    // TODO Test that Rank display implementation returns correct strings for each rank.
    // TODO Test that Rank points implementation returns correct points for each rank.

    #[test]
    fn test_values() {
        let vals = values();
        assert_eq!(13, vals.len());
        assert!(vals[0] == Rank::Ace);
        assert!(vals[1] == Rank::Two);
        assert!(vals[2] == Rank::Three);
        assert!(vals[3] == Rank::Four);
        assert!(vals[4] == Rank::Five);
        assert!(vals[5] == Rank::Six);
        assert!(vals[6] == Rank::Seven);
        assert!(vals[7] == Rank::Eight);
        assert!(vals[8] == Rank::Nine);
        assert!(vals[9] == Rank::Ten);
        assert!(vals[10] == Rank::Jack);
        assert!(vals[11] == Rank::Queen);
        assert!(vals[12] == Rank::King);
    }
}
