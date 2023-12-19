use std::cmp::{max, min};
use std::collections::HashMap;
use std::fs::read_to_string;
use std::iter::zip;
use std::ops::Range;

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

fn gardening_thing_from_description(description: String) -> GardeningThing {
    match description.as_str() {
        "seed" => GardeningThing::Seed,
        "soil" => GardeningThing::Soil,
        "fertilizer" => GardeningThing::Fertilizer,
        "water" => GardeningThing::Water,
        "light" => GardeningThing::Light,
        "temperature" => GardeningThing::Temperature,
        "humidity" => GardeningThing::Humidity,
        "location" => GardeningThing::Location,
        _ => panic!(),
    }
}

struct MapKind {
    source: GardeningThing,
    destination: GardeningThing,
}

impl MapKind {
    fn from_descriptions(source_description: &str, destination_description: &str) -> MapKind {
        MapKind {
            source: gardening_thing_from_description(String::from(source_description)),
            destination: gardening_thing_from_description(String::from(destination_description)),
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

struct InputMap {
    kind: MapKind,
    rows: Vec<InputDataRow>,
}

fn find_range_overlap(x: &Range<u64>, y: &Range<u64>) -> Range<u64> {
    max(x.start, y.start)..min(x.end, y.end)
}

struct RangeMap {
    kind: MapKind,
    mapping: HashMap<Range<u64>, Range<u64>>,
}

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
    assert_eq!(
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
    _check_range_mapping_consistency(
        &HashMap::from_iter([(
            seed_range.to_owned().to_owned(),
            intermediate_range.to_owned().to_owned(),
        )]),
        &range_mapping,
    );
    if range_mapping.len() > 1 {
        assert!(range_mapping.iter().any(|(key, value)| key != value));
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
        .unwrap();
    for pair in &current_range_map.mapping {
        for (key, value) in progress_range_pair(pair, relevant_input_map) {
            range_mapping.insert(key, value);
        }
    }
    let kind = MapKind {
        source: GardeningThing::Seed,
        destination: relevant_input_map.kind.destination,
    };
    _check_range_mapping_consistency(&current_range_map.mapping, &range_mapping);
    RangeMap {
        kind,
        mapping: range_mapping,
    }
}

struct InputData {
    seed_ranges: Vec<Range<u64>>,
    maps: Vec<InputMap>,
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

fn parse_row_from_input(unparsed_row: &&str) -> InputDataRow {
    match unparsed_row
        .split_whitespace()
        .into_iter()
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<u64>>()[..]
    {
        [destination_start, source_start, range_length] => InputDataRow {
            destination_start,
            source_start,
            range_length,
        },
        _ => panic!(),
    }
}

fn parse_kind_from_input(kind_description: &str) -> MapKind {
    match kind_description.split("-").collect::<Vec<&str>>()[..] {
        [source_description, _, destination_description] => {
            MapKind::from_descriptions(source_description, destination_description)
        }
        _ => panic!(),
    }
}

fn parse_map_from_input(unparsed_map: &&str) -> InputMap {
    match &unparsed_map.split("\r\n").collect::<Vec<&str>>()[..] {
        [first_line, unparsed_rows @ ..] => {
            assert!(unparsed_rows.len() > 1);
            let kind_description = first_line.split(" ").next().unwrap();
            let kind = parse_kind_from_input(kind_description);
            let rows: Vec<InputDataRow> = unparsed_rows.iter().map(parse_row_from_input).collect();
            InputMap { kind, rows }
        }
        _ => panic!(),
    }
}

fn parse_seed_ranges_from_input(seed_description: &str) -> Vec<Range<u64>> {
    seed_description
        .split(" ")
        .enumerate()
        .filter(|(i, _)| *i != 0_usize)
        .map(|(_, s)| s.parse::<u64>().unwrap())
        .collect::<Vec<u64>>()
        .windows(2)
        .enumerate()
        .filter(|(i, _)| (i % 2) == 0)
        .map(|(_, w)| w[0]..(w[0] + w[1]))
        .collect()
}

fn parse_input(filename: &str) -> InputData {
    let puzzle_input = read_to_string(filename).unwrap();
    match &puzzle_input.split("\r\n\r\n").collect::<Vec<&str>>()[..] {
        [unparsed_seeds, unparsed_maps @ ..] => {
            assert!(unparsed_maps.len() > 1);
            let seed_ranges = parse_seed_ranges_from_input(&unparsed_seeds);
            let maps: Vec<InputMap> = unparsed_maps.iter().map(parse_map_from_input).collect();
            InputData { seed_ranges, maps }
        }
        _ => panic!(),
    }
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
