use core::fmt;
use std::{collections::HashMap, fs::read_to_string, str::FromStr};

use anyhow::{bail, Context, Result};

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
enum Tile {
    RoundRock,
    CubeRock,
    Empty,
}

impl TryFrom<char> for Tile {
    type Error = anyhow::Error;

    fn try_from(s: char) -> Result<Self> {
        match s {
            'O' => Ok(Tile::RoundRock),
            '#' => Ok(Tile::CubeRock),
            '.' => Ok(Tile::Empty),
            _ => bail!("Can't create a tile from {}", s),
        }
    }
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let c = match self {
            Tile::RoundRock => "O",
            Tile::CubeRock => "#",
            Tile::Empty => ".",
        };
        write!(f, "{c}")
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Coordinate(u32, u32);

impl Coordinate {
    fn from_usize_pair(x: usize, y: usize) -> Result<Self> {
        match (x.try_into(), y.try_into()) {
            (Ok(x1), Ok(x2)) => Ok(Coordinate(x1, x2)),
            _ => bail!("Failed to construct coordinate from ({}, {})", x, y),
        }
    }
}

impl fmt::Display for Coordinate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Coordinate(x, y) = self;
        write!(f, "Coordinate({x}, {y})")
    }
}

type TileMap = HashMap<Coordinate, Tile>;

struct Platform {
    tile_map: TileMap,
    max_x: u32,
    max_y: u32,
}

impl Platform {
    fn tilt_north(&mut self) {
        for x in 0..self.max_x {
            let mut y = 0;
            'outer_column_loop: loop {
                if y >= (self.max_y - 1) {
                    break;
                }
                let coord = Coordinate(x, y);
                let this_tile = self.tile_map[&coord];
                if this_tile != Tile::Empty {
                    y += 1;
                    continue;
                }
                for following_y in (y + 1)..self.max_y {
                    let other_coord = Coordinate(x, following_y);
                    let other_tile = self.tile_map[&other_coord];
                    match other_tile {
                        Tile::CubeRock => {
                            if following_y == (self.max_y - 1) {
                                break 'outer_column_loop;
                            };
                            y = following_y + 1;
                            continue 'outer_column_loop;
                        }
                        Tile::RoundRock => {
                            self.tile_map.insert(coord, Tile::RoundRock);
                            self.tile_map.insert(other_coord, Tile::Empty);
                            if following_y == (self.max_y - 1) {
                                break 'outer_column_loop;
                            };
                            break;
                        }
                        Tile::Empty => {
                            if following_y == (self.max_y - 1) {
                                break 'outer_column_loop;
                            };
                            continue;
                        }
                    }
                }
                y += 1;
            }
        }
    }

    fn tilt_south(&mut self) {
        for x in (0..self.max_x).rev() {
            let mut y = self.max_y - 1;
            'outer_column_loop: loop {
                if y == 0 {
                    break;
                }
                let coord = Coordinate(x, y);
                let this_tile = self.tile_map[&coord];
                if this_tile != Tile::Empty {
                    y -= 1;
                    continue;
                }
                for following_y in (0..y).rev() {
                    let other_coord = Coordinate(x, following_y);
                    let other_tile = self.tile_map[&other_coord];
                    match other_tile {
                        Tile::CubeRock => {
                            if following_y == 0 {
                                break 'outer_column_loop;
                            };
                            y = following_y - 1;
                            continue 'outer_column_loop;
                        }
                        Tile::RoundRock => {
                            self.tile_map.insert(coord, Tile::RoundRock);
                            self.tile_map.insert(other_coord, Tile::Empty);
                            if following_y == 0 {
                                break 'outer_column_loop;
                            };
                            break;
                        }
                        Tile::Empty => {
                            if following_y == 0 {
                                break 'outer_column_loop;
                            };
                            continue;
                        }
                    }
                }
                y -= 1;
            }
        }
    }

    fn tilt_west(&mut self) {
        for y in 0..self.max_y {
            let mut x = 0;
            'outer_column_loop: loop {
                if x == (self.max_x - 1) {
                    break;
                }
                let coord = Coordinate(x, y);
                let this_tile = self.tile_map[&coord];
                if this_tile != Tile::Empty {
                    x += 1;
                    continue;
                }
                for following_x in (x + 1)..self.max_x {
                    let other_coord = Coordinate(following_x, y);
                    let other_tile = self.tile_map[&other_coord];
                    match other_tile {
                        Tile::CubeRock => {
                            if following_x == (self.max_x - 1) {
                                break 'outer_column_loop;
                            };
                            x = following_x + 1;
                            continue 'outer_column_loop;
                        }
                        Tile::RoundRock => {
                            self.tile_map.insert(coord, Tile::RoundRock);
                            self.tile_map.insert(other_coord, Tile::Empty);
                            if following_x == (self.max_x - 1) {
                                break 'outer_column_loop;
                            };
                            break;
                        }
                        Tile::Empty => {
                            if following_x == (self.max_x - 1) {
                                break 'outer_column_loop;
                            };
                            continue;
                        }
                    }
                }
                x += 1;
            }
        }
    }

    fn tilt_east(&mut self) {
        for y in 0..self.max_y {
            let mut x = self.max_x - 1;
            'outer_column_loop: loop {
                if x == 0 {
                    break;
                }
                let coord = Coordinate(x, y);
                let this_tile = self.tile_map[&coord];
                if this_tile != Tile::Empty {
                    x -= 1;
                    continue;
                }
                for following_x in (0..x).rev() {
                    let other_coord = Coordinate(following_x, y);
                    let other_tile = self.tile_map[&other_coord];
                    match other_tile {
                        Tile::CubeRock => {
                            if following_x == 0 {
                                break 'outer_column_loop;
                            };
                            x = following_x - 1;
                            continue 'outer_column_loop;
                        }
                        Tile::RoundRock => {
                            self.tile_map.insert(coord, Tile::RoundRock);
                            self.tile_map.insert(other_coord, Tile::Empty);
                            if following_x == 0 {
                                break 'outer_column_loop;
                            };
                            break;
                        }
                        Tile::Empty => {
                            if following_x == 0 {
                                break 'outer_column_loop;
                            };
                            continue;
                        }
                    }
                }
                x -= 1;
            }
        }
    }

    fn cycle(&mut self) {
        self.tilt_north();
        self.tilt_west();
        self.tilt_south();
        self.tilt_east();
    }

    fn calculate_load(&self) -> u32 {
        let mut answer = 0;
        let y_to_load_map = Vec::from_iter((1..(self.max_y + 1)).rev());
        for x in 0..self.max_x {
            for y in 0..self.max_y {
                let coord = Coordinate(x, y);
                if self.tile_map[&coord] == Tile::RoundRock {
                    answer += y_to_load_map[y as usize];
                }
            }
        }
        answer
    }
}

