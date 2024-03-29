use std::collections::HashMap;
use std::fs::read_to_string;

enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Clone, Copy)]
enum Pipe {
    NorthSouth,
    SouthEast,
    EastWest,
    NorthWest,
    SouthWest,
    NorthEast,
}

type Coordinates = (u16, u16);

struct PuzzleInput {
    pipe_map: HashMap<Coordinates, Pipe>,
    start_coordinates: Coordinates,
}

fn solve(puzzle_input: PuzzleInput) -> i64 {
    let start_coords = puzzle_input.start_coordinates;

    let (mut x, mut y) = start_coords;
    let mut coords = (x, y - 1);
    let mut previous_movement = Direction::North;
    let mut relevant_coords: Vec<Coordinates> = vec![start_coords, coords];

    while coords != start_coords {
        (x, y) = coords;
        let node = puzzle_input.pipe_map[&coords];
        (coords, previous_movement) = match (node, previous_movement) {
            (Pipe::NorthSouth, Direction::North) => ((x, y - 1), Direction::North),
            (Pipe::NorthSouth, Direction::South) => ((x, y + 1), Direction::South),
            (Pipe::EastWest, Direction::East) => ((x + 1, y), Direction::East),
            (Pipe::EastWest, Direction::West) => ((x - 1, y), Direction::West),
            (Pipe::SouthEast, Direction::North) => ((x + 1, y), Direction::East),
            (Pipe::SouthEast, Direction::West) => ((x, y + 1), Direction::South),
            (Pipe::NorthWest, Direction::South) => ((x - 1, y), Direction::West),
            (Pipe::NorthWest, Direction::East) => ((x, y - 1), Direction::North),
            (Pipe::SouthWest, Direction::North) => ((x - 1, y), Direction::West),
            (Pipe::SouthWest, Direction::East) => ((x, y + 1), Direction::South),
            (Pipe::NorthEast, Direction::West) => ((x, y - 1), Direction::North),
            (Pipe::NorthEast, Direction::South) => ((x + 1, y), Direction::East),
            _ => panic!(),
        };
        relevant_coords.push(coords)
    }

    // https://en.wikipedia.org/wiki/Shoelace_formula
    let twice_area = relevant_coords
        .windows(2)
        .map(|w| ((w[0].0 as i64) * (w[1].1 as i64)) - ((w[0].1 as i64) * (w[1].0 as i64)))
        .sum::<i64>()
        .abs();
    (twice_area / 2) - (((relevant_coords.len() as i64) / 2) - 1)
}

fn parse_input(filename: &str) -> PuzzleInput {
    let mut pipe_map: HashMap<Coordinates, Pipe> = HashMap::new();
    let mut start_coordinates: Option<Coordinates> = None;
    for (y, line) in read_to_string(filename).unwrap().lines().enumerate() {
        for (x, c) in line.trim().chars().enumerate() {
            let coordinates = (x as u16, y as u16);
            let pipe = match c {
                '.' => continue,
                'S' => {
                    start_coordinates = Some(coordinates);
                    Pipe::NorthSouth
                }
                '|' => Pipe::NorthSouth,
                '-' => Pipe::EastWest,
                'L' => Pipe::NorthEast,
                'J' => Pipe::NorthWest,
                '7' => Pipe::SouthWest,
                'F' => Pipe::SouthEast,
                _ => panic!("Unexpected char {c}"),
            };
            pipe_map.insert(coordinates, pipe);
        }
    }
    match start_coordinates {
        Some((x, y)) => PuzzleInput {
            pipe_map,
            start_coordinates: (x, y),
        },
        None => panic!("Couldn't find the start coordinates!"),
    }
}

fn main() {
    let input = parse_input("input.txt");
    println!("{}", solve(input));
}
