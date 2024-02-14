/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
use std::{
    collections::{BinaryHeap, HashMap},
    str::FromStr,
};

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

struct Card<'a>(i32, &'a str);

trait ParseCard {
    fn parse_card(&self) -> Result<Card, ParseCardError>;
}
#[derive(Debug, PartialEq, Eq)]
struct ParseCardError;

impl ParseCard for str {
    fn parse_card(&self) -> Result<Card, ParseCardError> {
        let face_values = HashMap::from([("J", 11), ("Q", 12), ("K", 13), ("A", 14)]);
        let (value_str, suit) = self.split_at(self.len() - 1);
        if let Ok(value) = value_str.parse::<i32>() {
            return Ok(Card(value, suit));
        }
        if let value = face_values[value_str] {
            return Ok(Card(value, suit));
        }
        Err(ParseCardError)
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum Category {
    HighCard,
    OnePair,
    TwoPair,
    ThreeKind,
    Straight,
    Flush,
    FullHouse,
    FourKind,
    StraightFlush,
}
struct Hand<'a> {
    hand: &'a str,
    category: Category,
    ranks: BinaryHeap<i32>,
}

#[derive(Debug, PartialEq, Eq)]
struct ParseHandError;

impl From<ParseCardError> for ParseHandError {
    fn from(error: ParseCardError) -> Self {
        ParseHandError
    }
}

impl<'a> FromStr for Hand<'a> {
    type Err = ParseHandError;

    fn from_str(hand: &str) -> Result<Self, Self::Err> {
        let cards: Vec<_> = hand
            .split(" ")
            .map(|card| card.parse_card().unwrap())
            .collect();
        if cards.len() != 5 {
            return Err(ParseHandError);
        }

        let mut hand: Hand<'a>;
        let suit = cards[0].1;
        for card in cards {}
        Ok(hand)
    }
}
