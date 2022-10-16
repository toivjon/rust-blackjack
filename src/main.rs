use std::io::stdin;

fn main() {
    loop {
        println!("================");
        println!("BLACKJACK - MENU");
        println!("1 : play game");
        println!("2 : quit game");
        println!("Enter selection:");

        let mut input = String::new();
        stdin().read_line(&mut input).expect("Invalid input");

        match input.trim() {
            "1" => println!("TODO Play Game!"),
            "2" => break,
            _ => continue,
        }
    }
}
