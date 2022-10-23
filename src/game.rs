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

    let (player_points, player_alt_points) = players_hand.points();
    let (dealer_points, dealer_alt_points) = dealers_hand.points();

    let player_hand_is_natural = player_points == 21 || player_alt_points == 21;
    let dealer_hand_is_natural = dealer_points == 21 || dealer_alt_points == 21;

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
                Decision::Stand => {
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
                        } else if dealer_points > points && dealer_points > alt_points {
                            println!("Dealer has higher points. You LOSE!");
                            println!("Game over. Better luck next time!");
                            break;
                        } else if dealer_alt_points < 21
                            && dealer_alt_points > points
                            && dealer_alt_points > alt_points
                        {
                            println!("Dealer has higher points. You LOSE!");
                            println!("Game over. Better luck next time!");
                            break;
                        }
                        dealers_hand.add(deck.draw());
                    }
                    break;
                }
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
