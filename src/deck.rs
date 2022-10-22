use rand::seq::SliceRandom;
use rand::Rng;

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
        return Deck { cards };
    }

    pub fn shuffle<R>(&mut self, rng: &mut R)
    where
        R: Rng + ?Sized,
    {
        self.cards.shuffle(rng);
    }

    pub fn draw(&mut self) -> Card {
        return self.cards.remove(0);
    }
}

#[cfg(test)]
mod tests {

    // TODO Test that constructor builds a deck with all suits and ranks.
    // TODO Test that shuffle uses shuffling function to shuffle the deck.
    // TODO Test that draw takes the topmost card from the deck.
}
