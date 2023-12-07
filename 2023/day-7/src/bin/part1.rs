use std::fs;

#[derive(Debug)]
struct Hand {
    cards: Vec<i32>,
    bid: i32,
}

impl Hand {
    fn new(cards: Vec<i32>, bid: i32) -> Self {
        Hand { cards, bid }
    }

    fn from_string(hand_str: &str) -> Self {
        let str_vec = hand_str.split(" ").collect::<Vec<&str>>();
        let cards = str_vec[0]
            .chars()
            .map(|c| {
                if c.to_digit(10).is_some() {
                    c.to_digit(10).unwrap() as i32 - 2
                } else {
                    8 + match c {
                        'T' => 0,
                        'J' => 1,
                        'Q' => 2,
                        'K' => 3,
                        'A' => 4,
                        _ => panic!("Invalid card"),
                    }
                }
            })
            .collect::<Vec<i32>>();
        let bid = str_vec[1].parse::<i32>().unwrap();

        Hand::new(cards, bid)
    }

    fn cards_value(&self) -> f32 {
        // 0: High card
        // 1: One pair
        // 2: Two pair
        // 3: Three of a kind
        // 4: Full house
        // 5: Four of a kind
        // 6: Five of a kind
        let mut card_counts = vec![0; 13];
        for card in &self.cards {
            card_counts[*card as usize] += 1;
        }

        // Used to separate same classes of hand based on card values
        let bonus = &self
            .cards
            .iter()
            .enumerate()
            .fold(0.0_f32, |acc, (index, &card)| {
                acc + card as f32 * (1.0 / (13.0_f32).powi((index + 1) as i32))
            });

        if card_counts.contains(&5) {
            return 6.0_f32 + bonus;
        } else if card_counts.contains(&4) {
            return 5.0_f32 + bonus;
        } else if card_counts.contains(&3) {
            if card_counts.contains(&2) {
                return 4.0_f32 + bonus;
            } else {
                return 3.0_f32 + bonus;
            }
        } else if card_counts.contains(&2) {
            if card_counts.iter().filter(|&card| *card == 2).count() == 2 {
                return 2.0_f32 + bonus;
            } else {
                return 1.0_f32 + bonus;
            }
        }

        *bonus
    }
}

fn main() {
    let input: String = fs::read_to_string("input.txt").expect("Unable to read input.txt");
    let mut hands = input
        .split("\n")
        .map(|hand_str| Hand::from_string(hand_str))
        // Sort by hand value
        .collect::<Vec<Hand>>();

    hands.sort_by(|a, b| a.cards_value().partial_cmp(&b.cards_value()).unwrap());

    let winnings = hands
        .iter()
        .zip(1..)
        .fold(0, |acc, (hand, index)| acc + hand.bid * index);

    println!("{:?}", winnings);
}
