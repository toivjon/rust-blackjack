use std::{
    fmt::{self, Display},
    io::stdin,
};

// The amount of suits in a full card deck.
const SUIT_COUNT: usize = 4;

/// The amount of ranks per one card suit.
const RANKS_PER_SUIT: usize = 13;

/// The amount of cards in a full card deck.
const CARD_COUNT: usize = SUIT_COUNT * RANKS_PER_SUIT;

enum Decision {
    Unknown,
    Hit,
    Stand,
}

#[derive(Copy, Clone, PartialEq)]
enum Suit {
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

struct Card {
    suit: Suit,
    rank: usize,
}

impl Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.rank, self.suit)
    }
}

pub fn play() {
    let mut deck = build_deck();

    let mut dealers_hand: Vec<Card> = Vec::new();
    dealers_hand.push(deck.remove(1)); // TODO select with rng
    dealers_hand.push(deck.remove(1)); // TODO select with rng

    let mut players_hand: Vec<Card> = Vec::new();
    players_hand.push(deck.remove(1)); // TODO select with rng
    players_hand.push(deck.remove(1)); // TODO select with rng

    loop {
        println!("=================");
        println!("BLACKJACK - ROUND");
        println!("Dealers hand:");
        println!("{}", dealers_hand[0]);
        println!("{}", dealers_hand[1]); // TODO temporarily show this during dev.
        println!("Players hand:");
        println!("{}", players_hand[0]);
        println!("{}", players_hand[1]);
        println!("1 : Hit");
        println!("2 : Stand");
        println!("Enter selection:");

        match wait_selection() {
            Decision::Unknown => continue,
            Decision::Hit => {
                println!("Hit!");
                break;
            }
            Decision::Stand => {
                println!("Stand!");
                break;
            }
        }
    }
}

fn build_deck() -> Vec<Card> {
    let mut deck = Vec::with_capacity(CARD_COUNT);
    for suit in [Suit::Club, Suit::Diamond, Suit::Heart, Suit::Spade] {
        for rank in 1..=RANKS_PER_SUIT {
            deck.push(Card { suit, rank })
        }
    }
    deck
}

fn wait_selection() -> Decision {
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Invalid input");
    match input.trim() {
        "1" => Decision::Hit,
        "2" => Decision::Stand,
        _ => Decision::Unknown,
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn build_deck_contains_all_suits_and_ranks() {
        let deck = build_deck();

        assert_eq!(CARD_COUNT, deck.len());
        assert_eq!(CARD_COUNT, deck.capacity());

        let mut suits = deck.chunks(RANKS_PER_SUIT);
        for suit in [Suit::Club, Suit::Diamond, Suit::Heart, Suit::Spade] {
            let ranks = suits.next().unwrap();
            assert_eq!(RANKS_PER_SUIT, ranks.len());
            for i in 0..RANKS_PER_SUIT {
                assert!(ranks[i].suit == suit);
                assert!(ranks[i].rank == i + 1);
            }
        }
    }
}