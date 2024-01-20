use std::fs::read_to_string;

fn calculate(filename: &str) -> u32 {
    let one: Vec<char> = "one".chars().collect();
    let two: Vec<char> = "two".chars().collect();
    let three: Vec<char> = "three".chars().collect();
    let four: Vec<char> = "four".chars().collect();
    let five: Vec<char> = "five".chars().collect();
    let six: Vec<char> = "six".chars().collect();
    let seven: Vec<char> = "seven".chars().collect();
    let eight: Vec<char> = "eight".chars().collect();
    let nine: Vec<char> = "nine".chars().collect();

    let mut total = 0;
    for line in read_to_string(filename).unwrap().lines() {
        let mut first = None;
        let mut last = None;
        let line_length = line.len();
        let chars: Vec<char> = line.chars().collect();

        // find first, iterating forwards:
        for i in 0..line_length {
            if first.is_some() {
                break;
            };

            if chars[i].is_ascii_digit() {
                first = chars[i].to_digit(10);
            } else if chars[i..].starts_with(&one) {
                first = Some(1)
            } else if chars[i..].starts_with(&two) {
                first = Some(2)
            } else if chars[i..].starts_with(&three) {
                first = Some(3)
            } else if chars[i..].starts_with(&four) {
                first = Some(4)
            } else if chars[i..].starts_with(&five) {
                first = Some(5)
            } else if chars[i..].starts_with(&six) {
                first = Some(6)
            } else if chars[i..].starts_with(&seven) {
                first = Some(7)
            } else if chars[i..].starts_with(&eight) {
                first = Some(8)
            } else if chars[i..].starts_with(&nine) {
                first = Some(9)
            }
        }

        // find last, iterating backwards:
        for i in (0..line_length).rev() {
            if last.is_some() {
                break;
            };

            if chars[i].is_ascii_digit() {
                last = chars[i].to_digit(10);
            } else if chars[i..].starts_with(&one) {
                last = Some(1)
            } else if chars[i..].starts_with(&two) {
                last = Some(2)
            } else if chars[i..].starts_with(&three) {
                last = Some(3)
            } else if chars[i..].starts_with(&four) {
                last = Some(4)
            } else if chars[i..].starts_with(&five) {
                last = Some(5)
            } else if chars[i..].starts_with(&six) {
                last = Some(6)
            } else if chars[i..].starts_with(&seven) {
                last = Some(7)
            } else if chars[i..].starts_with(&eight) {
                last = Some(8)
            } else if chars[i..].starts_with(&nine) {
                last = Some(9)
            }
        }

        if let (Some(f), Some(l)) = (first, last) {
            let calibration_value = (f * 10) + l;
            total += calibration_value;
        } else {
            panic!()
        };
    }
    total
}

fn main() {
    let answer = calculate("input.txt");
    println!("{}", answer);
}
