use std::cmp::min;
use std::collections::{HashMap, HashSet};
use std::fmt::Display;
use std::fs::read_to_string;
use std::iter::once;

use itertools::{chain, Itertools};

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn reverse(&self) -> Self {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct Point {
    x: u8,
    y: u8,
}

impl Point {
    fn new(x: u8, y: u8) -> Self {
        Point { x, y }
    }

    fn go(self, direction: Direction) -> Self {
        let Point { x, y } = self;
        match direction {
            Direction::Up => Point { x, y: y - 1 },
            Direction::Down => Point { x, y: y + 1 },
            Direction::Left => Point { x: x - 1, y },
            Direction::Right => Point { x: x + 1, y },
        }
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Point { x, y } = *self;
        write!(f, "({x}, {y})")
    }
}

fn next_direction_possibilities(
    point: Point,
    direction_history: Vec<&(Point, Direction)>,
    max_x: u8,
    max_y: u8,
) -> HashSet<Direction> {
    let mut possibilities = HashSet::from([
        Direction::Up,
        Direction::Down,
        Direction::Left,
        Direction::Right,
    ]);
    if point.x == 0 {
        possibilities.remove(&Direction::Left);
    } else if point.x == max_x {
        possibilities.remove(&Direction::Right);
    }
    if point.y == 0 {
        possibilities.remove(&Direction::Up);
    } else if point.y == max_y {
        possibilities.remove(&Direction::Down);
    }
    possibilities.remove(&direction_history[0].1.reverse());
    if direction_history
        .iter()
        .rev()
        .take(3)
        .map(|(_, direction)| direction)
        .all_equal()
    {
        possibilities.remove(&direction_history[0].1);
    }
    for possibility in possibilities.clone() {
        if direction_history.contains(&&(point, possibility)) {
            possibilities.remove(&possibility);
        }
    }
    // println!("{}, {:?}, {:?}", point, direction_history, possibilities);
    possibilities
}

type Grid = HashMap<Point, u8>;

fn minimum_cost_from_here(
    point: Point,
    destination: Point,
    grid: &Grid,
    cache: &mut HashMap<(Point, Direction), u32>,
    direction_history: Vec<&(Point, Direction)>,
    mut cost_so_far: u32,
    minimum_found_so_far: u32,
) -> Option<u32> {
    cost_so_far += grid[&point] as u32;
    if cost_so_far >= minimum_found_so_far {
        return None;
    }
    if point == destination {
        return Some(cost_so_far);
    };
    let possible_directions = next_direction_possibilities(
        point,
        direction_history.clone(),
        destination.x,
        destination.y,
    );
    if possible_directions.is_empty() {
        return None;
    };
    let mut possible_costs = vec![];
    for possible_direction in possible_directions {
        let new_point = point.go(possible_direction);
        let cache_key = &(new_point, possible_direction);
        if cache.contains_key(cache_key) {
            let cache_entry = cache.get(cache_key);
            if let Some(cache_entry) = cache_entry {
                possible_costs.push(cache_entry.to_owned())
            }
        } else {
            let new_history = Vec::from_iter(chain(once(cache_key), direction_history.clone()));
            let possible_cost = minimum_cost_from_here(
                point.go(possible_direction),
                destination,
                grid,
                cache,
                new_history,
                cost_so_far,
                minimum_found_so_far,
            );
            if let Some(possible_cost) = possible_cost {
                cache.insert(*cache_key, possible_cost);
                possible_costs.push(possible_cost)
            }
        }
    }
    Some(cost_so_far + possible_costs.iter().min()?.to_owned().to_owned())
}

struct PuzzleInput {
    grid: Grid,
    destination: Point,
}

impl PuzzleInput {
    fn load(filename: &str) -> Self {
        let input =
            read_to_string(filename).unwrap_or_else(|_| panic!("Expected {filename} to exist!"));
        let mut grid = HashMap::new();
        let (mut max_x, mut max_y) = (0, 0);
        for (y, line) in input.lines().enumerate() {
            let y = y.try_into().unwrap();
            max_y = y;
            for (x, c) in line.chars().enumerate() {
                let x = x.try_into().unwrap();
                max_x = x;
                grid.insert(Point { x, y }, c.to_string().as_str().parse().unwrap());
            }
        }
        Self {
            grid,
            destination: Point { x: max_x, y: max_y },
        }
    }
}

fn reasonably_direct_route_cost(input: &PuzzleInput) -> u32 {
    let mut cost = 0_u32;
    let mut point = Point::new(0, 0);
    let mut iterations = 0_u16;
    while point != input.destination {
        if iterations % 2 == 0 {
            point.x += 1
        } else {
            point.y += 1
        }
        cost += input.grid[&point] as u32;
        iterations += 1
    }
    cost
}

fn safe_min(a: u32, b: Option<u32>) -> u32 {
    if let Some(b) = b {
        min(a, b)
    } else {
        a
    }
}

fn solve(input: PuzzleInput) -> u32 {
    let start = Point::new(0, 0);
    let mut cache = HashMap::<(Point, Direction), u32>::new();
    let minimum = reasonably_direct_route_cost(&input);
    let minimum = safe_min(
        minimum,
        minimum_cost_from_here(
            Point::new(0, 1),
            input.destination,
            &input.grid,
            &mut cache,
            vec![&(start, Direction::Down)],
            0,
            minimum,
        ),
    );
    safe_min(
        minimum,
        minimum_cost_from_here(
            Point::new(1, 0),
            input.destination,
            &input.grid,
            &mut cache,
            vec![&(start, Direction::Right)],
            0,
            minimum,
        ),
    )
}

fn main() {
    let input = PuzzleInput::load("input.txt");
    print!("{}", solve(input))
}
