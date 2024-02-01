use std::fs::read_to_string;

fn calculate(filename: &str) -> u32 {
    let mut total = 0;
    for line in read_to_string(filename).unwrap().lines() {
        let mut first = None;
        let mut last = None;
        for char in line.chars() {
            if char.is_ascii_digit() {
                if first.is_none() {
                    first = char.to_digit(10)
                };
                last = char.to_digit(10);
            }
        }
        match (first, last) {
            (Some(f), Some(l)) => {
                let calibration_value = (f * 10) + l;
                total += calibration_value;
            }
            _ => panic!(),
        };
    }
    total
}

fn main() {
    println!("{}", calculate("input.txt"));
}
