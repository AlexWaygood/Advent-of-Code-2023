use std::fs::read_to_string;
use std::iter::zip;

fn find_next_value(history: Vec<i64>) -> i64 {
    let mut differences = history;
    let mut latest = &differences;
    let mut answer = differences[differences.len() - 1];
    while latest.windows(2).any(|w| w[0] != w[1]) {
        differences = zip(latest, &latest[1..])
            .map(|(a, b)| b - a)
            .collect::<Vec<i64>>();
        latest = &differences;
        answer += latest[latest.len() - 1];
    }
    answer
}

fn solve(filename: &str) -> i64 {
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|string| string.parse::<i64>().unwrap())
                .collect()
        })
        .map(find_next_value)
        .sum()
}

fn main() {
    println!("{}", solve("input.txt"));
}
