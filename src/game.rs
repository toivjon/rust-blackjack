use rand::seq::SliceRandom;
use rand::thread_rng;
use rand::Rng;
use std::io::stdin;

use crate::card;
use crate::card::Card;
use crate::rank;
use crate::suit;

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

struct Deck {
    cards: Vec<card::Card>,
}

impl Deck {
    fn new() -> Deck {
        let mut cards = Vec::with_capacity(suit::values().len() * rank::values().len());
        for suit in suit::values() {
            for rank in rank::values() {
                cards.push(Card { rank, suit });
            }
        }
        return Deck { cards };
    }

    fn shuffle<R>(&mut self, rng: &mut R)
    where
        R: Rng + ?Sized,
    {
        self.cards.shuffle(rng);
    }

    fn draw(&mut self) -> card::Card {
        return self.cards.remove(0);
    }
}

pub fn play() {
    let mut deck = Deck::new();
    deck.shuffle(&mut thread_rng());

    let mut dealers_hand: Vec<card::Card> = Vec::new();
    dealers_hand.push(deck.draw());
    dealers_hand.push(deck.draw());

    let mut players_hand: Vec<card::Card> = Vec::new();
    players_hand.push(deck.draw());
    players_hand.push(deck.draw());

    loop {
        let (dealer_points, dealer_alt_points) = get_points_for_cards(&dealers_hand);
        let (player_points, player_alt_points) = get_points_for_cards(&players_hand);

        println!("=================");
        println!("BLACKJACK - ROUND");
        println!("Dealers hand:");
        println!("  {}", dealers_hand[0]);
        println!("  {}", dealers_hand[1]); // TODO temporarily show this during dev.
        if dealer_points != dealer_alt_points {
            println!("  total {}/{}", dealer_points, dealer_alt_points)
        } else {
            println!("  total {}", dealer_points)
        }
        println!("Players hand:");
        for card in players_hand.iter() {
            println!("  {}", card);
        }
        if player_points != player_alt_points {
            println!("  total {}/{}", player_points, player_alt_points)
        } else {
            println!("  total {}", player_points)
        }
        println!("1 : Hit");
        println!("2 : Stand");
        println!("Enter selection:");

        match wait_selection() {
            Decision::Unknown => continue,
            Decision::Hit => players_hand.push(deck.draw()),
            Decision::Stand => {
                println!("Stand!");
                break;
            }
        }
    }
}

fn get_points_for_cards(cards: &Vec<card::Card>) -> (usize, usize) {
    let mut points = (0, 0);
    for card in cards {
        let card_points = card.rank.points();
        points.0 += card_points.0;
        points.1 += card_points.1;
    }
    points
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
        let deck = Deck::new();

        assert_eq!(CARD_COUNT, deck.cards.len());
        assert_eq!(CARD_COUNT, deck.cards.capacity());

        let mut suits = deck.cards.chunks(RANKS_PER_SUIT);
        for suit in suit::values() {
            let ranks = suits.next().unwrap();
            assert_eq!(RANKS_PER_SUIT, ranks.len());
            for i in 0..RANKS_PER_SUIT {
                assert!(ranks[i].suit == suit);
                assert!(ranks[i].rank == rank::values()[i]);
            }
        }
    }

    #[test]
    fn get_points_for_cards_with_empty_slice() {
        let deck: Vec<card::Card> = vec![];
        assert_eq!((0, 0), get_points_for_cards(&deck));
    }
}
