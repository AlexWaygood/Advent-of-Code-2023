use std::collections::{HashMap, VecDeque};
use std::fs::read_to_string;
use std::iter::Sum;
use std::str::FromStr;

use anyhow::{anyhow, bail, Result};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PulseKind {
    High,
    Low,
}

struct PulseRequest {
    kind: PulseKind,
    sender: String,
}

trait Module {
    fn name(&self) -> &str;
    fn connections(&self) -> &Vec<String>;
    fn receive_pulse(&mut self, kind: PulseKind, from_: String) -> Option<PulseRequest>;
    fn send_pulse(&self, kind: PulseKind) -> Option<PulseRequest> {
        Some(PulseRequest {
            kind,
            sender: self.name().to_string(),
        })
    }
}

struct FlipFlopModule {
    _name: String,
    _connections: Vec<String>,
    is_on: bool,
}

impl FlipFlopModule {
    fn new(name: &str, connections: &[String]) -> Self {
        Self {
            _name: name.to_string(),
            _connections: Vec::from(connections),
            is_on: false,
        }
    }
}

impl Module for FlipFlopModule {
    fn name(&self) -> &str {
        self._name.as_str()
    }

    fn connections(&self) -> &Vec<String> {
        &self._connections
    }

    fn receive_pulse(&mut self, kind: PulseKind, _: String) -> Option<PulseRequest> {
        match (self.is_on, kind) {
            (_, PulseKind::High) => None,
            (true, PulseKind::Low) => {
                self.is_on = false;
                self.send_pulse(PulseKind::Low)
            }
            (false, PulseKind::Low) => {
                self.is_on = true;
                self.send_pulse(PulseKind::High)
            }
        }
    }
}

struct ConjunctionModule {
    _name: String,
    _connections: Vec<String>,
    _memory: HashMap<String, PulseKind>,
}

impl ConjunctionModule {
    fn new(name: &str, connections: &[String], inputs: &[String]) -> Self {
        Self {
            _name: name.to_string(),
            _connections: Vec::from(connections),
            _memory: HashMap::from_iter(inputs.iter().map(|s| (s.to_owned(), PulseKind::Low))),
        }
    }
}

impl Module for ConjunctionModule {
    fn name(&self) -> &str {
        self._name.as_str()
    }

    fn connections(&self) -> &Vec<String> {
        &self._connections
    }

    fn receive_pulse(&mut self, kind: PulseKind, from_: String) -> Option<PulseRequest> {
        debug_assert!(self._memory.contains_key(&from_));
        self._memory.insert(from_, kind);
        if self._memory.values().all(|k| k == &PulseKind::High) {
            self.send_pulse(PulseKind::Low)
        } else {
            self.send_pulse(PulseKind::High)
        }
    }
}

struct BroadcastModule {
    _connections: Vec<String>,
}

impl BroadcastModule {
    fn new(connections: &[String]) -> Self {
        Self {
            _connections: Vec::from(connections),
        }
    }
}

impl Module for BroadcastModule {
    fn name(&self) -> &str {
        "broadcaster"
    }

    fn connections(&self) -> &Vec<String> {
        &self._connections
    }

    fn receive_pulse(&mut self, kind: PulseKind, _: String) -> Option<PulseRequest> {
        self.send_pulse(kind)
    }
}

struct UntypedModule {
    _name: String,
    _connections: Vec<String>,
}

impl UntypedModule {
    fn new(name: &str) -> Self {
        Self {
            _name: name.to_string(),
            _connections: vec![],
        }
    }
}

impl Module for UntypedModule {
    fn connections(&self) -> &Vec<String> {
        &self._connections
    }

    fn name(&self) -> &str {
        self._name.as_str()
    }

    fn receive_pulse(&mut self, _: PulseKind, _: String) -> Option<PulseRequest> {
        None
    }
}

struct PulseStatistics {
    high_pulses_sent: u32,
    low_pulses_sent: u32,
}

impl PulseStatistics {
    fn new() -> Self {
        Self {
            high_pulses_sent: 0,
            low_pulses_sent: 1,
        }
    }

    fn update(&mut self, kind: &PulseKind) {
        match kind {
            PulseKind::High => self.high_pulses_sent += 1,
            PulseKind::Low => self.low_pulses_sent += 1,
        }
    }

    fn multiply(&self) -> u32 {
        self.high_pulses_sent * self.low_pulses_sent
    }
}

impl Sum for PulseStatistics {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        let mut high_pulses_sent = 0;
        let mut low_pulses_sent = 0;
        for item in iter {
            high_pulses_sent += item.high_pulses_sent;
            low_pulses_sent += item.low_pulses_sent
        }
        Self {
            high_pulses_sent,
            low_pulses_sent,
        }
    }
}

