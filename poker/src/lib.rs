/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
use ::std::collections::HashMap;

pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
    let mut winners: Vec<&'a str> = vec![];
    let mut highest = 0;

    for hand in hands {
        let hand_value = calculate_hand(hand);
        match hand_value {
            x if x > highest => {
                winners = vec![hand];
                highest = hand_value;
            }
            x if x == highest => winners.push(hand),
            _ => (),
        }
    }
    winners
}

fn calculate_hand<'a>(hand: &'a str) -> i32 {
    let mut value = 0;

    for card in hand.split(" ") {
        let card_value = card.parse_card().0;
        if card_value > value {
            value = card_value;
        }
    }
    value
}

trait SplitCard {
    fn split_card(&self) -> (&str, &str);
}

impl SplitCard for str {
    fn split_card(&self) -> (&str, &str) {
        self.split_at(self.len() - 2)
    }
}

trait ParseCard {
    fn parse_card(&self) -> (i32, &str);
}

impl ParseCard for str {
    fn parse_card(&self) -> (i32, &str) {
        let values = HashMap::from([("J", 11), ("Q", 12), ("K", 13), ("A", 14)]);
        let (value_str, suit) = self.split_card();
        if let Ok(value) = value_str.parse::<i32>() {
            return (value, suit);
        }
        let value = values[value_str];
        (value, suit)
    }
}
