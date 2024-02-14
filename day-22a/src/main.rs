use std::cell::RefCell;
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;
use std::ops::RangeInclusive;
use std::{fmt::Display, str::FromStr};

use anyhow::{bail, Result};
use itertools::{iproduct, Itertools};

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct XYPoint {
    x: u16,
    y: u16,
}

#[derive(Debug, Clone, Copy)]
struct Brick {
    min_x: u16,
    max_x: u16,
    min_y: u16,
    max_y: u16,
    min_z: u16,
    z_range_len: u16,
}

impl Brick {
    fn max_z(&self) -> u16 {
        self.min_z + self.z_range_len - 1
    }

    fn z_range(&self) -> RangeInclusive<u16> {
        self.min_z..=self.max_z()
    }

    fn fall_by_one(self) -> Self {
        Self {
            min_z: self.min_z - 1,
            ..self
        }
    }

    fn xy_points(&self) -> impl Iterator<Item = XYPoint> {
        let Self {
            min_x,
            max_x,
            min_y,
            max_y,
            ..
        } = *self;
        iproduct!(min_x..=max_x, min_y..=max_y).map(|(x, y)| XYPoint { x, y })
    }
}

impl Display for Brick {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self {
            min_x,
            max_x,
            min_y,
            max_y,
            min_z,
            ..
        } = *self;
        let max_z = self.max_z();
        write!(
            f,
            "Brick(x=({min_x}->{max_x}), y=({min_y}->{max_y}), z=({min_z}->{max_z})"
        )
    }
}

impl FromStr for Brick {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let [left, right] = s.split('~').collect_vec()[..] else {
            bail!("Expected the string to contain a `~`!")
        };
        let (x0, y0, z0) = match left.split(',').collect_vec()[..] {
            [x0, y0, z0] => (x0.parse()?, y0.parse()?, z0.parse()?),
            _ => bail!("Expected the left-hand side to contain exactly two commas!"),
        };
        let (x1, y1, z1) = match right.split(',').collect_vec()[..] {
            [x1, y1, z1] => (x1.parse()?, y1.parse()?, z1.parse()?),
            _ => bail!("Expected the right-hand side to contain exactly two commas!"),
        };
        let (min_z, max_z) = (min(z0, z1), max(z0, z1));
        Ok(Self {
            min_x: min(x0, x1),
            max_x: max(x0, x1),
            min_y: min(y0, y1),
            max_y: max(y0, y1),
            min_z,
            z_range_len: (max_z - min_z) + 1,
        })
    }
}

type ZCoordinate = u16;
type BrickId = u16;
type GridOfGrids = RefCell<HashMap<ZCoordinate, HashMap<XYPoint, BrickId>>>;
type IdToBrickMap = HashMap<BrickId, Brick>;

struct PuzzleInput {
    id_to_brick_map: IdToBrickMap,
    coord_to_brick_map: GridOfGrids,
}

impl FromStr for PuzzleInput {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut id_to_brick_map: IdToBrickMap = HashMap::new();
        for (i, line) in s.lines().enumerate() {
            let i = i.try_into()?;
            let brick = line.parse()?;
            id_to_brick_map.insert(i, brick);
        }
        let coord_to_brick_map: GridOfGrids = RefCell::new(HashMap::new());
        for (id, brick) in &id_to_brick_map {
            for z in brick.z_range() {
                coord_to_brick_map.borrow_mut().entry(z).or_default();
                for point in brick.xy_points() {
                    coord_to_brick_map
                        .borrow_mut()
                        .get_mut(&z)
                        .unwrap()
                        .insert(point, *id);
                }
            }
        }
        Ok(Self {
            id_to_brick_map,
            coord_to_brick_map,
        })
    }
}

fn parse_input(input_filename: &str) -> Result<PuzzleInput> {
    let input = read_to_string(input_filename)?;
    PuzzleInput::from_str(&input)
}

fn drop_brick(brick_id: u16, mut brick: Brick, map: &mut GridOfGrids) -> Brick {
    while brick.min_z > 1 {
        let next_z_down = &(brick.min_z - 1);
        map.borrow_mut().entry(*next_z_down).or_default();
        if !map.borrow().get(next_z_down).unwrap().is_empty()
            && brick
                .xy_points()
                .any(|p| map.borrow().get(next_z_down).unwrap().contains_key(&p))
        {
            break;
        };
        for point in brick.xy_points() {
            map.borrow_mut()
                .get_mut(next_z_down)
                .unwrap()
                .insert(point, brick_id);
            map.borrow_mut()
                .get_mut(&(brick.max_z()))
                .unwrap()
                .remove(&point);
        }
        brick = brick.fall_by_one()
    }
    brick
}

fn has_two_or_more_bricks_below(
    brick: &Brick,
    map: &HashMap<ZCoordinate, HashMap<XYPoint, BrickId>>,
) -> bool {
    let mut brick_ids_below = HashSet::new();
    let grid_below = map.get(&(brick.min_z - 1)).unwrap();
    for point in brick.xy_points() {
        if let Some(brick_id_below) = grid_below.get(&point) {
            brick_ids_below.insert(brick_id_below);
        }
        if brick_ids_below.len() > 1 {
            return true;
        }
    }
    false
}

fn brick_could_safely_be_disintegrated(
    brick: &Brick,
    map: &GridOfGrids,
    id_to_brick_map: &IdToBrickMap,
) -> bool {
    let mut map_borrow = map.borrow_mut();
    let Some(grid_above) = map_borrow.get_mut(&(brick.max_z() + 1)) else {
        return true;
    };
    let mut brick_ids_above = HashSet::new();
    for point in brick.xy_points() {
        if let Some(brick_id_above) = grid_above.get(&point) {
            brick_ids_above.insert(brick_id_above);
        }
    }
    brick_ids_above
        .iter()
        .all(|id| has_two_or_more_bricks_below(id_to_brick_map.get(id).unwrap(), &map.borrow()))
}

fn solve(input_filename: &str) -> usize {
    let input = RefCell::new(parse_input(input_filename).unwrap());
    let mut bricks = {
        let borrowed_input = input.borrow();
        Vec::from_iter(
            borrowed_input
                .id_to_brick_map
                .iter()
                .map(|(id, brick)| (*id, *brick)),
        )
    };
    bricks.sort_unstable_by_key(|(_, brick)| brick.min_z);
    let bricks = bricks
        .iter()
        .map(|(id, brick)| drop_brick(*id, *brick, &mut input.borrow_mut().coord_to_brick_map));
    let mut answer = 0;
    for brick in bricks {
        let input = input.borrow();
        if brick_could_safely_be_disintegrated(
            &brick,
            &input.coord_to_brick_map,
            &input.id_to_brick_map,
        ) {
            answer += 1;
        }
    }
    answer
}

fn main() {
    println!("{}", solve("input.txt"))
}
