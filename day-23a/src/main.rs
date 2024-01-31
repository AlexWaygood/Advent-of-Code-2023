use std::collections::{HashMap, HashSet};
use std::fmt::Display;
use std::fs::read_to_string;
use std::hash::Hash;
use std::str::FromStr;

use anyhow::{bail, Result};
use strum::IntoEnumIterator;
use strum_macros::{EnumIs, EnumIter};

#[derive(Debug, Hash, PartialEq, Eq, EnumIter, Clone, Copy)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn all() -> HashSet<Direction> {
        HashSet::from_iter(Direction::iter())
    }
}

#[derive(Debug, Hash, PartialEq, Eq, EnumIs)]
enum Tile {
    Path,
    Forest,
    Slope(Direction),
}

impl Tile {
    fn available_directions(&self) -> HashSet<Direction> {
        match self {
            Tile::Path => Direction::all(),
            Tile::Slope(direction) => HashSet::from([*direction]),
            Tile::Forest => panic!("Looks like we accidentally stepped onto a `Forest` tile!"),
        }
    }

    fn as_char(&self) -> char {
        match self {
            Self::Path => '.',
            Self::Forest => '#',
            Self::Slope(Direction::Down) => 'v',
            Self::Slope(Direction::Up) => '^',
            Self::Slope(Direction::Left) => '<',
            Self::Slope(Direction::Right) => '>',
        }
    }
}

impl TryFrom<&char> for Tile {
    type Error = anyhow::Error;

    fn try_from(s: &char) -> Result<Self> {
        match s {
            '.' => Ok(Self::Path),
            '#' => Ok(Self::Forest),
            '^' => Ok(Self::Slope(Direction::Up)),
            '>' => Ok(Self::Slope(Direction::Right)),
            'v' => Ok(Self::Slope(Direction::Down)),
            '<' => Ok(Self::Slope(Direction::Left)),
            _ => bail!("Don't know what tile {s} is meant to be!"),
        }
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_char())
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct Point {
    x: i16,
    y: i16,
}

impl Point {
    fn new(x: i16, y: i16) -> Self {
        Self { x, y }
    }

    fn go(&self, direction: &Direction) -> Point {
        match direction {
            Direction::Up => Self {
                y: self.y - 1,
                ..*self
            },
            Direction::Down => Self {
                y: self.y + 1,
                ..*self
            },
            Direction::Left => Self {
                x: self.x - 1,
                ..*self
            },
            Direction::Right => Self {
                x: self.x + 1,
                ..*self
            },
        }
    }

    fn available_directions(&self, max_x: &i16, max_y: &i16) -> HashSet<Direction> {
        let mut directions = Direction::all();
        let Point { x, y } = self;
        if x == &0 {
            directions.remove(&Direction::Left);
        } else if x == max_x {
            directions.remove(&Direction::Right);
        }
        if y == &0 {
            directions.remove(&Direction::Up);
        } else if y == max_y {
            directions.remove(&Direction::Down);
        }
        directions
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Point { x, y } = self;
        write!(f, "({x}, {y})")
    }
}

fn possible_next_points(
    point: &Point,
    grid: &Grid,
    route_so_far: &HashSet<Point>,
) -> HashSet<Point> {
    debug_assert_ne!(point, &grid.end_point);
    let tile = &grid.map[point];
    let available_directions_from_point = point.available_directions(&grid.max_x, &grid.max_y);
    let available_directions_from_tile = tile.available_directions();
    HashSet::from_iter(
        available_directions_from_point
            .intersection(&available_directions_from_tile)
            .map(|direction| point.go(direction))
            .filter(|point| !route_so_far.contains(point) && !grid.map[point].is_forest()),
    )
}

struct Grid {
    map: HashMap<Point, Tile>,
    max_x: i16,
    max_y: i16,
    end_point: Point,
}

impl Grid {
    fn new(map: HashMap<Point, Tile>, max_x: i16, max_y: i16) -> Self {
        Grid {
            map,
            max_x,
            max_y,
            end_point: Point {
                x: max_x - 1,
                y: max_y,
            },
        }
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut rows = vec![];
        for y in 0..=self.max_y {
            let mut row = String::new();
            for x in 0..=self.max_x {
                let point = Point::new(x, y);
                let tile = &self.map[&point];
                row.push(tile.as_char())
            }
            debug_assert_eq!(row.len(), ((self.max_x + 1) as usize));
            rows.push(row)
        }
        debug_assert_eq!(rows.len(), ((self.max_y + 1) as usize));
        write!(f, "{}", rows.join("\n"))
    }
}

impl FromStr for Grid {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut map = HashMap::new();
        let (mut max_x, mut max_y) = (0, 0);
        for (y, line) in s.lines().enumerate() {
            let y = y.try_into()?;
            max_y = y;
            for (x, c) in line.chars().enumerate() {
                let x = x.try_into()?;
                max_x = x;
                let point = Point { x, y };
                let tile = Tile::try_from(&c)?;
                map.insert(point, tile);
            }
        }
        Ok(Grid::new(map, max_x, max_y))
    }
}

const START_POINT: Point = Point { x: 1, y: 0 };

