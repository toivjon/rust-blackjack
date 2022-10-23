use rand::thread_rng;
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
    deck.shuffle(&mut thread_rng());

    let mut dealers_hand = Hand::new("Dealer");
    dealers_hand.add(deck.draw());
    dealers_hand.add(deck.draw());

    let mut players_hand = Hand::new("Player");
    players_hand.add(deck.draw());
    players_hand.add(deck.draw());

    loop {
        // let (dealer_points, dealer_alt_points) = dealers_hand.points();
        // let (player_points, player_alt_points) = players_hand.points();

        println!("=================");
        println!("BLACKJACK - ROUND");
        dealers_hand.print_partial();
        players_hand.print_reveal();
        println!("1 : Hit");
        println!("2 : Stand");
        println!("Enter selection:");

        match wait_selection() {
            Decision::Unknown => continue,
            Decision::Hit => players_hand.add(deck.draw()),
            Decision::Stand => {
                println!("Stand!");
                break;
            }
        }
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
