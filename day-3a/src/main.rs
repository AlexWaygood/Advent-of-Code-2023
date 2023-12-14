use std::cmp::{max, min};
use std::fs::read_to_string;
use std::iter::Extend;
use std::ops::Not;
use regex::Regex;

struct LocRange {
    start: usize,
    end: usize
}

fn gather_surrounding_chars(
    loc_range: LocRange,
    lineno: usize,
    line: &str,
    all_lines: &Vec<&str>
) -> Vec<char> {
    let mut answer = Vec::new();
    let left = max(0, loc_range.start.wrapping_sub(1));
    let right = min(all_lines[0].len(), loc_range.end);
    match all_lines.get(lineno.wrapping_sub(1)) {
        Some(prev_line) => answer.extend(prev_line[left..right].chars()),
        None => {}
    }
    match all_lines.get(lineno.wrapping_add(1)) {
        Some(next_line) => answer.extend(next_line[left..right].chars()),
        None => {}
    }
    let line_as_bytes = line.as_bytes();
    for c in [line_as_bytes[left], line_as_bytes[right.wrapping_sub(1)]] {
        if c.is_ascii_digit().not() {
            answer.push(c as char)
        }
    };
    answer
}

fn char_is_symbol(c: &char) -> bool{
    let period: &char = &'.';
    c.is_digit(10) && c != period
}

fn is_part_number(loc_range: LocRange, lineno: usize, line: &str, all_lines: &Vec<&str>) -> bool {
    let surrounding_chars = gather_surrounding_chars(loc_range, lineno, line, all_lines);
    surrounding_chars.iter().any(char_is_symbol)
}

fn gather_part_numbers_from_line(lineno: usize, line: &str, all_lines: &Vec<&str>) -> Vec<u32> {
    let mut answer = Vec::new();
    let number_re = Regex::new(r"\d+").unwrap();
    for number_match in number_re.find_iter(line) {
        let loc_range = LocRange{start: number_match.start(), end: number_match.end()};
        if is_part_number(loc_range, lineno, line, all_lines) {
            let parsed_number = number_match.as_str().parse::<u32>().unwrap();
            answer.push(parsed_number)
        }
    }
    answer
}

fn gather_part_numbers_from_file(lines: Vec<&str>) -> Vec<u32> {
    let mut answer = Vec::new();
    for (lineno, line) in lines.iter().enumerate() {
        let found_parts = gather_part_numbers_from_line(lineno, line, &lines);
        answer.extend(found_parts);
    };
    answer
}

fn solve(filename: &str) -> u32 {
    let file = read_to_string(filename).unwrap();
    let lines: Vec<&str> = file.lines().collect();
    gather_part_numbers_from_file(lines).iter().sum()
}

fn main() {
    let answer = solve("src/input.txt");
    println!("{}", answer)
}