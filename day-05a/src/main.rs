use std::fs::read_to_string;
use std::num::ParseIntError;
use std::ops::Range;
use std::str::FromStr;

use anyhow::{bail, Result};

#[derive(PartialEq, Eq, Clone, Copy)]
enum GardeningThing {
    Seed,
    Soil,
    Fertilizer,
    Water,
    Light,
    Temperature,
    Humidity,
    Location,
}

impl FromStr for GardeningThing {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "seed" => Ok(Self::Seed),
            "soil" => Ok(Self::Soil),
            "fertilizer" => Ok(Self::Fertilizer),
            "water" => Ok(Self::Water),
            "light" => Ok(Self::Light),
            "temperature" => Ok(Self::Temperature),
            "humidity" => Ok(Self::Humidity),
            "location" => Ok(Self::Location),
            _ => bail!("Don't know how to create a `Gardening thing from {}", s),
        }
    }
}

struct MapKind {
    source: GardeningThing,
    destination: GardeningThing,
}

impl FromStr for MapKind {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        match s.split('-').collect::<Vec<_>>()[..] {
            [source_description, _, destination_description] => Ok(MapKind {
                source: GardeningThing::from_str(source_description)?,
                destination: GardeningThing::from_str(destination_description)?,
            }),
            _ => bail!("Expected there to only be one '-' character!"),
        }
    }
}

struct InputDataRow {
    destination_start: u32,
    source_start: u32,
    range_length: u32,
}

impl InputDataRow {
    fn source_range(&self) -> Range<u32> {
        self.source_start..(self.source_start.wrapping_add(self.range_length))
    }
}

struct Map {
    kind: MapKind,
    rows: Vec<InputDataRow>,
}

impl Map {
    fn convert(&self, item: u32) -> u32 {
        for row in &self.rows {
            if row.source_range().contains(&item) {
                let difference = item - row.source_start;
                return row.destination_start + difference;
            }
        }
        item
    }
}

impl FromStr for Map {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Map> {
        match s.lines().collect::<Vec<_>>().split_first() {
            Some((first_line, unparsed_rows @ [_, ..])) => {
                let kind_description = first_line.split(' ').next().unwrap();
                let kind = MapKind::from_str(kind_description)?;
                let mut rows = Vec::with_capacity(unparsed_rows.len());
                for unparsed_row in unparsed_rows {
                    rows.push(parse_row_from_input(unparsed_row)?)
                }
                Ok(Map { kind, rows })
            }
            _ => bail!("Expected there to be at least one line"),
        }
    }
}

fn location_from_seed(seed: u32, maps: &[Map]) -> u32 {
    let mut answer = seed;
    let mut thing = &GardeningThing::Seed;
    while thing != &GardeningThing::Location {
        let relevant_map = maps.iter().find(|m| &m.kind.source == thing).unwrap();
        answer = relevant_map.convert(answer);
        thing = &relevant_map.kind.destination;
    }
    answer
}

struct InputData {
    seeds: Vec<u32>,
    maps: Vec<Map>,
}

impl InputData {
    fn seed_locations(&self) -> impl Iterator<Item = u32> + '_ {
        self.seeds
            .iter()
            .map(|s| location_from_seed(*s, &self.maps))
    }
}

impl FromStr for InputData {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let input = s.replace("\r\n", "\n");
        let [unparsed_seeds, unparsed_maps @ ..] = &input.split("\n\n").collect::<Vec<_>>()[..]
        else {
            bail!("Expected there to be a double-newline separating the first line from the rest")
        };
        let seeds = parse_seeds_from_input(unparsed_seeds)?;
        debug_assert!(unparsed_maps.len() > 1);
        let maps = unparsed_maps
            .iter()
            .map(|unparsed_map| Map::from_str(unparsed_map))
            .collect::<Result<Vec<_>>>()?;
        Ok(InputData { seeds, maps })
    }
}

fn parse_row_from_input(unparsed_row: &str) -> Result<InputDataRow> {
    match unparsed_row
        .split_whitespace()
        .map(|s| s.parse())
        .collect::<std::result::Result<Vec<u32>, _>>()?[..]
    {
        [destination_start, source_start, range_length] => Ok(InputDataRow {
            destination_start,
            source_start,
            range_length,
        }),
        _ => bail!("Expected the row to have exactly three items"),
    }
}

fn parse_seeds_from_input(seed_description: &str) -> std::result::Result<Vec<u32>, ParseIntError> {
    seed_description
        .split(' ')
        .skip(1)
        .map(|s| s.parse())
        .collect()
}

fn solve(filename: &str) -> u32 {
    let input =
        read_to_string(filename).unwrap_or_else(|_| panic!("Expected {} to exist", filename));
    let input_data = InputData::from_str(&input).unwrap();
    input_data.seed_locations().min().unwrap()
}

fn main() {
    println!("{}", solve("input.txt"));
}
