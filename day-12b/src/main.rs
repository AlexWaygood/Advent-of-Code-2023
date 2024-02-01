use std::collections::HashSet;
use std::fmt::Display;
use std::fs::read_to_string;
use std::iter::repeat;
use std::str::FromStr;

use anyhow::{bail, Context, Ok, Result};
use cached::proc_macro::cached;
use itertools::Itertools;
use strum_macros::EnumIs;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug, EnumIs)]
enum Condition {
    Damaged,
    Unknown,
    Operational,
}

impl Display for Condition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Condition::Damaged => 'D',
            Condition::Operational => 'O',
            Condition::Unknown => 'U',
        };
        write!(f, "{c}")
    }
}

impl TryFrom<&char> for Condition {
    type Error = anyhow::Error;

    fn try_from(s: &char) -> Result<Self> {
        match s {
            '#' => Ok(Condition::Damaged),
            '?' => Ok(Condition::Unknown),
            '.' => Ok(Condition::Operational),
            _ => bail!("Can't construct a condition from {s}"),
        }
    }
}

#[cached]
fn num_possible_fits(contiguous_broken: Vec<u32>, conditions: Vec<Condition>) -> usize {
    if conditions.len() < contiguous_broken.len() {
        return 0;
    }

    if conditions[0].is_operational() {
        return num_possible_fits(contiguous_broken, conditions[1..].to_vec());
    }

    let grouped_by_operational: Vec<(bool, usize)> = conditions
        .iter()
        .group_by(|c| c.is_operational())
        .into_iter()
        .map(|(operational, group_iter)| (operational, group_iter.count()))
        .collect();
    debug_assert!(!grouped_by_operational[0].0);
    debug_assert!(!grouped_by_operational[grouped_by_operational.len() - 1].0);

    if (contiguous_broken.iter().sum::<u32>() as usize)
        > grouped_by_operational
            .iter()
            .filter(|(operational, _)| !operational)
            .map(|(_, group_length)| group_length)
            .sum()
    {
        return 0;
    }

    let grouped_by_condition: Vec<(&Condition, usize)> = conditions
        .iter()
        .group_by(|c| c.to_owned())
        .into_iter()
        .map(|(condition, group_iter)| (condition, group_iter.count()))
        .collect();
    debug_assert_ne!(grouped_by_condition[0].0, &Condition::Operational);
    debug_assert_ne!(
        grouped_by_condition[grouped_by_condition.len() - 1].0,
        &Condition::Operational
    );

    let first_contiguous = contiguous_broken[0] as usize;

    if grouped_by_operational[0].1 < first_contiguous {
        let first_operational_index = grouped_by_operational[0].1 + 1;
        if conditions[..first_operational_index].contains(&Condition::Damaged) {
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
        let last_operational_index =
            conditions.len() - grouped_by_operational[grouped_by_operational.len() - 1].1 - 1;
        if conditions[last_operational_index..].contains(&Condition::Damaged) {
            return 0;
        }
        return num_possible_fits(
            contiguous_broken,
            conditions[..last_operational_index].to_vec(),
        );
    }

    let mut answer = 0;

    if contiguous_broken.len() == 1 {
        if grouped_by_condition.iter().any(|(c, _)| c.is_damaged()) {
            for i in 0..conditions.len() {
                if i != 0 && conditions[i - 1].is_damaged() {
                    break;
                }

                if let Some(slice) = conditions.get((i + first_contiguous)..) {
                    if slice.contains(&Condition::Damaged) {
                        continue;
                    }
                }

                match conditions.get(i..(i + first_contiguous)) {
                    Some(slice) => {
                        if slice.len() < first_contiguous {
                            break;
                        }
                        let to_test: HashSet<&Condition> = HashSet::from_iter(slice);
                        if to_test.contains(&Condition::Operational) {
                            continue;
                        }
                        if !to_test.contains(&Condition::Damaged) {
                            continue;
                        }
                    }
                    None => break,
                }

                answer += 1
            }
        } else {
            for (condition, group_length) in grouped_by_condition {
                if condition.is_unknown() && group_length >= first_contiguous {
                    answer += (group_length - first_contiguous) + 1
                }
            }
        }
    } else {
        let range_to_test = grouped_by_operational[0].1 - first_contiguous + 1;
        for i in 0..range_to_test {
            if i != 0 && conditions[i - 1].is_damaged() {
                break;
            }
            if let Some(Condition::Damaged) = conditions.get(i + first_contiguous) {
                continue;
            }
            if let Some(slice) = conditions.get((i + first_contiguous + 1)..) {
                answer += num_possible_fits(contiguous_broken[1..].to_vec(), slice.to_vec())
            }
        }

        if conditions[..range_to_test].iter().all(|c| c.is_unknown()) {
            answer += num_possible_fits(contiguous_broken, conditions[range_to_test..].to_vec())
        }
    }
    answer
}

fn find_conditions(string: &str) -> Result<Vec<Condition>> {
    let re = regex::Regex::new(r"\.+").unwrap();
    let modded_string = re.replace_all(string, ".");
    modded_string
        .trim_matches('.')
        .chars()
        .map(|c| Condition::try_from(&c))
        .collect()
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
        let (left, right) = match s.split(' ').collect_vec()[..] {
            [left, right] => (left, right),
            _ => bail!("Couldn't parse {s} into a row"),
        };
        let conditions = find_conditions(repeat(left).take(REPEATS).join("?").as_str())?;
        let contiguous_broken_groups = repeat(right)
            .take(REPEATS)
            .join(",")
            .split(',')
            .map(|val| val.parse())
            .collect::<Result<_, _>>()?;
        Ok(Row {
            conditions,
            contiguous_broken_groups,
        })
    }
}

fn read_input(filename: &str) -> Result<String> {
    read_to_string(filename).with_context(|| format!("Expected {filename} to exist!"))
}

fn solve(filename: &str) -> usize {
    read_input(filename)
        .unwrap()
        .lines()
        .map(|line| Row::from_str(line).unwrap().num_possible_arrangements())
        .sum()
}

fn main() {
    println!("{}", solve("input.txt"))
}
