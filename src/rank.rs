use std::fmt;

#[derive(PartialEq, Eq, Debug)]
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

impl fmt::Display for Rank {
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
            Rank::Ace => (1, 11),
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

    #[test]
    fn test_display() {
        assert_eq!("A", Rank::Ace.to_string());
        assert_eq!("2", Rank::Two.to_string());
        assert_eq!("3", Rank::Three.to_string());
        assert_eq!("4", Rank::Four.to_string());
        assert_eq!("5", Rank::Five.to_string());
        assert_eq!("6", Rank::Six.to_string());
        assert_eq!("7", Rank::Seven.to_string());
        assert_eq!("8", Rank::Eight.to_string());
        assert_eq!("9", Rank::Nine.to_string());
        assert_eq!("10", Rank::Ten.to_string());
        assert_eq!("J", Rank::Jack.to_string());
        assert_eq!("Q", Rank::Queen.to_string());
        assert_eq!("K", Rank::King.to_string());
    }

    #[test]
    fn test_points() {
        assert_eq!((1, 11), Rank::Ace.points());
        assert_eq!((2, 2), Rank::Two.points());
        assert_eq!((3, 3), Rank::Three.points());
        assert_eq!((4, 4), Rank::Four.points());
        assert_eq!((5, 5), Rank::Five.points());
        assert_eq!((6, 6), Rank::Six.points());
        assert_eq!((7, 7), Rank::Seven.points());
        assert_eq!((8, 8), Rank::Eight.points());
        assert_eq!((9, 9), Rank::Nine.points());
        assert_eq!((10, 10), Rank::Ten.points());
        assert_eq!((10, 10), Rank::Jack.points());
        assert_eq!((10, 10), Rank::Queen.points());
        assert_eq!((10, 10), Rank::King.points());
    }

    #[test]
    fn test_values() {
        let vals = values();
        assert_eq!(13, vals.len());
        assert_eq!(vals[0], Rank::Ace);
        assert_eq!(vals[1], Rank::Two);
        assert_eq!(vals[2], Rank::Three);
        assert_eq!(vals[3], Rank::Four);
        assert_eq!(vals[4], Rank::Five);
        assert_eq!(vals[5], Rank::Six);
        assert_eq!(vals[6], Rank::Seven);
        assert_eq!(vals[7], Rank::Eight);
        assert_eq!(vals[8], Rank::Nine);
        assert_eq!(vals[9], Rank::Ten);
        assert_eq!(vals[10], Rank::Jack);
        assert_eq!(vals[11], Rank::Queen);
        assert_eq!(vals[12], Rank::King);
    }
}