fn push_button(puzzle_input: &mut HashMap<String, Box<dyn Module>>) -> PulseStatistics {
    let first_request = puzzle_input
        .get_mut("broadcaster")
        .expect("Expected there to be a broadcaster in this map!")
        .receive_pulse(PulseKind::Low, String::from("button"));
    let Some(first_request) = first_request else {
        panic!("Wasn't expecting this to be None!")
    };
    let mut pulse_requests = VecDeque::from([first_request]);
    let mut statistics = PulseStatistics::new();
    loop {
        let Some(request) = pulse_requests.pop_front() else {
            break;
        };
        let connections = Vec::from_iter(
            puzzle_input
                .get(&request.sender)
                .expect(&format!(
                    "Expected {} to be present in the map!",
                    &request.sender
                ))
                .connections()
                .iter()
                .map(|s| s.to_owned()),
        );
        for conn_name in connections {
            statistics.update(&request.kind);
            if let Some(new_request) = puzzle_input
                .get_mut(&conn_name)
                .unwrap()
                .receive_pulse(request.kind, request.sender.to_owned())
            {
                pulse_requests.push_back(new_request)
            }
        }
    }
    debug_assert!(statistics.high_pulses_sent > 0 || statistics.low_pulses_sent > 1);
    statistics
}

fn solve(mut node_map: HashMap<String, Box<dyn Module>>) -> u32 {
    (0..1000)
        .map(|_| push_button(&mut node_map))
        .sum::<PulseStatistics>()
        .multiply()
}

enum ModuleKind {
    FlipFlop(String),
    Conjunction(String),
    Broadcaster,
    // Untyped deliberately omitted here,
    // as it can't appear on the left side of the line
}

impl ModuleKind {
    fn name(&self) -> String {
        match &self {
            &ModuleKind::FlipFlop(name) => name.to_owned(),
            &ModuleKind::Conjunction(name) => name.to_owned(),
            &ModuleKind::Broadcaster => String::from("broadcaster"),
        }
    }
}

impl FromStr for ModuleKind {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "broadcaster" => Ok(ModuleKind::Broadcaster),
            _ => match s.chars().next().unwrap() {
                '&' => Ok(ModuleKind::Conjunction(String::from(&s[1..]))),
                '%' => Ok(ModuleKind::FlipFlop(String::from(&s[1..]))),
                _ => Err(anyhow!("Don't know what module kind {} represents", s)),
            },
        }
    }
}

struct LineInfo {
    kind: ModuleKind,
    connections: Vec<String>,
}

impl FromStr for LineInfo {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let [left, right] = Vec::from_iter(s.trim().split(" -> "))[..] else {
            bail!("Expected every line to have an arrow in the middle!")
        };
        let kind = ModuleKind::from_str(left)?;
        let connections = Vec::from_iter(right.split(", ").map(|x| x.to_string()));
        Ok(Self { kind, connections })
    }
}

fn parse_input(input_lines: Vec<&str>) -> Result<HashMap<String, Box<dyn Module>>> {
    let lines = input_lines
        .iter()
        .map(|l| l.parse())
        .collect::<Result<Vec<LineInfo>>>()?;

    let mut modules = HashMap::new();

    for line in &lines {
        let (name, module): (String, Box<dyn Module>) = match &line.kind {
            ModuleKind::Broadcaster => (
                String::from("broadcaster"),
                Box::new(BroadcastModule::new(&line.connections)),
            ),
            ModuleKind::FlipFlop(name) => (
                name.to_string(),
                Box::new(FlipFlopModule::new(&name, &line.connections)),
            ),
            ModuleKind::Conjunction(name) => {
                let inputs = &lines
                    .iter()
                    .filter(|l| l.connections.contains(&name))
                    .map(|l| l.kind.name().to_owned())
                    .collect::<Vec<String>>();
                (
                    name.to_owned(),
                    Box::new(ConjunctionModule::new(&name, &line.connections, inputs)),
                )
            }
        };
        modules.insert(name.to_owned(), module);
    }

    for line in &lines {
        for name in &line.connections {
            modules
                .entry(name.to_owned())
                .or_insert(Box::new(UntypedModule::new(&name)));
        }
    }

    Ok(modules)
}

fn main() {
    let input = read_to_string("input.txt")
        .expect(format!("Expected 'input.txt' to exist as a file!").as_str());
    let modules = parse_input(Vec::from_iter(input.lines())).unwrap();
    println!("{}", solve(modules))
}
