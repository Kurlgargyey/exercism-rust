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
        let value = face_values[value_str];

        return Ok(Card(value, suit));
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
            (true, true, false, false, false, false) => {
                hand.category = build_straight_flush(&cards)
            }
            (true, false, false, false, false, false) => hand.category = build_flush(&cards),
            (false, true, false, false, false, false) => hand.category = build_straight(&cards),
            (false, false, true, false, false, false) => hand.category = count_repeats(&cards),
            (false, false, false, true, false, true) => hand.category = count_repeats(&cards),
            (false, false, false, true, false, false) => hand.category = count_repeats(&cards),
            (false, false, false, false, true, false) => hand.category = count_repeats(&cards),
            (false, false, false, false, false, true) => hand.category = count_repeats(&cards),

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
        if pair[0].0 - pair[1].0 != 1 {
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

fn count_repeats(cards: &Vec<Card>) -> Category {
    let mut card_counts: HashMap<usize, usize> = HashMap::new();

    for card in cards {
        *card_counts.entry(card.0).or_default() += 1;
    }

    let four_kind = card_counts.values().filter(|count| **count == 4).count() == 1;
    if four_kind {
        return build_four_kind(card_counts);
    }
    let two_pair = card_counts.values().filter(|count| **count == 2).count() == 2;
    if two_pair {
        return build_two_pair(card_counts);
    }

    let one_pair = card_counts.values().filter(|count| **count == 2).count() == 1;
    let three_kind = card_counts.values().filter(|count| **count == 3).count() == 1;

    if one_pair && three_kind {
        return build_full_house(card_counts);
    }

    if one_pair {
        return build_one_pair(card_counts);
    }

    if three_kind {
        return build_three_kind(card_counts);
    }

    Category::default()
}

fn build_four_kind(card_counts: HashMap<usize, usize>) -> Category {
    let mut quad = 0;
    let mut kick = 0;
    for count in card_counts {
        if count.1 == 4 {
            quad = count.0;
        } else {
            kick = count.0;
        }
    }
    Category::FourKind {
        quadruplet: quad,
        kicker: kick,
    }
}

fn build_two_pair(card_counts: HashMap<usize, usize>) -> Category {
    let mut pairs = vec![];
    let mut kick: usize = 0;

    for count in card_counts {
        if count.1 == 2 {
            pairs.push(count.0);
        } else {
            kick = count.0;
        }
    }

    pairs.sort_unstable_by(|a, b| b.partial_cmp(a).unwrap());

    Category::TwoPair {
        high_pair: pairs[0],
        low_pair: pairs[1],
        kicker: kick,
    }
}

fn build_full_house(card_counts: HashMap<usize, usize>) -> Category {
    let mut triplet: usize = 0;
    let mut pair: usize = 0;

    for count in card_counts {
        if count.1 == 3 {
            triplet = count.0;
        } else {
            pair = count.0;
        }
    }

    Category::FullHouse {
        triplet: triplet,
        pair: pair,
    }
}

fn build_one_pair(card_counts: HashMap<usize, usize>) -> Category {
    let mut pair: usize = 0;
    let mut kickers = vec![];

    for count in card_counts {
        if count.1 == 2 {
            pair = count.0;
        } else {
            kickers.push(count.0);
        }
    }
    kickers.sort_unstable_by(|a, b| b.partial_cmp(a).unwrap());
    Category::OnePair {
        pair: pair,
        kickers: kickers,
    }
}

fn build_three_kind(card_counts: HashMap<usize, usize>) -> Category {
    let mut triplet = 0;
    let mut kickers = vec![];
    for count in card_counts {
        if count.1 == 3 {
            triplet = count.0;
        } else {
            kickers.push(count.0);
        }
    }
    kickers.sort_unstable_by(|a, b| b.partial_cmp(a).unwrap());
    Category::ThreeKind {
        triplet: triplet,
        kickers: kickers,
    }
}

fn build_straight(cards: &Vec<Card>) -> Category {
    Category::Straight {
        high_card: cards[0].0,
    }
}

fn build_straight_flush(cards: &Vec<Card>) -> Category {
    Category::StraightFlush {
        high_card: cards[0].0,
    }
}

fn build_flush(cards: &Vec<Card>) -> Category {
    let mut kickers = vec![];

    for card in cards {
        kickers.push(card.0);
    }
    kickers.sort_unstable_by(|a, b| b.partial_cmp(a).unwrap());
    Category::Flush { kickers: kickers }
}
