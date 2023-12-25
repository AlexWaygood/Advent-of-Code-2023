use std::{fmt::Display, fs::read_to_string, ops::Not, str::FromStr};

use anyhow::{anyhow, bail, Context, Ok, Result};
use cached::proc_macro::cached;
use itertools::Itertools;
use regex;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum Condition {
    DAMAGED,
    UNKNOWN,
    OPERATIONAL,
}

impl Condition {
    fn is_operational(&self) -> bool {
        self == &Condition::OPERATIONAL
    }
}

impl Display for Condition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string = match self {
            Condition::DAMAGED => "D",
            Condition::OPERATIONAL => "O",
            Condition::UNKNOWN => "U",
        };
        write!(f, "{}", string)
    }
}

impl FromStr for Condition {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "#" => Ok(Condition::DAMAGED),
            "?" => Ok(Condition::UNKNOWN),
            "." => Ok(Condition::OPERATIONAL),
            _ => Err(anyhow!("Can't construct a condition from {}", s)),
        }
    }
}

#[cached]
fn num_possible_fits(contiguous_broken: Vec<u32>, conditions: Vec<Condition>) -> usize {
    if conditions.len() < contiguous_broken.len() {
        return 0;
    }

    if conditions[0] == Condition::OPERATIONAL {
        return num_possible_fits(contiguous_broken, conditions[1..].to_vec());
    }

    let grouped_by_operational: Vec<(bool, usize)> = conditions
        .iter()
        .group_by(|c| c.is_operational())
        .into_iter()
        .map(|(operational, group_iter)| (operational, group_iter.map(|_| 1).sum()))
        .collect();

    if (contiguous_broken.iter().sum::<u32>() as usize)
        > grouped_by_operational
            .iter()
            .filter(|(operational, _)| operational.not())
            .map(|(_, group_length)| group_length)
            .sum()
    {
        return 0;
    }

    let grouped_by_condition: Vec<(&Condition, u32)> = conditions
        .iter()
        .group_by(|c| c.to_owned())
        .into_iter()
        .map(|(condition, group_iter)| (condition, group_iter.map(|_| 1).sum()))
        .collect();

    let first_contiguous = contiguous_broken[0] as usize;

    if grouped_by_operational[0].1 < first_contiguous {
        let first_operational_index = (grouped_by_operational[0].1 + 1) as usize;
        if conditions[..first_operational_index]
            .iter()
            .any(|c| c == &Condition::DAMAGED)
        {
            return 0;
        }
        return num_possible_fits(
            contiguous_broken,
            conditions[first_operational_index..].to_vec(),
        );
    }

    if grouped_by_operational[grouped_by_operational.len() - 1].1
        < (contiguous_broken[contiguous_broken.len() - 1] as usize)
    {
        let last_operational_index = conditions.len()
            - (grouped_by_operational[grouped_by_operational.len() - 1].1 as usize)
            - 1;
        if conditions[last_operational_index..]
            .iter()
            .any(|c| c == &Condition::DAMAGED)
        {
            return 0;
        }
        return num_possible_fits(
            contiguous_broken,
            conditions[..last_operational_index].to_vec(),
        );
    }

    let mut answer = 0;

    if contiguous_broken.len() == 1 {
        if grouped_by_condition
            .iter()
            .any(|(c, _)| c == &&Condition::DAMAGED)
        {
            for i in 0..conditions.len() {
                if i != 0 && conditions[i - 1] == Condition::DAMAGED {
                    break;
                }

                if let Some(slice) = conditions.get((i + first_contiguous)..) {
                    if slice.iter().any(|c| c == &Condition::DAMAGED) {
                        continue;
                    }
                }

                match conditions.get(i..(i + first_contiguous)) {
                    Some(to_test) => {
                        if to_test.len() < first_contiguous {
                            break;
                        }
                        if to_test.iter().any(|c| c == &Condition::OPERATIONAL) {
                            continue;
                        }
                        if to_test.iter().all(|c| c != &Condition::DAMAGED) {
                            continue;
                        }
                    }
                    None => break,
                }

                answer += 1
            }
        } else {
            for (condition, group_length) in grouped_by_condition {
                let group_length_usize = group_length as usize;
                if condition == &Condition::UNKNOWN && group_length_usize >= first_contiguous {
                    answer += (group_length_usize - first_contiguous) + 1
                }
            }
        }
    } else {
        let range_to_test = (grouped_by_operational[0].1 as usize) - first_contiguous + 1;
        for i in 0..range_to_test {
            if i != 0 && conditions[i - 1] == Condition::DAMAGED {
                break;
            }
            if let Some(Condition::DAMAGED) = conditions.get(i + first_contiguous) {
                continue;
            }
            if let Some(slice) = conditions.get((i + first_contiguous + 1)..) {
                answer += num_possible_fits(contiguous_broken[1..].to_vec(), slice.to_vec())
            }
        }

        if conditions[..range_to_test]
            .iter()
            .all(|c| c == &Condition::UNKNOWN)
        {
            answer += num_possible_fits(contiguous_broken, conditions[range_to_test..].to_vec())
        }
    }
    answer
}

fn find_conditions(string: &str) -> Result<Vec<Condition>> {
    let re = regex::Regex::new(r"\.+").unwrap();
    let modded_string = re.replace_all(string, ".");
    let mut result = vec![];
    for c in modded_string.trim_matches('.').chars() {
        result.push(c.to_string().as_str().parse()?)
    }
    Ok(result)
}

#[derive(Clone)]
struct Row {
    conditions: Vec<Condition>,
    contiguous_broken_groups: Vec<u32>,
}

impl Row {
    fn num_possible_arrangements(self) -> usize {
        num_possible_fits(self.contiguous_broken_groups, self.conditions)
    }
}

const REPEATS: usize = 5;

impl FromStr for Row {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let (left, right) = match s.split(" ").collect::<Vec<&str>>()[..] {
            [left, right] => (left, right),
            _ => bail!("Couldn't parse {} into a row", s),
        };
        let conditions = find_conditions([&left].iter().cycle().take(REPEATS).join("?").as_str())?;
        let mut contiguous_broken_groups = vec![];
        for val in [&right].iter().cycle().take(REPEATS).join(",").split(",") {
            contiguous_broken_groups.push(val.parse()?)
        }
        Ok(Row {
            conditions,
            contiguous_broken_groups,
        })
    }
}

fn parse_input(filename: &str) -> Result<Vec<Row>> {
    let input_file =
        read_to_string(filename).context(format!("Expected {} to exist!", filename))?;
    input_file.lines().map(|s| s.parse()).collect()
}

fn solve(filename: &str) -> usize {
    let mut answer = 0;
    for row in parse_input(filename).unwrap() {
        answer += row.num_possible_arrangements()
    }
    answer
}

fn main() {
    println!("{}", solve("input.txt"))
}
