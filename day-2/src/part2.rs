use std::cmp::max;
use std::collections::HashMap;
use std::fs::read_to_string;
use std::ops::Not;

struct Round {
    red: u32,
    green: u32,
    blue: u32
}

impl Round {
    fn from_hash_map(mut map: HashMap<&str, u32>) -> Round {
        let red = match map.remove("red") {
            Some(value) => value,
            None => 0
        };
        let green = match map.remove("green") {
            Some(value) => value,
            None => 0
        };
        let blue = match map.remove("blue") {
            Some(value) => value,
            None => 0
        };
        assert!(map.is_empty(), "Unexpected values are present in the hash map!");
        Round{red, green, blue}
    }
}


struct Game {
    rounds: Vec<Round>
}


struct MinimumPossibleCubeSet {
    red: u32,
    green: u32,
    blue: u32
}

impl MinimumPossibleCubeSet {
    fn of_game(game: &Game) -> MinimumPossibleCubeSet {
        let (mut red, mut green, mut blue) = match game.rounds.first() {
            Some(round) => (round.red, round.green, round.blue),
            None => panic!("Unexpectedly passed a game with 0 rounds!")
        };
        for round in &game.rounds[1..] {
            red = max(red, round.red);
            green = max(green, round.green);
            blue = max(blue, round.blue);
        };
        MinimumPossibleCubeSet{red, green, blue}
    }

    fn power(self) -> u32 {
        self.red * self.green * self.blue
    }
}


fn parse_input_file(input_filename: &str) -> Vec<Game> {
    let mut given_games: Vec<Game> = Vec::new();
    for game_description in read_to_string(input_filename)
        .unwrap()
        .lines() {

        if game_description.trim() == "" {
            continue;
        }

        match game_description.splitn(2, ": ").last() {
            Some(mut round_descriptions) => {
                round_descriptions = round_descriptions.trim();
                assert!(round_descriptions.contains(":").not());

                let mut rounds: Vec<Round> = Vec::new();

                for round_description in round_descriptions.split("; ") {
                    let mut round_data: HashMap<&str, u32> = HashMap::new();

                    for colour_description in round_description.split(", ") {
                        let colour_description_split: Vec<&str> = colour_description
                            .split(" ")
                            .collect();

                        match colour_description_split[..] {
                            [number_description, colour] => {
                                let number = number_description.parse::<u32>().unwrap();
                                round_data.insert(colour.trim(), number);
                            },
                            _ => panic!(
                                "Expected colour_description_split to have exactly length 3!"
                            )
                        };
                    };

                    rounds.push(Round::from_hash_map(round_data))
                };

                given_games.push(Game {rounds})
            },
            None => panic!("Couldn't find the round_description")
        };
    };
    given_games
}


fn calculate(input_filename: &str) -> u32 {
    let given_games = parse_input_file(input_filename);
    let possible_cube_sets = given_games.iter().map(MinimumPossibleCubeSet::of_game);
    possible_cube_sets.map(MinimumPossibleCubeSet::power).sum()
}

fn main() {
    let answer = calculate("input.txt");
    println!("{}", answer);
}
