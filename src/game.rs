use std::io::stdin;

use crate::deck::Deck;
use crate::hand::Hand;

enum Decision {
    Unknown,
    Hit,
    Stand,
}

pub fn play() {
    let mut deck = Deck::new();

    let mut players_hand = prepare_hand("Player", &mut deck);
    let mut dealers_hand = prepare_hand("Dealer", &mut deck);

    let player_hand_is_natural = is_natural(&players_hand);
    let dealer_hand_is_natural = is_natural(&dealers_hand);

    if player_hand_is_natural || dealer_hand_is_natural {
        println!("=================");
        println!("BLACKJACK - ROUND");
        dealers_hand.print_reveal();
        players_hand.print_reveal();
        if player_hand_is_natural && dealer_hand_is_natural {
            println!("You and dealer both have BLACKJACKS! It's a DRAW!");
            println!("Game over. Better luck next time!");
        } else if player_hand_is_natural {
            println!("You have BLACKJACK! You WIN!");
            println!("Game over. Congratulations!");
        } else if dealer_hand_is_natural {
            println!("Dealer has BLACKJACK! You LOSE!");
            println!("Game over. Better luck next time!");
        }
    } else {
        loop {
            println!("=================");
            println!("BLACKJACK - ROUND");
            dealers_hand.print_partial();
            players_hand.print_reveal();
            let (points, alt_points) = players_hand.points();
            if points > 21 && alt_points > 21 {
                println!("Card total exceeds 21. You LOSE!");
                println!("Game over. Better luck next time!");
                break;
            }
            println!("1 : Hit");
            println!("2 : Stand");
            println!("Enter selection:");

            match wait_selection() {
                Decision::Unknown => continue,
                Decision::Hit => players_hand.add(deck.draw()),
                Decision::Stand => break,
            }
        }
    }
    play_dealers_turn(&players_hand, &mut dealers_hand, &mut deck)
}

fn play_dealers_turn(players_hand: &Hand, dealers_hand: &mut Hand, deck: &mut Deck) {
    let (player_points, player_alt_points) = players_hand.points();
    loop {
        println!("=================");
        println!("BLACKJACK - ROUND");
        dealers_hand.print_reveal();
        players_hand.print_reveal();
        let (dealer_points, dealer_alt_points) = dealers_hand.points();
        if dealer_points > 21 && dealer_alt_points > 21 {
            println!("Dealer card exceeds 21. You WIN!");
            println!("Game over. Congratulations!");
            break;
        } else if dealer_points > player_points && dealer_points > player_alt_points
            || (dealer_alt_points < 21
                && dealer_alt_points > player_points
                && dealer_alt_points > player_alt_points)
        {
            println!("Dealer has higher points. You LOSE!");
            println!("Game over. Better luck next time!");
            break;
        }
        dealers_hand.add(deck.draw());
    }
}

fn prepare_hand(name: &str, deck: &mut Deck) -> Hand {
    let mut hand = Hand::new(name);
    hand.add(deck.draw());
    hand.add(deck.draw());
    hand
}

/// Check whether the provided hand is natural (aka. Blackjack hand).
///
/// Natural hand is a hand that can be acquired only during the first round of
/// the game. This kind of hand contains two cards which together provide 21
/// points. This is possible only if the hand contains an Ace (11 points) along
/// with a card that provides ten points e.g. ten or any of the court cards.
fn is_natural(hand: &Hand) -> bool {
    if hand.cards.len() != 2 {
        false
    } else {
        let (points, alt_points) = hand.points();
        points == 21 || alt_points == 21
    }
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
    use crate::{card::Card, rank::Rank, suit::Suit};

    use super::*;

    #[test]
    fn test_prepare_hand() {
        let mut deck = Deck::new();
        let hand = prepare_hand("TEST", &mut deck);
        assert_eq!(hand.name, "TEST");
        assert_eq!(hand.cards.len(), 2);
    }

    #[test]
    fn test_is_natural() {
        let mut hand = Hand::new("TEST");
        hand.add(Card {
            suit: Suit::Spade,
            rank: Rank::Ace,
        });
        hand.add(Card {
            suit: Suit::Spade,
            rank: Rank::Ten,
        });
        assert!(is_natural(&hand));
    }
}
