use std::collections::HashMap;
use std::fmt::Display;
use std::fs::read_to_string;
use std::str::FromStr;

use anyhow::{bail, Context, Error, Result};

#[derive(Debug)]
enum Decision {
    Accept,
    Reject,
    OtherWorkflow(String),
}

impl FromStr for Decision {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "A" => Ok(Self::Accept),
            "R" => Ok(Self::Reject),
            _ => Ok(Self::OtherWorkflow(s.to_string())),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Part {
    x: u32,
    m: u32,
    a: u32,
    s: u32,
}

impl Part {
    fn score(&self) -> u32 {
        self.x + self.m + self.a + self.s
    }
}

impl FromStr for Part {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut data = HashMap::new();
        let sections = s[1..(s.len() - 1)].split(",");
        for section in sections {
            let split_section = Vec::from_iter(section.split("="));
            let rating = u32::from_str(split_section[split_section.len() - 1])?;
            data.insert(split_section[0], rating);
        }
        let x = *data.get("x").context("Expected 'x' to be present in the part description!")?;
        let m = *data.get("m").context("Expected 'm' to be present in the part description!")?;
        let a = *data.get("a").context("Expected 'a' to be present in the part description!")?;
        let s = *data.get("s").context("Expected 's' to be present in the part description!")?;
        Ok(Self {x, m, a, s})
    }
}

#[derive(Debug, Clone, Copy)]
enum Compare {
    Lt,
    Gt,
    NoOp,
}

impl FromStr for Compare {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            ">" => Ok(Self::Gt),
            "<" => Ok(Self::Lt),
            _ => bail!("Don't know how to create a `Compare` variant from {}", s),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Attr {
    X,
    M,
    A,
    S,
}

impl FromStr for Attr {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "x" => Ok(Attr::X),
            "m" => Ok(Attr::M),
            "a" => Ok(Attr::A),
            "s" => Ok(Attr::S),
            _ => bail!("Don't know how to create an `Attr` from {}", s),
        }
    }
}

struct Rule {
    attr: Option<Attr>,
    cmp: Compare,
    value: u32,
    outcome: Decision,
}

impl Rule {
    fn new(attr: Attr, cmp: Compare, value: u32, outcome: Decision) -> Self {
        assert!(!matches!(cmp, Compare::NoOp));
        Rule {
            attr: Some(attr),
            cmp,
            value,
            outcome,
        }
    }

    fn noop(outcome: Decision) -> Self {
        Rule {
            attr: None,
            cmp: Compare::NoOp,
            value: 0,
            outcome,
        }
    }

    fn process(&self, part: &Part) -> Option<Decision> {
        let inner: Box<dyn Fn(&Part) -> bool> = match (self.attr, self.cmp) {
            (Some(Attr::X), Compare::Gt) => Box::new(|p: &Part| p.x > self.value),
            (Some(Attr::X), Compare::Lt) => Box::new(|p: &Part| p.x < self.value),
            (Some(Attr::M), Compare::Gt) => Box::new(|p: &Part| p.m > self.value),
            (Some(Attr::M), Compare::Lt) => Box::new(|p: &Part| p.m < self.value),
            (Some(Attr::A), Compare::Gt) => Box::new(|p: &Part| p.a > self.value),
            (Some(Attr::A), Compare::Lt) => Box::new(|p: &Part| p.a < self.value),
            (Some(Attr::S), Compare::Gt) => Box::new(|p: &Part| p.s > self.value),
            (Some(Attr::S), Compare::Lt) => Box::new(|p: &Part| p.s < self.value),
            (None, Compare::NoOp) => Box::new(|_: &Part| true),
            _ => unreachable!(
                "The combination of {:?} and {:?} should be impossible!",
                self.attr, self.cmp
            ),
        };
        if inner.as_ref()(part) {
            let outcome = match &self.outcome {
                Decision::Accept => Decision::Accept,
                Decision::Reject => Decision::Reject,
                Decision::OtherWorkflow(s) => Decision::OtherWorkflow(s.to_owned()),
            };
            Some(outcome)
        } else {
            None
        }
    }
}

