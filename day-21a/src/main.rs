use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;
use std::str::FromStr;

use anyhow::{anyhow, bail, Result};
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
        match direction {
            Direction::North => Point {
                y: self.y - 1,
                ..*self
            },
            Direction::South => Point {
                y: self.y + 1,
                ..*self
            },
            Direction::East => Point {
                x: self.x + 1,
                ..*self
            },
            Direction::West => Point {
                x: self.x - 1,
                ..*self
            },
        }
    }
}

#[derive(EnumIs)]
enum Tile {
    Start,
    GardenPlot,
    Rock,
}

impl FromStr for Tile {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "S" => Ok(Self::Start),
            "." => Ok(Self::GardenPlot),
            "#" => Ok(Self::Rock),
            _ => Err(anyhow!("Don't know what kind of tile {} is", s)),
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
                let tile = Tile::from_str(c.to_string().as_str())?;
                if tile.is_start() {
                    start = Some(point);
                };
                map.insert(point, tile);
            }
        }
        let Some(start) = start else {
            bail!("Couldnt' find the starting position!")
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
    Ok(PuzzleInput::from_str(&input)?)
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
                .map(|p| points_from_here(&p, &puzzle_input))
                .flatten(),
        )
    }
    points.len()
}

fn main() {
    let input = parse_input("input.txt").unwrap();
    println!("{}", solve(input))
}
