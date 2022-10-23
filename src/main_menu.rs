use crate::game;
use std::io::stdin;

enum MenuSelection {
    Unknown,
    PlayGame,
    QuitGame,
}

pub fn run() {
    loop {
        println!("================");
        println!("BLACKJACK - MENU");
        println!("1 : play game");
        println!("2 : quit game");
        println!("Enter selection:");

        match wait_selection() {
            MenuSelection::Unknown => continue,
            MenuSelection::PlayGame => game::play(),
            MenuSelection::QuitGame => break,
        }
    }
}

fn wait_selection() -> MenuSelection {
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Invalid input");
    match input.trim() {
        "1" => MenuSelection::PlayGame,
        "2" => MenuSelection::QuitGame,
        _ => MenuSelection::Unknown,
    }
}
