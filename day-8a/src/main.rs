use std::collections::HashMap;
use std::fs::read_to_string;

#[derive(Clone, Copy)]
enum StepKind {
    Left,
    Right
}


#[derive(Clone)]
struct Node {
    place: String,
    leftwards: String,
    rightwards: String
}


fn step<'a>(
    from: &'a Node, direction: &'a StepKind, node_map: &'a HashMap<String, Node>
) -> &'a Node {
    match direction {
        StepKind::Left => &node_map[&from.leftwards],
        StepKind::Right => &node_map[&from.rightwards]
    }
}


fn compute_steps_needed(puzzle_input: PuzzleInput) -> u32 {
    let mut node = puzzle_input.node_map.get("AAA").unwrap();
    let mut steps_taken = 0;
    let mut direction_iter = puzzle_input.step_sequence.iter().cycle();
    while node.place != "ZZZ" {
        let direction = direction_iter.next().unwrap();
        node = step(&node, direction, &puzzle_input.node_map);
        steps_taken += 1;
    };
    steps_taken
}


struct PuzzleInput {
    step_sequence: Vec<StepKind>,
    node_map: HashMap<String, Node>
}


fn parse_input(filename: &str) -> PuzzleInput {
    let unparsed_input = read_to_string(filename).unwrap();
    let [first_line, rest] = match unparsed_input
        .split("\r\n\r\n")
        .collect::<Vec<&str>>()[..] {
            [first, rest] => [first, rest],
            _ => panic!()
        };
    let step_sequence: Vec<StepKind> = first_line
        .chars()
        .map(|c| match c {
                'L' => StepKind::Left,
                'R' => StepKind::Right,
                _ => panic!()
            }
        )
        .collect();
    let mut node_map: HashMap<String, Node> = HashMap::new();
    for line in rest.lines() {
        let [place, rest] = match line.split(" = ").collect::<Vec<&str>>()[..] {
            [place, rest] => [place.to_string(), rest.to_string()],
            _ => panic!()
        };
        let [left, right] = match rest
            .trim_start_matches('(')
            .trim_end_matches(')')
            .split(", ")
            .collect::<Vec<&str>>()[..] {
                [left, right] => [left.to_string(), right.to_string()],
                _ => panic!()
            };
        node_map.insert(
            place.clone(), Node {place, leftwards: left, rightwards: right}
        );
    }
    PuzzleInput {step_sequence, node_map}
}


fn solve(filename: &str) -> u32 {
    let puzzle_input = parse_input(filename);
    compute_steps_needed(puzzle_input)
}

fn main() {
    println!("{}", solve("input.txt"));
}
