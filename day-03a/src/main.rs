use std::cmp::min;
use std::collections::HashSet;
use std::fs::read_to_string;
use std::ops::Range;

fn gather_surrounding_chars(
    loc_range: Range<usize>,
    lineno: usize,
    line: &str,
    all_lines: &[&str],
) -> HashSet<char> {
    let left = loc_range.start.saturating_sub(1);
    let right = min(all_lines[0].len() - 1, loc_range.end);
    let mut answer = HashSet::new();
    if let Some(prev_line) = all_lines.get(lineno.saturating_sub(1)) {
        answer.extend(prev_line[left..=right].chars());
    }
    if let Some(next_line) = all_lines.get(lineno + 1) {
        answer.extend(next_line[left..=right].chars());
    }
    let line_as_bytes = line.as_bytes();
    answer.insert(line_as_bytes[left].into());
    answer.insert(line_as_bytes[right].into());
    answer
}

fn char_is_symbol(c: &char) -> bool {
    c != &'.' && !c.is_ascii_digit()
}

fn is_part_number(loc_range: Range<usize>, lineno: usize, line: &str, all_lines: &[&str]) -> bool {
    gather_surrounding_chars(loc_range, lineno, line, all_lines)
        .iter()
        .any(char_is_symbol)
}

fn gather_part_numbers_from_line(lineno: usize, line: &str, all_lines: &[&str]) -> Vec<u32> {
    let number_re = regex::Regex::new(r"\d+").expect("Thought this would be a valid regex");
    number_re
        .find_iter(line)
        .filter(|needle| is_part_number(needle.range(), lineno, line, all_lines))
        .map(|needle| {
            needle
                .as_str()
                .parse()
                .expect("Expected this to parse as a number")
        })
        .collect()
}

fn gather_part_numbers_from_file(input: String) -> Vec<u32> {
    let lines: Vec<&str> = input.lines().collect();
    lines
        .iter()
        .enumerate()
        .flat_map(|(lineno, line)| gather_part_numbers_from_line(lineno, line, &lines))
        .collect()
}

fn read_input(filename: &str) -> String {
    read_to_string(filename).unwrap_or_else(|_| panic!("Expected {filename} to exist"))
}

fn solve(filename: &str) -> u32 {
    let input = read_input(filename);
    gather_part_numbers_from_file(input).iter().sum()
}

fn main() {
    println!("{}", solve("input.txt"));
}
