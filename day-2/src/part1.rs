use std::collections::HashMap;
use std::convert::TryInto;
use std::fs::read_to_string;
use std::ops::Not;

struct Round {
    red: u32,
    green: u32,
    blue: u32,
}

impl Round {
    fn from_hash_map(mut map: HashMap<&str, u32>) -> Round {
        let red = match map.remove("red") {
            Some(value) => value,
            None => 0,
        };
        let green = match map.remove("green") {
            Some(value) => value,
            None => 0,
        };
        let blue = match map.remove("blue") {
            Some(value) => value,
            None => 0,
        };
        assert!(
            map.is_empty(),
            "Unexpected values are present in the hash map!"
        );
        Round { red, green, blue }
    }

    fn total_cubes(&self) -> u32 {
        self.red + self.green + self.blue
    }

    fn satisfies_constraints(&self, constraints: Round) -> bool {
        self.red <= constraints.red
            && self.green <= constraints.green
            && self.blue <= constraints.blue
            && self.total_cubes() <= constraints.total_cubes()
    }
}

struct Game {
    game_id: u32,
    rounds: Vec<Round>,
}

const CONSTRAINTS: Round = Round {
    red: 12,
    green: 13,
    blue: 14,
};

fn game_was_possible(game: &&Game) -> bool {
    game.rounds
        .iter()
        .any(|r: &Round| r.satisfies_constraints(CONSTRAINTS).not())
        .not()
}

fn parse_input_file(input_filename: &str) -> Vec<Game> {
    let mut given_games: Vec<Game> = Vec::new();
    for (index, game_description) in read_to_string(input_filename).unwrap().lines().enumerate() {
        if game_description.trim() == "" {
            continue;
        }

        match game_description.splitn(2, ": ").last() {
            Some(mut round_descriptions) => {
                round_descriptions = round_descriptions.trim();
                assert!(round_descriptions.contains(":").not());

                let game_id: u32 = (index + 1).try_into().unwrap();
                let mut rounds: Vec<Round> = Vec::new();

                for round_description in round_descriptions.split("; ") {
                    let mut round_data: HashMap<&str, u32> = HashMap::new();

                    for colour_description in round_description.split(", ") {
                        let colour_description_split: Vec<&str> =
                            colour_description.split(" ").collect();

                        match colour_description_split[..] {
                            [number_description, colour] => {
                                let number = number_description.parse::<u32>().unwrap();
                                round_data.insert(colour.trim(), number);
                            }
                            _ => panic!(
                                "Expected colour_description_split to have exactly length 3!"
                            ),
                        };
                    }

                    rounds.push(Round::from_hash_map(round_data))
                }

                given_games.push(Game { game_id, rounds })
            }
            None => panic!("Couldn't find the round_description"),
        };
    }
    given_games
}

fn calculate(input_filename: &str) -> u32 {
    let given_games = parse_input_file(input_filename);
    let possible_games = given_games.iter().filter(game_was_possible);
    possible_games.map(|g: &Game| g.game_id).sum()
}

fn main() {
    let answer = calculate("input.txt");
    println!("{}", answer);
}
