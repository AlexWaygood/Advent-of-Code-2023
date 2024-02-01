use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;
use std::str::FromStr;

use anyhow::{bail, Result};
use strum::IntoEnumIterator;
use strum_macros::{EnumIs, EnumIter};

#[derive(EnumIter)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct Point {
    x: i16,
    y: i16,
}

impl Point {
    fn go(&self, direction: &Direction) -> Point {
        let Point { x, y } = *self;
        match direction {
            Direction::North => Point { x, y: y - 1 },
            Direction::South => Point { x, y: y + 1 },
            Direction::East => Point { x: x + 1, y },
            Direction::West => Point { x: x - 1, y },
        }
    }
}

#[derive(EnumIs)]
enum Tile {
    Start,
    GardenPlot,
    Rock,
}

impl TryFrom<&char> for Tile {
    type Error = anyhow::Error;

    fn try_from(s: &char) -> Result<Self> {
        match s {
            'S' => Ok(Self::Start),
            '.' => Ok(Self::GardenPlot),
            '#' => Ok(Self::Rock),
            _ => bail!("Don't know what kind of tile {s} is"),
        }
    }
}

struct PuzzleInput {
    start: Point,
    map: HashMap<Point, Tile>,
    max_x: i16,
    max_y: i16,
}

impl FromStr for PuzzleInput {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut map = HashMap::new();
        let (mut max_x, mut max_y) = (0, 0);
        let mut start = None;
        for (y, line) in s.lines().enumerate() {
            let y = y.try_into()?;
            max_y = y;
            for (x, c) in line.chars().enumerate() {
                let x = x.try_into()?;
                max_x = x;
                let point = Point { x, y };
                let tile = Tile::try_from(&c)?;
                if tile.is_start() {
                    start = Some(point);
                };
                map.insert(point, tile);
            }
        }
        let Some(start) = start else {
            bail!("Couldn't find the starting position!")
        };
        Ok(PuzzleInput {
            start,
            map,
            max_x,
            max_y,
        })
    }
}

fn parse_input(filename: &str) -> Result<PuzzleInput> {
    let input = read_to_string(filename)?;
    PuzzleInput::from_str(&input)
}

fn points_from_here(point: &Point, puzzle_input: &PuzzleInput) -> Vec<Point> {
    Direction::iter()
        .map(|d| point.go(&d))
        .filter(|p| {
            p.x >= 0
                && p.y >= 0
                && p.x <= puzzle_input.max_x
                && p.y <= puzzle_input.max_y
                && puzzle_input.map.get(p).is_some_and(|t| !t.is_rock())
        })
        .collect()
}

const STEPS_TO_TAKE: u8 = 64;

fn solve(puzzle_input: PuzzleInput) -> usize {
    let mut points = HashSet::from([puzzle_input.start]);
    for _ in 0..STEPS_TO_TAKE {
        points = HashSet::from_iter(
            points
                .iter()
                .flat_map(|p| points_from_here(p, &puzzle_input)),
        )
    }
    points.len()
}

fn main() {
    let input = parse_input("input.txt").unwrap();
    println!("{}", solve(input))
}