fn longest_route_from(point: &Point, grid: &Grid, mut route: HashSet<Point>) -> HashSet<Point> {
    let mut possibilities = possible_next_points(point, grid, &route);
    while possibilities.len() == 1 {
        let next_point = *possibilities.iter().next().unwrap();
        if route.contains(&next_point) {
            return HashSet::new();
        }
        route.insert(next_point);
        if next_point == grid.end_point {
            return route;
        };
        possibilities = possible_next_points(&next_point, grid, &route)
    }
    let mut biggest_possibility = HashSet::new();
    for possibility in possibilities {
        let new_route = &route | &HashSet::from([possibility]);
        let route_from_there = longest_route_from(&possibility, grid, new_route);
        if route_from_there.len() > biggest_possibility.len() {
            biggest_possibility = route_from_there;
        }
    }
    biggest_possibility
}

fn solve(grid: Grid) -> usize {
    longest_route_from(&START_POINT, &grid, HashSet::from([START_POINT])).len() - 1
}

const INPUT_FILENAME: &str = "input.txt";

fn load_input() -> String {
    read_to_string(INPUT_FILENAME).expect("Expected `input.txt` to exist as a file!")
}

fn main() {
    let raw_input = load_input();
    let input = Grid::from_str(&raw_input).unwrap();
    println!("{}", solve(input))
}

#[cfg(test)]
mod tests {
    use std::{collections::HashSet, str::FromStr};

    use crate::{load_input, solve, Direction, Grid, Point, Tile, START_POINT};

    #[test]
    fn test_parsing_tile_roundtrip() {
        let characters = ".#^>v<";
        for character in characters.chars() {
            let parsed = Tile::try_from(&character).unwrap();
            let roundtripped = parsed.as_char();
            assert_eq!(
                roundtripped, character,
                "Parsing {} failed to roundtrip",
                character
            )
        }
    }

    #[test]
    fn test_parsing_input_file() {
        let raw_input = load_input();
        let map = Grid::from_str(&raw_input).unwrap().map;
        let tiles_found: HashSet<&Tile> = HashSet::from_iter(map.values());
        assert!(tiles_found.contains(&Tile::Forest));
        assert!(tiles_found.contains(&Tile::Path));
        assert!(tiles_found.contains(&Tile::Slope(Direction::Down)));
        assert!(tiles_found.contains(&Tile::Slope(Direction::Right)));
    }

    #[test]
    fn test_enum_iteration() {
        assert_eq!(Direction::all().len(), 4)
    }

    #[test]
    fn test_available_directions_of_point() {
        let (max_x, max_y) = (100, 100);
        let point1 = Point::new(0, 0);
        let expected1 = HashSet::from([Direction::Down, Direction::Right]);
        assert_eq!(point1.available_directions(&max_x, &max_y), expected1);

        let point2 = Point::new(1, 0);
        let expected2 = HashSet::from([Direction::Down, Direction::Left, Direction::Right]);
        assert_eq!(point2.available_directions(&max_x, &max_y), expected2);

        let point3 = Point::new(0, 1);
        let expected3 = HashSet::from([Direction::Up, Direction::Down, Direction::Right]);
        assert_eq!(point3.available_directions(&max_x, &max_y), expected3);

        let point4 = Point::new(50, 50);
        let expected4 = HashSet::from([
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ]);
        assert_eq!(point4.available_directions(&max_x, &max_y), expected4);

        let point5 = Point::new(max_x - 1, max_y);
        let expected5 = HashSet::from([Direction::Up, Direction::Left, Direction::Right]);
        assert_eq!(point5.available_directions(&max_x, &max_y), expected5);

        let point6 = Point::new(max_x, max_y - 1);
        let expected6 = HashSet::from([Direction::Up, Direction::Down, Direction::Left]);
        assert_eq!(point6.available_directions(&max_x, &max_y), expected6);

        let point7 = Point::new(max_x, max_y);
        let expected7 = HashSet::from([Direction::Up, Direction::Left]);
        assert_eq!(point7.available_directions(&max_x, &max_y), expected7);
    }

    #[test]
    fn test_available_directions_of_good_tiles() {
        let expected1 = HashSet::from([
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ]);
        assert_eq!(Tile::Path.available_directions(), expected1);

        assert_eq!(
            Tile::Slope(Direction::Down).available_directions(),
            HashSet::from([Direction::Down])
        );
    }

    #[test]
    #[should_panic]
    fn test_available_directions_on_forest_tile_panics() {
        Tile::Forest.available_directions();
    }

    #[test]
    fn test_file_parsing_roundtrip() {
        let raw_input = load_input().replace("\r\n", "\n");
        let parsed = Grid::from_str(&raw_input).unwrap();
        let formatted = format!("{}", parsed);
        assert_eq!(formatted.trim(), raw_input.trim(), "{}", formatted)
    }

    #[test]
    fn test_start() {
        let raw_input = load_input();
        let input = Grid::from_str(&raw_input).unwrap();
        assert_eq!(input.map[&START_POINT], Tile::Path);
    }

    #[test]
    fn test_example() {
        let example = "#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#";
        let grid = Grid::from_str(example).unwrap();
        let answer = solve(grid);
        assert_eq!(answer, 94)
    }
}
