use std::{fs::read_to_string, ops::Range};

use cached::proc_macro::cached;

#[derive(PartialEq, Clone)]
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

#[cached]
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

fn location_from_seed(seed: u32, maps: &Vec<Map>) -> u32 {
    let mut answer = seed;
    let mut thing = &GardeningThing::Seed;
    while thing != &GardeningThing::Location {
        let relevant_map = maps
            .iter()
            .filter(|m| &m.kind.source == thing)
            .next()
            .unwrap();
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

fn parse_row_from_input(unparsed_row: &str) -> InputDataRow {
    match unparsed_row
        .split_whitespace()
        .into_iter()
        .map(|s| s.parse::<u32>().unwrap())
        .collect::<Vec<u32>>()[..]
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

fn parse_map_from_input(unparsed_map: &str) -> Map {
    match &unparsed_map.split("\r\n").collect::<Vec<&str>>()[..] {
        [first_line, unparsed_rows @ ..] => {
            assert!(unparsed_rows.len() > 1);
            let kind_description = first_line.split(" ").next().unwrap();
            let kind = parse_kind_from_input(kind_description);
            let mut rows = Vec::<InputDataRow>::new();
            for unparsed_row in unparsed_rows {
                rows.push(parse_row_from_input(&unparsed_row))
            }
            Map { kind, rows }
        }
        _ => panic!(),
    }
}

fn parse_seeds_from_input(seed_description: &str) -> Vec<u32> {
    seed_description.split(" ").collect::<Vec<&str>>()[1..]
        .iter()
        .map(|s| s.parse::<u32>().unwrap())
        .collect::<Vec<u32>>()
}

fn parse_input(filename: &str) -> InputData {
    let puzzle_input = read_to_string(filename).unwrap();
    let mut maps = Vec::<Map>::new();
    let seeds = match &puzzle_input.split("\r\n\r\n").collect::<Vec<&str>>()[..] {
        [unparsed_seeds, unparsed_maps @ ..] => {
            assert!(unparsed_maps.len() > 1);
            let seeds = parse_seeds_from_input(&unparsed_seeds);
            for unparsed_map in unparsed_maps {
                maps.push(parse_map_from_input(unparsed_map))
            }
            seeds
        }
        _ => panic!(),
    };
    InputData { seeds, maps }
}

fn solve(filename: &str) -> u32 {
    let input_data = parse_input(filename);
    input_data.seed_locations().min().unwrap()
}

fn main() {
    println!("{}", solve("input.txt"));
}