impl FromStr for Rule {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match &s.chars().collect::<Vec<char>>()[..] {
            [attr @ ('x' | 'm' | 'a' | 's'), cmp @ ('>' | '<'), rest @ ..] => {
                let attr = Attr::from_str(attr.to_string().as_str())?;
                let cmp = Compare::from_str(cmp.to_string().as_str())?;
                let rest = String::from_iter(rest);
                let [digits, outcome] = rest.split(":").collect::<Vec<&str>>()[..] else {
                    bail!("Don't know how to create a Rule from {}", s)
                };
                let value = u32::from_str(digits)?;
                let outcome = Decision::from_str(outcome)?;
                Ok(Rule::new(attr, cmp, value, outcome))
            }
            chars @ [..] => {
                let outcome = Decision::from_str(String::from_iter(chars).as_str())?;
                Ok(Rule::noop(outcome))
            }
        }
    }
}

struct Workflow {
    name: String,
    rules: Vec<Rule>,
}

impl FromStr for Workflow {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let s = s.trim();
        let s = &s[..(s.len() - 1)];
        let [name, rule_strings] = s.split("{").collect::<Vec<&str>>()[..] else {
            bail!("Unexpected number of braces in {}", s)
        };
        let rules = rule_strings
            .split(",")
            .map(|rs| Rule::from_str(rs))
            .collect::<Result<Vec<Rule>>>()?;
        Ok(Workflow {
            name: name.to_string(),
            rules,
        })
    }
}

impl Workflow {
    fn process(&self, part: Part) -> Decision {
        for rule in &self.rules {
            if let Some(decision) = rule.process(&part) {
                return decision;
            }
        }
        unreachable!("At least one rule in self.rules should have returned a `Decision` variant!")
    }
}

impl Display for Workflow {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let num_rules = self.rules.len();
        write!(f, "Workflow(\"{}\", <{} rules>)", self.name, num_rules)
    }
}

struct PuzzleInput {
    workflow_map: HashMap<String, Workflow>,
    parts: Vec<Part>,
}

impl FromStr for PuzzleInput {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let string = s.replace("\r\n", "\n");
        let [workflow_strings, part_strings] = string.split("\n\n").collect::<Vec<&str>>()[..]
        else {
            bail!("Unexpectedly found more than one double-linebreak in the puzzle input!")
        };
        let workflows = workflow_strings
            .lines()
            .map(|line| line.parse())
            .collect::<Result<Vec<Workflow>>>()?;
        let mut workflow_map = HashMap::new();
        for workflow in workflows {
            workflow_map.insert(workflow.name.to_owned(), workflow);
        }
        let parts = part_strings
            .lines()
            .map(|line| line.parse())
            .collect::<Result<Vec<Part>>>()?;
        Ok(PuzzleInput {
            workflow_map,
            parts,
        })
    }
}

fn parse_input(filename: &str) -> Result<PuzzleInput> {
    let input_string =
        read_to_string(filename).context(format!("Expected {} to exist as a file!", filename))?;
    Ok(PuzzleInput::from_str(&input_string)?)
}

fn solve(filename: &str) -> u32 {
    let input = parse_input(filename).unwrap();
    let mut answer = 0;
    for part in input.parts {
        let mut outcome = Decision::OtherWorkflow("in".to_string());
        loop {
            match outcome {
                Decision::Accept => {
                    answer += part.score();
                    break;
                }
                Decision::Reject => break,
                Decision::OtherWorkflow(ref s) => {
                    let workflow = input.workflow_map.get(s).unwrap();
                    outcome = workflow.process(part)
                },
            }
        }
    }
    answer
}

fn main() {
    println!("{}", solve("input.txt"));
}
