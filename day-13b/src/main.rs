use std::cmp::{max, min};
use std::collections::HashSet;
use std::fs::read_to_string;
use std::iter::zip;

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

type RowOrColumn = HashSet<(usize, char)>;

fn is_match(left: &[RowOrColumn], right: &[RowOrColumn]) -> bool {
    let mut nearly_equal_one_found = false;
    for (l, r) in zip(left, right.iter().rev()) {
        match l.symmetric_difference(r).map(|_| 1).sum() {
            0 => continue,
            1 => unreachable!("Should be unreachable!"),
            2 => {
                if nearly_equal_one_found {
                    return false;
                }
                nearly_equal_one_found = true
            }
            _ => return false,
        }
    }
    nearly_equal_one_found
}

fn find_score(pattern: &Vec<String>) -> u32 {
    let num_rows = pattern.len();

    let rows: Vec<RowOrColumn> = pattern
        .iter()
        .map(|line| HashSet::from_iter(line.chars().enumerate()))
        .collect();
    for i in 1..num_rows {
        let (upper, lower) = upper_and_lower(i, num_rows);
        if is_match(&rows[lower..i], &rows[i..upper]) {
            return (i * 100).try_into().unwrap();
        }
    }

    let num_columns = pattern[0].len();
    let mut columns: Vec<RowOrColumn> = vec![];
    for i in 0..num_columns {
        columns.push(HashSet::from_iter(
            pattern
                .iter()
                .map(|r| r.chars().nth(i).unwrap())
                .enumerate(),
        ))
    }
    for i in 1..num_columns {
        let (upper, lower) = upper_and_lower(i, num_columns);
        if is_match(&columns[lower..i], &columns[i..upper]) {
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
