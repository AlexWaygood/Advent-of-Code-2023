use std::collections::{BTreeSet, HashMap, BTreeMap};
use std::fs::read_to_string;
use std::hash::Hash;
use std::ops::Range;

use cached::proc_macro::cached;

#[derive(PartialEq, Eq, Hash, Clone)]
struct Card {
    card_id: u32,
    winning_numbers: BTreeSet<u32>,
    numbers_we_have: BTreeSet<u32>,
}


#[cached]
fn copied_cards_won(card: Card) -> Range<u32> {
    let intersection = card.winning_numbers.intersection(&card.numbers_we_have);
    let num_won = intersection.collect::<Vec<&u32>>().len();
    (card.card_id + 1)..(card.card_id + 1 + num_won as u32)
}


fn parse_input(filename: &str) -> BTreeMap<u32, Card> {
    let mut cards = BTreeMap::new();
    for (index, line) in read_to_string(filename).unwrap().lines().enumerate() {
        match line.split(": ").collect::<Vec<&str>>()[..] {
            [_, data] => {
                match data.split(" | ").collect::<Vec<&str>>()[..] {
                    [left, right] => {
                        let winning_numbers = BTreeSet::<u32>::from_iter(
                            left.split_whitespace().map(|n|n.parse::<u32>().unwrap())
                        );
                        let numbers_we_have = BTreeSet::<u32>::from_iter(
                            right.split_whitespace().map(|n|n.parse::<u32>().unwrap())
                        );
                        let card_id: u32 = (index + 1).try_into().unwrap();
                        let card = Card{card_id, winning_numbers, numbers_we_have};
                        cards.insert(card_id, card);
                    },
                    _ => panic!()
                }
            },
            _ => panic!()
        }
    }
    cards
}


fn compute_total_scratchcards(cards: BTreeMap<u32, Card>) -> u32 {
    let mut counter = cards.values()
        .map(|c| (c, 1_u32))
        .collect::<HashMap<&Card, u32>>();

    for card in cards.values() {
        for card_won_id in copied_cards_won(card.clone()) {
            let count = counter[card];
            counter.entry(&cards[&card_won_id]).and_modify(|c| {*c += count});
        }
    };

    counter.values().sum()
}


fn solve(filename: &str) -> u32 {
    let cards = parse_input(filename);
    compute_total_scratchcards(cards)
}


fn main() {
    println!("{}", solve("input.txt"));
}
