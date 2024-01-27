use std::collections::HashSet;
use std::fs::read_to_string;

struct Card {
    winning_numbers: HashSet<u32>,
    numbers_we_have: HashSet<u32>,
}

impl Card {
    fn total_points(&self) -> u32 {
        let intersection = self.winning_numbers.intersection(&self.numbers_we_have);
        match intersection.count() {
            0 => 0,
            number => 2_u32.pow((number as u32) - 1),
        }
    }
}

fn parse_input(filename: &str) -> Vec<Card> {
    let mut cards = vec![];
    for line in read_to_string(filename).unwrap().lines() {
        let [_, data] = line.split(": ").collect::<Vec<&str>>()[..] else {
            panic!()
        };
        let [left, right] = data.split(" | ").collect::<Vec<&str>>()[..] else {
            panic!()
        };
        let winning_numbers =
            HashSet::<u32>::from_iter(left.split_whitespace().map(|n| n.parse::<u32>().unwrap()));
        let numbers_we_have =
            HashSet::<u32>::from_iter(right.split_whitespace().map(|n| n.parse::<u32>().unwrap()));
        cards.push(Card {
            winning_numbers,
            numbers_we_have,
        })
    }
    cards
}

fn solve(filename: &str) -> u32 {
    parse_input(filename).iter().map(|c| c.total_points()).sum()
}

fn main() {
    println!("{}", solve("input.txt"));
}
