#[derive(Debug)]
struct HypotheticalRaceAttempt {
    time_held_down: u64,
    available_time: u64,
    record_distance: u64,
}

impl HypotheticalRaceAttempt {
    fn beats_record(&self) -> bool {
        let speed = self.time_held_down;
        let remaining_time = self.available_time - self.time_held_down;
        let distance_travelled = speed * remaining_time;
        distance_travelled > self.record_distance
    }
}

fn ways_to_win(available_time: u64, record_distance: u64) -> u64 {
    let mut total = 0;
    let mut middle_reached = false;
    for time_held_down in (1..available_time).rev() {
        let hypothetical_attempt = HypotheticalRaceAttempt {
            time_held_down,
            available_time,
            record_distance,
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

fn main() {
    let answer = ways_to_win(62649190, 553101014731074);
    println!("{answer}");
}
