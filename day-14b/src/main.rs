use core::fmt;
use std::{collections::HashMap, fs::read_to_string, str::FromStr};

use anyhow::{anyhow, Context, Result};

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
enum Tile {
    RoundRock,
    CubeRock,
    Empty,
}

impl FromStr for Tile {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "O" => Ok(Tile::RoundRock),
            "#" => Ok(Tile::CubeRock),
            "." => Ok(Tile::Empty),
            _ => Err(anyhow!("Can't create a tile from {}", s)),
        }
    }
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let c = match self {
            &Tile::RoundRock => "O",
            &Tile::CubeRock => "#",
            &Tile::Empty => ".",
        };
        write!(f, "{}", c)
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Coordinate(u32, u32);

impl Coordinate {
    fn from_usize_pair(x: usize, y: usize) -> Result<Self> {
        match (x.try_into(), y.try_into()) {
            (Ok(x1), Ok(x2)) => Ok(Coordinate(x1, x2)),
            _ => {
                let e = anyhow!("Failed to construct coordinate from ({}, {})", x, y);
                Err(e)
            }
        }
    }
}

impl fmt::Display for Coordinate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Coordinate({}, {})", self.0, self.1)
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
                let this_tile = self.tile_map.get(&coord).unwrap();
                if this_tile != &Tile::Empty {
                    y += 1;
                    continue;
                }
                for following_y in (y + 1)..self.max_y {
                    let other_coord = Coordinate(x, following_y);
                    let other_tile = self.tile_map.get(&other_coord).unwrap();
                    match other_tile {
                        &Tile::CubeRock => {
                            y = following_y + 1;
                            if following_y == (self.max_y - 1) {
                                break 'outer_column_loop;
                            };
                            continue 'outer_column_loop;
                        },
                        &Tile::RoundRock => {
                            self.tile_map.insert(coord, Tile::RoundRock);
                            self.tile_map.insert(other_coord, Tile::Empty);
                            if following_y == (self.max_y - 1) {
                                break 'outer_column_loop;
                            };
                            break;
                        },
                        &Tile::Empty => {
                            if following_y == (self.max_y - 1) {
                                break 'outer_column_loop;
                            };                           
                        }
                    }
                }
                y += 1;
            };
        }
    }

    fn calculate_load(&self) -> u32 {
        let mut answer = 0;
        let y_to_load_map = HashMap::<u32, u32>::from_iter(
            (1..(self.max_y + 1))
                .rev()
                .enumerate()
                .map(|(i, y)| (i.try_into().unwrap(), y)),
        );
        for x in 0..self.max_x {
            for y in 0..self.max_y {
                let coord = Coordinate(x, y);
                if self.tile_map.get(&coord).unwrap() == &Tile::RoundRock {
                    answer += y_to_load_map.get(&y).unwrap();
                }
            }
        }
        answer
    }
}

impl FromStr for Platform {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let lines: Vec<&str> = s.lines().collect();
        let mut tile_map = HashMap::new();
        for (y, row) in lines.iter().enumerate() {
            for (x, c) in row.chars().enumerate() {
                let coordinate = Coordinate::from_usize_pair(x, y).unwrap();
                let tile = c.to_string().parse().unwrap();
                tile_map.insert(coordinate, tile);
            }
        }
        match (lines[0].len().try_into(), lines.len().try_into()) {
            (Ok(max_x), Ok(max_y)) => Ok(Platform {
                tile_map,
                max_x,
                max_y,
            }),
            _ => Err(anyhow!("Couldn't parse the puzzle input :(")),
        }
    }
}

impl fmt::Display for Platform {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = String::new();
        for y in 0..self.max_y {
            for x in 0..self.max_x {
                let coordinate = Coordinate(x, y);
                let tile = self.tile_map.get(&coordinate).unwrap();
                s.push_str(&format!("{}", tile))
            }
            s.push_str("\n")
        }
        f.write_str(&s)
    }
}

fn parse_input(filename: &str) -> Result<Platform> {
    Ok(read_to_string(filename)
        .context(format!("Expected {} to exist!", filename))?
        .parse()?)
}

fn solve(filename: &str) -> u32 {
    let mut platform = parse_input(filename).unwrap();
    platform.tilt_north();
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

    #[test]
    fn test_parsing_basics() {
        let platform = parse_input("input.txt").unwrap();
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
        let platform_display = String::from(format!("{}", platform).trim());
        assert_eq!(platform_display, input)
    }

    #[test]
    fn test_tilting() {
        let mut platform = parse_input("input.txt").unwrap();
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
    fn test_examples() {
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
        let platform_display = String::from(format!("{}", platform).trim());
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
        let new_platform_display = String::from(format!("{}", platform).trim());
        assert_eq!(
            tilted_input,
            new_platform_display.as_str(),
            "\n{}",
            new_platform_display
        );
        assert_eq!(platform.calculate_load(), 136)
    }
}
