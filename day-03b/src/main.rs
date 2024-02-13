use std::cmp::min;
use std::fs::read_to_string;

use once_cell::sync::Lazy;
use regex::Regex;

fn read_input(filename: &str) -> String {
    read_to_string(filename).unwrap_or_else(|_| panic!("Expected {filename} to exist"))
}

fn get_gear_ratio(index: usize, all_lines: &[&str], lineno: usize, line_length: usize) -> u32 {
    let line = all_lines[lineno];
    let c = line.chars().nth(index).unwrap();
    if c != '*' {
        return 0;
    }
    static NUMBER_RE: Lazy<Regex> =
        Lazy::new(|| Regex::new(r"\d{1,3}").expect("Expected this to be a valid regex"));
    let range_to_search = index.saturating_sub(3)..=min(index + 3, line_length);
    let haystacks = [
        &line[range_to_search.clone()],
        &all_lines[lineno - 1][range_to_search.clone()],
        &all_lines[lineno + 1][range_to_search],
    ];
    let matches: Vec<_> = haystacks
        .iter()
        .flat_map(|haystack| NUMBER_RE.find_iter(haystack))
        .filter(|m| (2..=4).any(|i| m.range().contains(&i)))
        .take(3)
        .collect();
    if matches.len() != 2 {
        return 0;
    }
    matches
        .iter()
        .map(|m| {
            m.as_str()
                .parse::<u32>()
                .expect("Expected all matches to parse as integers")
        })
        .product()
}

fn get_gear_ratio_sum_in_line(all_lines: &[&str], lineno: usize, line_length: usize) -> u32 {
    (0..line_length)
        .map(|index| get_gear_ratio(index, all_lines, lineno, line_length))
        .sum()
}

fn solve(filename: &str) -> u32 {
    let input = read_input(filename);
    let lines: Vec<&str> = input.lines().collect();
    let line_length = lines[0].len();
    (1..(lines.len() - 1))
        .map(|lineno| get_gear_ratio_sum_in_line(&lines, lineno, line_length))
        .sum()
}

fn main() {
    println!("{}", solve("input.txt"));
}
