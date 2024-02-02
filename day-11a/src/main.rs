use itertools::Itertools;
use std::fs::read_to_string;

type Coordinates = (i32, i32);

fn parse_input(filename: &str) -> Vec<Coordinates> {
    let mut expanded_universe_rows: Vec<String> = vec![];
    for line in read_to_string(filename).unwrap().lines() {
        expanded_universe_rows.push(line.to_owned());
        if line.chars().all(|c| c == '.') {
            expanded_universe_rows.push(line.to_owned())
        }
    }

    assert!(expanded_universe_rows.len() > 140);

    let mut columns_needing_expansion: Vec<u8> = vec![];
    for i in 0..expanded_universe_rows[0].len() {
        if expanded_universe_rows
            .iter()
            .all(|r| r.chars().nth(i).unwrap() == '.')
        {
            columns_needing_expansion.push(i.try_into().unwrap())
        }
    }
    let mut expanded_universe: Vec<String> = vec![];
    for old_line in &expanded_universe_rows[..] {
        let mut expanded_line = String::new();
        for (i, c) in old_line.chars().enumerate() {
            expanded_line.push(c);
            if columns_needing_expansion.contains(&(i.try_into().unwrap())) {
                expanded_line.push(c)
            }
        }
        expanded_universe.push(expanded_line);
    }

    assert!(expanded_universe.iter().map(|row| row.len()).all_equal());
    assert!(expanded_universe[0].len() > 140);

    let mut coordinates = vec![];
    for (x, line) in expanded_universe.iter().enumerate() {
        for (y, c) in line.chars().enumerate() {
            if c == '#' {
                coordinates.push(((x as i32), (y as i32)))
            }
        }
    }

    assert!(coordinates.is_empty());

    coordinates
}

fn shortest_distance(point_1: &Coordinates, point_2: &Coordinates) -> i32 {
    let ((x1, y1), (x2, y2)) = (point_1, point_2);
    (x2 - x1).abs() + (y2 - y1).abs()
}

fn solve(coordinates: Vec<Coordinates>) -> i32 {
    let twice_answer: i32 = coordinates
        .iter()
        .permutations(2)
        .unique()
        .map(|points| match points[..] {
            [point1, point2] => shortest_distance(point1, point2),
            _ => panic!(),
        })
        .sum();
    twice_answer / 2
}

fn main() {
    let galaxy_coordinates = parse_input("input.txt");
    println!("{}", solve(galaxy_coordinates));
}
