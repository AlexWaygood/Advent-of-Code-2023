use std::{fs::read_to_string, iter::zip};

struct HypotheticalRaceAttempt {
    time_held_down: u32,
    available_time: u32,
    record_distance: u32,
}

impl HypotheticalRaceAttempt {
    fn beats_record(&self) -> bool {
        let speed = self.time_held_down;
        let remaining_time = self.available_time - self.time_held_down;
        let distance_travelled = speed * remaining_time;
        distance_travelled > self.record_distance
    }
}

struct ScheduledRace {
    available_time: u32,
    record_distance: u32,
}

impl ScheduledRace {
    fn ways_to_win(&self) -> u32 {
        let mut total = 0;
        let mut middle_reached = false;
        for time_held_down in (1..self.available_time).rev() {
            let hypothetical_attempt = HypotheticalRaceAttempt {
                time_held_down,
                available_time: self.available_time,
                record_distance: self.record_distance,
            };
            match (hypothetical_attempt.beats_record(), middle_reached) {
                (false, false) => continue,
                (true, _) => {
                    total += 1;
                    middle_reached = true;
                }
                (false, true) => break,
            }
        }
        total
    }
}

fn parse_number_list(number_list: &str) -> Vec<u32> {
    let split_line = number_list.split_whitespace().collect::<Vec<_>>();
    let [_, rest @ ..] = &split_line[..] else {
        panic!()
    };
    rest.iter().map(|s| s.parse().unwrap()).collect()
}

fn parse_input(filename: &str) -> Vec<ScheduledRace> {
    let file_contents = read_to_string(filename).unwrap();
    let puzzle_input = file_contents.lines().collect::<Vec<_>>();
    let [first_line, second_line] = puzzle_input[..] else {
        panic!()
    };
    let times = parse_number_list(first_line);
    let distances = parse_number_list(second_line);
    zip(times, distances)
        .map(|(time, distance)| ScheduledRace {
            available_time: time,
            record_distance: distance,
        })
        .collect()
}

fn solve(filename: &str) -> u32 {
    let scheduled_races = parse_input(filename);
    scheduled_races
        .iter()
        .map(|race| race.ways_to_win())
        .product()
}

fn main() {
    println!("{}", solve("input.txt"));
}
