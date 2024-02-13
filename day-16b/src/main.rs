use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct Point {
    x: i16,
    y: i16,
}

impl Point {
    fn go(self, direction: Direction) -> Self {
        let Point { x, y } = self;
        match direction {
            Direction::Left => Self { x: x - 1, y },
            Direction::Right => Self { x: x + 1, y },
            Direction::Up => Self { x, y: y - 1 },
            Direction::Down => Self { x, y: y + 1 },
        }
    }
}

type VisitationRecord = (Point, Direction);

struct Solution {
    max_x: i16,
    max_y: i16,
    node_map: HashMap<Point, char>,
    visitation_record: HashSet<VisitationRecord>,
    visited_nodes: HashSet<Point>,
}

impl Solution {
    fn new(input: String) -> Self {
        let mut node_map = HashMap::new();
        let (mut max_x, mut max_y) = (0, 0);
        for (y, line) in input.lines().enumerate() {
            let y = y.try_into().unwrap();
            max_y = y;
            for (x, c) in line.chars().enumerate() {
                let x = x.try_into().unwrap();
                max_x = x;
                let point = Point { x, y };
                node_map.insert(point, c);
            }
        }
        Solution {
            max_x,
            max_y,
            node_map,
            visitation_record: HashSet::new(),
            visited_nodes: HashSet::new(),
        }
    }

    fn visit_node(&mut self, node: Point, direction: Direction) {
        //println!("{:?}, {:?}", node, direction);
        if node.x < 0 || node.y < 0 {
            return;
        }
        if node.x > self.max_x || node.y > self.max_y {
            return;
        }
        let record = (node, direction);
        // returns `false` if the entry was already present,
        // i.e., we've already traversed this node in that direction
        if !self.visitation_record.insert(record) {
            return;
        }
        self.visited_nodes.insert(node);
        let node_contents = self.node_map[&node];
        match (node_contents, direction) {
            ('.', _) => self.visit_node(node.go(direction), direction),
            ('/', Direction::Down) => self.visit_node(node.go(Direction::Left), Direction::Left),
            ('/', Direction::Up) => self.visit_node(node.go(Direction::Right), Direction::Right),
            ('/', Direction::Right) => self.visit_node(node.go(Direction::Up), Direction::Up),
            ('/', Direction::Left) => self.visit_node(node.go(Direction::Down), Direction::Down),
            ('\\', Direction::Down) => self.visit_node(node.go(Direction::Right), Direction::Right),
            ('\\', Direction::Up) => self.visit_node(node.go(Direction::Left), Direction::Left),
            ('\\', Direction::Right) => self.visit_node(node.go(Direction::Down), Direction::Down),
            ('\\', Direction::Left) => self.visit_node(node.go(Direction::Up), Direction::Up),
            ('|', Direction::Up | Direction::Down) => {
                self.visit_node(node.go(direction), direction)
            }
            ('|', Direction::Left | Direction::Right) => {
                self.visit_node(node.go(Direction::Up), Direction::Up);
                self.visit_node(node.go(Direction::Down), Direction::Down)
            }
            ('-', Direction::Right | Direction::Left) => {
                self.visit_node(node.go(direction), direction)
            }
            ('-', Direction::Up | Direction::Down) => {
                self.visit_node(node.go(Direction::Left), Direction::Left);
                self.visit_node(node.go(Direction::Right), Direction::Right)
            }
            _ => unreachable!("Expected this to be unreachable!"),
        }
    }

    fn num_energised_tiles(&mut self, start_node: Point, start_direction: Direction) -> usize {
        self.visit_node(start_node, start_direction);
        let answer = self.visited_nodes.len();
        self.visitation_record.clear();
        self.visited_nodes.clear();
        answer
    }

    fn solve(&mut self) -> usize {
        let mut possibilities = vec![];
        for x in 0..=self.max_x {
            possibilities.push(self.num_energised_tiles(Point { x, y: 0 }, Direction::Down));
            possibilities.push(self.num_energised_tiles(Point { x, y: self.max_y }, Direction::Up))
        }
        for y in 0..=self.max_y {
            possibilities.push(self.num_energised_tiles(Point { x: 0, y }, Direction::Right));
            possibilities
                .push(self.num_energised_tiles(Point { x: self.max_x, y }, Direction::Left))
        }
        possibilities.iter().max().unwrap().to_owned()
    }
}

fn main() {
    let input = read_to_string("input.txt").unwrap();
    let mut solution = Solution::new(input);
    println!("{}", solution.solve())
}
