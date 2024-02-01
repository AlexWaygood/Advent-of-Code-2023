use std::fs::read_to_string;

use cached::proc_macro::cached;

#[cached]
fn run_algorithm(step: String) -> u8 {
    debug_assert!(step.is_ascii());
    let mut answer: u32 = 0;
    for byte in step.bytes() {
        answer += byte as u32;
        answer *= 17;
        answer %= 256
    }
    answer.try_into().expect("Expected result to be <256!")
}

fn read_input(filename: &str) -> String {
    read_to_string(filename).unwrap_or_else(|_| panic!("Expected {filename} to exist!"))
}

fn solve(filename: &str) -> u32 {
    read_input(filename)
        .split(',')
        .map(|step| (run_algorithm(step.to_string()) as u32))
        .sum()
}

fn main() {
    println!("{}", solve("input.txt"));
}
