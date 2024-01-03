use std::cmp::{max, min};
use std::collections::HashMap;
use std::fs::read_to_string;
use std::iter::zip;
use std::ops::Range;
use std::str::FromStr;

use ::anyhow;
use anyhow::{anyhow, bail, Context, Result};

#[derive(PartialEq, Clone, Copy)]
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

    fn from_str(s: &str) -> anyhow::Result<Self, Self::Err> {
        match s {
            "seed" => Ok(GardeningThing::Seed),
            "soil" => Ok(GardeningThing::Soil),
            "fertilizer" => Ok(GardeningThing::Fertilizer),
            "water" => Ok(GardeningThing::Water),
            "light" => Ok(GardeningThing::Light),
            "temperature" => Ok(GardeningThing::Temperature),
            "humidity" => Ok(GardeningThing::Humidity),
            "location" => Ok(GardeningThing::Location),
            _ => Err(anyhow!("Unknown gardening thing {}", s)),
        }
    }
}

struct MapKind {
    source: GardeningThing,
    destination: GardeningThing,
}

impl FromStr for MapKind {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split("-").collect::<Vec<&str>>()[..] {
            [source_description, _, destination_description] => Ok(MapKind {
                source: source_description.parse()?,
                destination: destination_description.parse()?,
            }),
            _ => Err(anyhow!("Can't construct a MapKind from {}", s)),
        }
    }
}

struct InputDataRow {
    destination_start: u64,
    source_start: u64,
    range_length: u64,
}

impl InputDataRow {
    fn source_range(&self) -> Range<u64> {
        self.source_start..(self.source_start + self.range_length)
    }

    fn convert_single(&self, item: u64) -> u64 {
        let source_range = self.source_range();
        assert!(source_range.contains(&item) || item == source_range.end);
        let difference = item - self.source_start;
        self.destination_start + difference
    }

    fn convert_range(&self, r: Range<u64>) -> Range<u64> {
        let start = self.convert_single(r.start);
        let end = self.convert_single(r.end);
        start..end
    }
}

impl FromStr for InputDataRow {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s
            .split_whitespace()
            .map(|s| s.parse())
            .collect::<Result<Vec<u64>, _>>()?[..]
        {
            [destination_start, source_start, range_length] => Ok(InputDataRow {
                destination_start,
                source_start,
                range_length,
            }),
            _ => Err(anyhow!("Couldn't construct an InputDataRow from {}", s)),
        }
    }
}

struct InputMap {
    kind: MapKind,
    rows: Vec<InputDataRow>,
}

impl FromStr for InputMap {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match &s.split("\n").collect::<Vec<&str>>()[..] {
            [first_line, unparsed_rows @ ..] => {
                if unparsed_rows.len() <= 1 {
                    bail!("Expected there to be two or more rows in the map!")
                }
                let kind_description = first_line
                    .split(" ")
                    .next()
                    .context("Expected the first line to have two or more words!")?;
                let kind: MapKind = kind_description.parse()?;
                let rows = unparsed_rows
                    .iter()
                    .map(|s| s.parse())
                    .collect::<Result<Vec<InputDataRow>>>()?;
                Ok(InputMap { kind, rows })
            }
            _ => Err(anyhow!("Couldn't construct an InputMap from {}", s)),
        }
    }
}

fn find_range_overlap(x: &Range<u64>, y: &Range<u64>) -> Range<u64> {
    max(x.start, y.start)..min(x.end, y.end)
}

struct RangeMap {
    kind: MapKind,
    mapping: HashMap<Range<u64>, Range<u64>>,
}

#[cfg(debug_assertions)]
fn _check_range_mapping_consistency(
    initial: &HashMap<Range<u64>, Range<u64>>,
    transformed: &HashMap<Range<u64>, Range<u64>>,
) {
    assert_eq!(
        initial.keys().map(|r| r.start).min().unwrap(),
        transformed.keys().map(|r| r.start).min().unwrap()
    );
    assert_eq!(
        initial.keys().map(|r| r.end).max().unwrap(),
        transformed.keys().map(|r| r.end).max().unwrap()
    );
    assert_eq!(
        initial.keys().map(|r| r.end - r.start).sum::<u64>(),
        transformed.keys().map(|r| r.end - r.start).sum::<u64>()
    );
    assert!(transformed.len() >= initial.len());
}