impl FromStr for Platform {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let lines: Vec<_> = s.lines().collect();
        let mut tile_map = HashMap::new();
        for (y, row) in lines.iter().enumerate() {
            for (x, c) in row.chars().enumerate() {
                let coordinate = Coordinate::from_usize_pair(x, y)?;
                let tile = Tile::try_from(c)?;
                tile_map.insert(coordinate, tile);
            }
        }
        match (lines[0].len().try_into(), lines.len().try_into()) {
            (Ok(max_x), Ok(max_y)) => Ok(Platform {
                tile_map,
                max_x,
                max_y,
            }),
            _ => bail!("Couldn't parse the puzzle input :("),
        }
    }
}

impl fmt::Display for Platform {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = String::new();
        for y in 0..self.max_y {
            for x in 0..self.max_x {
                let coordinate = Coordinate(x, y);
                let tile = self.tile_map[&coordinate];
                s.push_str(&format!("{tile}"))
            }
            s.push('\n')
        }
        f.write_str(s.trim())
    }
}

fn parse_input(filename: &str) -> Result<Platform> {
    read_to_string(filename)
        .context(format!("Expected {} to exist!", filename))?
        .parse()
}

// Given to us in the puzzle description
const NUM_ITERATIONS_REQUIRED: usize = 1000000000;

// Hardcoded number determined by observing the printed output of each iteration,
// and realising that the values were cycling every 18 iterations
const CYCLE_LENGTH: usize = 18;

fn solve(filename: &str) -> u32 {
    let mut platform = parse_input(filename).unwrap();
    let mut previous_record = [0; CYCLE_LENGTH];
    let mut this_record = [1; CYCLE_LENGTH];
    let mut i = 0;
    loop {
        let cycle_step = i % CYCLE_LENGTH;
        if cycle_step == 0 {
            if this_record == previous_record {
                break;
            }
            (previous_record, this_record) = (this_record, previous_record)
        }
        platform.cycle();
        let load = platform.calculate_load();
        this_record[cycle_step] = load;
        i += 1
    }
    let jumps = (NUM_ITERATIONS_REQUIRED - i) % CYCLE_LENGTH;
    for _ in 0..jumps {
        platform.cycle();
    }
    platform.calculate_load()
}

fn main() {
    println!("{}", solve("input.txt"))
}

#[cfg(test)]
mod tests {
    use crate::{parse_input, Coordinate, Platform, Tile};
    use std::{
        collections::{HashMap, HashSet},
        fs::read_to_string,
    };

    const FILENAME: &str = "input.txt";

    fn create_platform() -> Platform {
        parse_input(FILENAME).unwrap()
    }

