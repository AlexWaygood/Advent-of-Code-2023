use std::cmp::{max, min};
use std::fs::read_to_string;

fn parse_input(filename: &str) -> Vec<Vec<String>> {
    read_to_string(filename)
        .expect("Expected input.txt to exist!")
        .replace("\r\n", "\n")
        .split("\n\n")
        .map(|s| s.lines().map(|s| s.to_string()).collect())
        .collect()
}

fn upper_and_lower(i: usize, num_rows_or_cols: usize) -> (usize, usize) {
    let diff = min(i, num_rows_or_cols - i);
    let upper = min(i + diff, num_rows_or_cols);
    let lower = max(0, i - diff);
    (upper, lower)
}

fn reversed_slice(seq: &[String], i: usize, upper: usize) -> Vec<String> {
    let mut slice = Vec::from_iter(seq[i..upper].iter().map(|s| s.to_owned()));
    slice.reverse();
    slice
}

fn find_score(pattern: &[String]) -> u32 {
    let num_rows = pattern.len();
    for i in 1..num_rows {
        let (upper, lower) = upper_and_lower(i, num_rows);
        if pattern[lower..i] == reversed_slice(pattern, i, upper)[..] {
            return (i * 100).try_into().unwrap();
        }
    }

    let num_columns = pattern[0].len();
    let mut columns: Vec<String> = vec![];
    for i in 0..num_columns {
        columns.push(String::from_iter(
            pattern.iter().map(|r| r.chars().nth(i).unwrap()),
        ))
    }
    for i in 1..num_columns {
        let (upper, lower) = upper_and_lower(i, num_columns);
        if columns[lower..i] == reversed_slice(&columns, i, upper)[..] {
            return i.try_into().unwrap();
        }
    }

    unreachable!("Should be unreachable!")
}

fn solve(filename: &str) -> u32 {
    parse_input(filename).iter().map(|p| find_score(p)).sum()
}

fn main() {
    println!("{}", solve("input.txt"));
}
