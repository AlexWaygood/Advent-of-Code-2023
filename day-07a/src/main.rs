use std::cmp::{Ordering, Reverse};
use std::collections::HashMap;
use std::fmt;
use std::fs::read_to_string;

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Hash, Clone, Copy)]
enum Card {
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
    T = 10,
    J = 11,
    Q = 12,
    K = 13,
    A = 14,
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = *self as i32;
        if value < 11 {
            write!(f, "Card({})", value)
        } else {
            write!(f, "Card({:?})", self)
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
enum HandCategory {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

fn determine_hand_category(card_counts: &[&u8]) -> HandCategory {
    match card_counts {
        [5] => HandCategory::FiveOfAKind,
        [4, 1] => HandCategory::FourOfAKind,
        [3, 2] => HandCategory::FullHouse,
        [3, 1, 1] => HandCategory::ThreeOfAKind,
        [2, 2, 1] => HandCategory::TwoPair,
        [2, ..] => HandCategory::OnePair,
        _ => HandCategory::HighCard,
    }
}

#[derive(PartialEq, Eq)]
struct Hand {
    cards: Vec<Card>,
    bid: u16,
}

impl Hand {
    fn category(&self) -> HandCategory {
        let mut counter: HashMap<Card, u8> = HashMap::new();
        for card in &self.cards {
            *counter.entry(*card).or_insert(0) += 1;
        }
        let mut counter_values: Vec<_> = counter.values().collect();
        counter_values.sort_unstable_by_key(|c| Reverse(**c));
        determine_hand_category(&counter_values)
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let (our_category, other_category) = (self.category(), other.category());
        if our_category != other_category {
            our_category.cmp(&other_category)
        } else {
            self.cards.cmp(&other.cards)
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn winnings_of_hand(hand: &Hand, rank: u16) -> u32 {
    (hand.bid as u32) * (rank as u32)
}

fn total_winnings(mut hands: Vec<Hand>) -> u32 {
    hands.sort();
    assert!(hands[0].category() == HandCategory::HighCard);
    assert!(hands[hands.len() - 1].category() == HandCategory::FiveOfAKind);
    hands
        .iter()
        .enumerate()
        .map(|(index, hand)| winnings_of_hand(hand, (index + 1) as u16))
        .sum()
}

fn parse_input(filename: &str) -> Vec<Hand> {
    let mut hands = Vec::new();
    for line in read_to_string(filename).unwrap().lines() {
        let (unparsed_hand, unparsed_bid) = match line.split_whitespace().collect::<Vec<_>>()[..] {
            [a, b] => (a, b),
            _ => panic!(),
        };
        debug_assert_eq!(unparsed_hand.len(), 5);
        let mut cards = Vec::with_capacity(5);
        for char in unparsed_hand.chars() {
            cards.push(match char {
                '2' => Card::Two,
                '3' => Card::Three,
                '4' => Card::Four,
                '5' => Card::Five,
                '6' => Card::Six,
                '7' => Card::Seven,
                '8' => Card::Eight,
                '9' => Card::Nine,
                'T' => Card::T,
                'J' => Card::J,
                'Q' => Card::Q,
                'K' => Card::K,
                'A' => Card::A,
                _ => panic!("Unexpected char {}", char),
            });
        }
        let bid = unparsed_bid.parse::<u16>().unwrap();
        debug_assert!(bid <= 1000);
        hands.push(Hand { cards, bid });
    }
    assert_eq!(hands.len(), 1000);
    hands
}

fn solve(filename: &str) -> u32 {
    let hands = parse_input(filename);
    total_winnings(hands)
}

fn main() {
    println!("{}", solve("input.txt"));
}