    #[test]
    fn test_parsing_basics() {
        let platform = create_platform();
        assert_eq!(platform.tile_map.len(), 10_000);
        assert_eq!(platform.max_x, 100);
        assert_eq!(platform.max_y, 100);

        for x in 0..platform.max_x {
            for y in 0..platform.max_y {
                let coordinate = Coordinate(x, y);
                assert!(platform.tile_map.contains_key(&coordinate))
            }
        }
    }

    #[test]
    fn test_parsing_roundtrip() {
        let input = String::from(
            read_to_string("input.txt")
                .unwrap()
                .replace("\r\n", "\n")
                .trim(),
        );
        let platform: Platform = input.parse().unwrap();
        assert_eq!(platform.to_string(), input)
    }

    #[test]
    fn test_tilting_basics() {
        let mut platform = create_platform();
        let tiles: HashMap<Coordinate, Tile> = platform
            .tile_map
            .iter()
            .map(|(k, v)| (k.to_owned(), v.to_owned()))
            .collect();
        assert_eq!(platform.tile_map, tiles);

        platform.tilt_north();
        assert_ne!(platform.tile_map, tiles);
        assert_eq!(platform.tile_map.len(), 10_000);
        assert_eq!(platform.max_x, 100);
        assert_eq!(platform.max_y, 100);

        for x in 0..platform.max_x {
            for y in 0..platform.max_y {
                let coordinate = Coordinate(x, y);
                assert!(platform.tile_map.contains_key(&coordinate))
            }
        }
    }

    #[test]
    fn test_tilting_roundtrip() {
        let mut platform = create_platform();

        platform.tilt_north();
        let platform_display_1 = platform.to_string();
        platform.tilt_south();
        let platform_display_2 = platform.to_string();
        assert_ne!(platform_display_1, platform_display_2);
        platform.tilt_north();
        let platform_display_3 = platform.to_string();
        assert_eq!(platform_display_1, platform_display_3);
        platform.tilt_south();
        let platform_display_4 = platform.to_string();
        assert_eq!(platform_display_2, platform_display_4);

        platform.tilt_east();
        let platform_display_5 = platform.to_string();
        platform.tilt_west();
        let platform_display_6 = platform.to_string();
        assert_ne!(platform_display_5, platform_display_6);
        platform.tilt_east();
        let platform_display_7 = platform.to_string();
        assert_eq!(platform_display_5, platform_display_7);
        platform.tilt_west();
        let platform_display_8 = platform.to_string();
        assert_eq!(platform_display_6, platform_display_8);
    }

    #[test]
    fn test_cycle_basics() {
        let mut platform = create_platform();
        platform.cycle();
        let platform_display = platform.to_string();
        platform.tilt_east();
        let platform_display_2 = platform.to_string();
        assert_eq!(platform_display, platform_display_2)
    }

    #[test]
    fn test_coordinate() {
        let coord = Coordinate(0, 0);
        let coord2 = Coordinate(0, 0);
        assert_eq!(coord, coord2);

        let mut set = HashSet::<Coordinate>::new();
        assert_eq!(set.len(), 0);

        set.insert(coord);
        assert_eq!(set.len(), 1);

        set.insert(coord2);
        assert_eq!(set.len(), 1)
    }

    #[test]
    fn test_tilting_examples() {
        let input = "\
O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";
        let mut platform: Platform = input.parse().unwrap();
        let platform_display = platform.to_string();
        assert_eq!(input, platform_display.as_str());

        let tilted_input = "\
OOOO.#.O..
OO..#....#
OO..O##..O
O..#.OO...
........#.
..#....#.#
..O..#.O.O
..O.......
#....###..
#....#....";
        platform.tilt_north();
        let new_platform_display = platform.to_string();
        assert_eq!(
            tilted_input,
            new_platform_display.as_str(),
            "\n{}",
            new_platform_display
        );
        assert_eq!(platform.calculate_load(), 136)
    }

    #[test]
    fn test_cycle_examples() {
        let input = "\
O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";
        let mut platform: Platform = input.parse().unwrap();
        let platform_display = platform.to_string();
        assert_eq!(input, platform_display.as_str());

        let cycled_input = "\
.....#....
....#...O#
...OO##...
.OO#......
.....OOO#.
.O#...O#.#
....O#....
......OOOO
#...O###..
#..OO#....";
        platform.cycle();
        let cycled_platform_display = platform.to_string();
        assert_eq!(cycled_input, cycled_platform_display.as_str());

        let cycled_input_2 = "\
.....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#..OO###..
#.OOO#...O";
        platform.cycle();
        let cycled_platform_display_2 = platform.to_string();
        assert_eq!(cycled_input_2, cycled_platform_display_2.as_str());

        let cycled_input_3 = "\
.....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#...O###.O
#.OOO#...O";
        platform.cycle();
        let cycled_platform_display_3 = platform.to_string();
        assert_eq!(cycled_input_3, cycled_platform_display_3.as_str());
    }
}
