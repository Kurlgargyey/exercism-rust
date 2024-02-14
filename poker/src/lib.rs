/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
use std::{cmp::Ordering, collections::HashMap};

pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
    let mut winners: Vec<&'a str> = vec![];
    let mut highest: Hand<'_> = Hand {
        hand: "",
        category: Category::default(),
    };

    for cards in hands {
        let hand: Hand<'a> = cards.parse_hand().unwrap();
        match hand {
            ref x if x > &highest => {
                winners = vec![&hand.hand];
                highest = hand;
            }
            ref x if x == &highest => winners.push(&hand.hand),
            _ => (),
        }
    }
    winners
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Card<'a>(usize, &'a str);

trait ParseCard {
    fn parse_card(&self) -> Result<Card, ParseCardError>;
}
#[derive(Debug, PartialEq, Eq)]
struct ParseCardError;

impl ParseCard for str {
    fn parse_card(&self) -> Result<Card, ParseCardError> {
        let face_values = HashMap::from([("J", 11), ("Q", 12), ("K", 13), ("A", 14)]);
        let (value_str, suit) = self.split_at(self.len() - 1);
        if let Ok(value) = value_str.parse::<usize>() {
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
    HighCard {
        kickers: Vec<usize>,
    },
    OnePair {
        pair: usize,
        kickers: Vec<usize>,
    },
    TwoPair {
        high_pair: usize,
        low_pair: usize,
        kicker: usize,
    },
    ThreeKind {
        triplet: usize,
        kickers: Vec<usize>,
    },
    Straight {
        high_card: usize,
    },
    Flush {
        kickers: Vec<usize>,
    },
    FullHouse {
        triplet: usize,
        pair: usize,
    },
    FourKind {
        quadruplet: usize,
        kicker: usize,
    },
    StraightFlush {
        high_card: usize,
    },
}

impl Default for Category {
    fn default() -> Self {
        Category::HighCard {
            kickers: vec![6, 5, 4, 3, 1],
        }
    }
}

#[derive(Eq, Default)]
struct Hand<'a> {
    category: Category,
    hand: &'a str,
}

impl Ord for Hand<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.category.cmp(&other.category)
    }
}
impl PartialOrd for Hand<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl PartialEq for Hand<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.category == other.category
    }
}

#[derive(Debug, PartialEq, Eq)]
struct ParseHandError;

impl From<ParseCardError> for ParseHandError {
    fn from(_error: ParseCardError) -> Self {
        ParseHandError
    }
}

trait ParseHand {
    fn parse_hand(&self) -> Result<Hand, ParseHandError>;
}

impl<'a> ParseHand for str {
    fn parse_hand(&self) -> Result<Hand<'_>, ParseHandError> {
        let mut hand = Hand::default();
        hand.hand = self;
        let mut cards: Vec<Card> = self
            .split(" ")
            .map(|card| card.parse_card().unwrap())
            .collect();
        if cards.len() != 5 {
            return Err(ParseHandError);
        }
        cards.sort_by(|card1, card2| card2.0.cmp(&card1.0));
        let categories = (
            is_flush(&cards),
            is_straight(&cards),
            is_four_of_a_kind(&cards),
            is_three_of_a_kind(&cards),
            is_two_pair(&cards),
            is_one_pair(&cards),
        );

        match categories {
            (true, true, false, false, false, false) => hand.category = Category::StraightFlush,
            (true, false, false, false, false, false) => hand.category = Category::Flush,
            (false, true, false, false, false, false) => hand.category = Category::Straight,
            (false, false, true, false, false, false) => hand.category = Category::FourKind,
            (false, false, false, true, false, true) => hand.category = Category::FullHouse,
            (false, false, false, true, false, false) => hand.category = Category::ThreeKind,
            (false, false, false, false, true, false) => {
                let mut first_pair_card = 0;
                for pair in cards.windows(2) {
                    if pair[1].0 == pair[0].0 {
                        first_pair_card = pair[1].0;
                    }
                }
                let mut second_pair_card = 0;
                for pair in cards
                    .iter()
                    .filter(|card| card.0 != first_pair_card)
                    .windows(2)
                {
                    if pair[1].0 == pair[0].0 {
                        first_pair_card = pair[1].0;
                    }
                }
                let mut kicker_card = 0;
                hand.category = Category::TwoPair
            }
            (false, false, false, false, false, true) => {
                let mut pair_card = 0;
                for pair in cards.windows(2) {
                    if pair[1].0 == pair[0].0 {
                        pair_card = pair[1].0;
                    }
                }
                let kicker_cards = cards
                    .iter()
                    .filter(|card| card.0 != pair_card)
                    .map(|card| card.0)
                    .collect();
                hand.category = Category::OnePair {
                    pair: pair_card,
                    kickers: kicker_cards,
                }
            }
            _ => {
                hand.category = Category::HighCard {
                    kickers: cards.iter().map(|card| card.0).collect(),
                }
            }
        }

        Ok(hand)
    }
}

fn is_flush(cards: &Vec<Card>) -> bool {
    let suit: &str = cards[0].1;
    cards.iter().all(|card| card.1 == suit)
}

fn is_straight(cards: &Vec<Card>) -> bool {
    for pair in cards.windows(2) {
        if pair[1].0 - pair[0].0 != 1 {
            return false;
        }
    }
    true
}

fn is_four_of_a_kind(_cards: &Vec<Card>) -> bool {
    let mut _count: usize = 0;
    false
}
fn is_three_of_a_kind(_cards: &Vec<Card>) -> bool {
    false
}
fn is_two_pair(_cards: &Vec<Card>) -> bool {
    false
}
fn is_one_pair(cards: &Vec<Card>) -> bool {
    for pair in cards.windows(2) {
        if pair[1].0 == pair[0].0 {
            return true;
        }
    }
    false
}
