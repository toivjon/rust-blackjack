use rand::seq::SliceRandom;
use rand::thread_rng;

use crate::card::Card;
use crate::rank;
use crate::suit;

pub struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    pub fn new() -> Deck {
        let mut cards = Vec::with_capacity(suit::values().len() * rank::values().len());
        for suit in suit::values() {
            for rank in rank::values() {
                cards.push(Card { rank, suit });
            }
        }
        cards.shuffle(&mut thread_rng());
        Deck { cards }
    }

    pub fn draw(&mut self) -> Card {
        self.cards.remove(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let deck = Deck::new();
        assert_eq!(
            deck.cards.len(),
            suit::values().len() * rank::values().len()
        );
        for suit in suit::values() {
            for rank in rank::values() {
                assert!(deck.cards.contains(&Card { suit, rank }));
            }
        }
    }

    #[test]
    fn test_draw() {
        let mut deck = Deck::new();
        let top_card = Card {
            suit: deck.cards[0].suit,
            rank: deck.cards[0].rank,
        };
        let card = deck.draw();
        assert_eq!(card.suit, top_card.suit);
        assert_eq!(card.rank, top_card.rank);
    }
}
