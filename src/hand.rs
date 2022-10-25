use crate::card::Card;

pub struct Hand {
    pub name: String,
    pub cards: Vec<Card>,
}

impl Hand {
    pub fn new(name: &str) -> Hand {
        Hand {
            name: name.to_string(),
            cards: vec![],
        }
    }

    pub fn add(&mut self, card: Card) {
        self.cards.push(card);
    }

    pub fn points(&self) -> (usize, usize) {
        let mut points = (0, 0);
        for card in self.cards.iter() {
            let card_points = card.rank.points();
            points.0 += card_points.0;
            points.1 += card_points.1;
        }
        points
    }

    pub fn print_partial(&self) {
        println!("{}'s Hand:", self.name);
        println!("  {}", self.cards[0]);
        println!("  ?");
        println!("  total ?");
    }

    pub fn print_reveal(&self) {
        println!("{}'s Hand:", self.name);
        for card in self.cards.iter() {
            println!("  {}", card);
        }
        let (points, alt_points) = self.points();
        if points != alt_points {
            println!("  total {}/{}", points, alt_points);
        } else {
            println!("  total {}", points);
        }
    }
}
