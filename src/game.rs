use std::io::stdin;

enum Decision {
    Unknown,
    Hit,
    Stand,
}

pub fn play() {
    loop {
        println!("=================");
        println!("BLACKJACK - ROUND");
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

fn wait_selection() -> Decision {
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Invalid input");
    match input.trim() {
        "1" => Decision::Hit,
        "2" => Decision::Stand,
        _ => Decision::Unknown,
    }
}
