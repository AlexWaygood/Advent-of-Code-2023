use std::fmt::Display;
use std::fs::read_to_string;
use std::str::FromStr;

use anyhow::{bail, Result};

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl FromStr for Direction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "D" => Ok(Direction::Down),
            "U" => Ok(Direction::Up),
            "L" => Ok(Direction::Left),
            "R" => Ok(Direction::Right),
            _ => bail!("Can't create a Direction from {s}"),
        }
    }
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let repr = match self {
            Direction::Down => 'D',
            Direction::Left => 'L',
            Direction::Right => 'R',
            Direction::Up => 'U',
        };
        write!(f, "{repr}")
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn go(&self, direction: Direction) -> Self {
        let Point { x, y } = *self;
        match direction {
            Direction::Up => Self { x, y: y - 1 },
            Direction::Down => Self { x, y: y + 1 },
            Direction::Left => Self { x: x - 1, y },
            Direction::Right => Self { x: x + 1, y },
        }
    }
}

fn find_bounds(instructions: Vec<Direction>) -> Vec<Point> {
    let origin = Point::new(0, 0);
    let mut point = origin;
    let mut points = vec![point];
    for direction in instructions {
        point = point.go(direction);
        points.push(point)
    }
    debug_assert_eq!(points[0], points[points.len() - 1]);
    points.pop();
    points
}

fn apply_shoelace_formula(bounds: Vec<Point>) -> u32 {
    let num_points: i32 = bounds.len().try_into().unwrap();
    // https://en.wikipedia.org/wiki/Shoelace_formula
    let twice_area = bounds
        .windows(2)
        .map(|w| (w[0].x * w[1].y) - (w[0].y * w[1].x))
        .sum::<i32>()
        .abs();
    debug_assert_eq!((twice_area - num_points) % 2, 0);
    let area_excluding_bounds = (twice_area - num_points) / 2 + 1;
    (area_excluding_bounds + num_points).try_into().unwrap()
}

fn parse_input(filename: &str) -> Result<Vec<Direction>> {
    let input = read_to_string(filename)?;
    let mut points = vec![];
    for (lineno, line) in input.lines().enumerate() {
        match line.split(' ').collect::<Vec<_>>()[..] {
            [d, n, _] => {
                let direction = Direction::from_str(d)?;
                let num = u8::from_str(n)?;
                for _ in 0..num {
                    points.push(direction)
                }
            }
            _ => bail!("Unexpected number of spaces in line {}", lineno + 1),
        }
    }
    Ok(points)
}

fn solve(filename: &str) -> u32 {
    let input = parse_input(filename).unwrap();
    let bounds = find_bounds(input);
    apply_shoelace_formula(bounds)
}

fn main() {
    println!("{}", solve("input.txt"));
}
