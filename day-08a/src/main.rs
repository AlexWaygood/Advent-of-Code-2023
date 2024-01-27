use std::collections::HashMap;
use std::fs::read_to_string;
use std::str::FromStr;

use anyhow::{bail, Result};

#[derive(Clone, Copy)]
enum StepKind {
    Left,
    Right,
}

impl TryFrom<char> for StepKind {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self> {
        match value {
            'L' => Ok(Self::Left),
            'R' => Ok(Self::Right),
            _ => bail!("Don't know how to create a `StepKind` from {value}"),
        }
    }
}

#[derive(Clone)]
struct Node {
    place: String,
    leftwards: String,
    rightwards: String,
}

fn step<'a>(
    from: &'a Node,
    direction: &'a StepKind,
    node_map: &'a HashMap<String, Node>,
) -> &'a Node {
    match direction {
        StepKind::Left => &node_map[&from.leftwards],
        StepKind::Right => &node_map[&from.rightwards],
    }
}

struct PuzzleInput {
    step_sequence: Vec<StepKind>,
    node_map: HashMap<String, Node>,
}

impl PuzzleInput {
    fn compute_steps_needed(&self) -> u32 {
        let mut node = &self.node_map["AAA"];
        let mut steps_taken = 0;
        let mut direction_iter = self.step_sequence.iter().cycle();
        while node.place != "ZZZ" {
            let direction = direction_iter.next().unwrap();
            node = step(node, direction, &self.node_map);
            steps_taken += 1;
        }
        steps_taken
    }
}

impl FromStr for PuzzleInput {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let unparsed_input = s.replace("\r\n", "\n");
        let [first_line, rest] = unparsed_input.split("\n\n").collect::<Vec<_>>()[..] else {
            bail!("Expected there to be a double line break somewhere")
        };
        let step_sequence: Vec<StepKind> = first_line
            .chars()
            .map(StepKind::try_from)
            .collect::<Result<_>>()?;
        let mut node_map: HashMap<String, Node> = HashMap::new();
        for line in rest.lines() {
            let [place, rest] = line.split(" = ").collect::<Vec<_>>()[..] else {
                bail!("Expected most lines to have an `=` in the middle")
            };
            let place = place.to_string();
            let [left, right] = rest
                .trim_start_matches('(')
                .trim_end_matches(')')
                .split(", ")
                .collect::<Vec<_>>()[..]
            else {
                bail!("Expected there to be exactly two comma-separated items")
            };
            node_map.insert(
                place.clone(),
                Node {
                    place,
                    leftwards: left.to_string(),
                    rightwards: right.to_string(),
                },
            );
        }
        Ok(Self {
            step_sequence,
            node_map,
        })
    }
}

fn solve(filename: &str) -> u32 {
    let unparsed_input = read_to_string(filename).unwrap();
    let puzzle_input = PuzzleInput::from_str(&unparsed_input).unwrap();
    puzzle_input.compute_steps_needed()
}

fn main() {
    println!("{}", solve("input.txt"));
}