fn progress_range_pair(
    pair: (&Range<u64>, &Range<u64>),
    input_map: &InputMap,
) -> HashMap<Range<u64>, Range<u64>> {
    let mut range_mapping = HashMap::new();
    let (ref seed_range, ref intermediate_range) = pair;
    debug_assert_eq!(
        (seed_range.end - seed_range.start),
        (intermediate_range.end - intermediate_range.start)
    );
    for row in &input_map.rows {
        let overlap = find_range_overlap(&intermediate_range, &row.source_range());
        if overlap.end > overlap.start {
            let new_key_start = seed_range.start + (overlap.start - intermediate_range.start);
            let new_key_end = seed_range.end - (intermediate_range.end - overlap.end);
            let new_key = new_key_start..new_key_end;
            range_mapping.insert(new_key, row.convert_range(overlap));
        };
    }
    if range_mapping.len() == 0 {
        return HashMap::from_iter([(
            seed_range.to_owned().to_owned(),
            intermediate_range.to_owned().to_owned(),
        )]);
    };
    let mut keys = range_mapping
        .keys()
        .map(|k| k.clone())
        .collect::<Vec<Range<u64>>>();
    keys.sort_unstable_by_key(|r| r.start);
    let (first_key, last_key) = match &keys[..] {
        [first_key, ..] => (first_key, &keys[keys.len() - 1]),
        _ => panic!(),
    };
    if seed_range.start < first_key.start {
        let startfill = seed_range.start..first_key.start;
        let startfill_value =
            intermediate_range.start..(intermediate_range.start + startfill.end - startfill.start);
        range_mapping.insert(startfill, startfill_value);
    }
    if seed_range.end > last_key.end {
        let endfill = last_key.end..seed_range.end;
        let endfill_value =
            (intermediate_range.end + endfill.start - endfill.end)..intermediate_range.end;
        range_mapping.insert(endfill, endfill_value);
    }
    for (this_range, next_range) in zip(&keys[..], &keys[1..]) {
        if this_range.end == next_range.start {
            continue;
        };
        let in_between = this_range.end..next_range.start;
        let in_between_value_start =
            intermediate_range.start + (in_between.start - seed_range.start);
        let in_between_value_end = intermediate_range.end - (seed_range.end - in_between.end);
        range_mapping.insert(in_between, in_between_value_start..in_between_value_end);
    }
    if cfg!(debug_assertions) {
        _check_range_mapping_consistency(
            &HashMap::from_iter([(
                seed_range.to_owned().to_owned(),
                intermediate_range.to_owned().to_owned(),
            )]),
            &range_mapping,
        );
    }
    if range_mapping.len() > 1 {
        debug_assert!(range_mapping.iter().any(|(key, value)| key != value));
    }
    range_mapping
}

fn progress_range_map(current_range_map: RangeMap, input_data: &InputData) -> RangeMap {
    let mut range_mapping = HashMap::<Range<u64>, Range<u64>>::new();
    let relevant_input_map = input_data
        .maps
        .iter()
        .filter(|m| m.kind.source == current_range_map.kind.destination)
        .next()
        .expect("Expected input_data.maps to have length of at least 1!");
    for pair in &current_range_map.mapping {
        for (key, value) in progress_range_pair(pair, relevant_input_map) {
            range_mapping.insert(key, value);
        }
    }
    let kind = MapKind {
        source: GardeningThing::Seed,
        destination: relevant_input_map.kind.destination,
    };
    if cfg!(debug_assertions) {
        _check_range_mapping_consistency(&current_range_map.mapping, &range_mapping);
    }
    RangeMap {
        kind,
        mapping: range_mapping,
    }
}

struct InputData {
    seed_ranges: Vec<Range<u64>>,
    maps: Vec<InputMap>,
}

impl FromStr for InputData {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match &s.replace("\r\n", "\n").split("\n\n").collect::<Vec<&str>>()[..] {
            [unparsed_seeds, unparsed_maps @ ..] => {
                if unparsed_maps.len() <= 1 {
                    bail!("Expected there to be 2 or more maps!")
                }
                let seed_ranges = parse_seed_ranges_from_input(&unparsed_seeds)?;
                let maps = unparsed_maps
                    .iter()
                    .map(|s| s.parse())
                    .collect::<Result<Vec<InputMap>>>()?;
                Ok(InputData { seed_ranges, maps })
            }
            _ => Err(anyhow!("Couldn't parse the input data!")),
        }
    }
}

fn parse_input(filename: &str) -> InputData {
    let puzzle_input =
        read_to_string(filename).expect(format!("Expected file {} to exist", filename).as_str());
    puzzle_input.parse().unwrap()
}

fn seedrange_to_locationrange(input_data: InputData) -> RangeMap {
    let kind = MapKind {
        source: GardeningThing::Seed,
        destination: GardeningThing::Seed,
    };
    let initial_range_map = HashMap::from_iter(
        input_data
            .seed_ranges
            .iter()
            .map(|r| (r.clone(), r.clone())),
    );
    let mut range_map = RangeMap {
        kind,
        mapping: initial_range_map,
    };
    while range_map.kind.destination != GardeningThing::Location {
        range_map = progress_range_map(range_map, &input_data)
    }
    range_map
}

fn parse_seed_ranges_from_input(seed_description: &str) -> Result<Vec<Range<u64>>> {
    Ok(seed_description
        .split(" ")
        .skip(1)
        .map(|s| s.parse::<u64>())
        .collect::<Result<Vec<u64>, _>>()?
        .chunks(2)
        .map(|chunk| chunk[0]..(chunk[0] + chunk[1]))
        .collect())
}

fn solve(filename: &str) -> u64 {
    let input_data = parse_input(filename);
    let range_map = seedrange_to_locationrange(input_data);
    range_map
        .mapping
        .values()
        .min_by_key(|r| r.start)
        .unwrap()
        .start
}

fn main() {
    println!("{}", solve("input.txt"));
}
