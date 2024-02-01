use std::collections::HashMap;
use std::{fs::read_to_string, str::FromStr};

use anyhow::{bail, Ok, Result};
use cached::proc_macro::cached;

type Label = String;

#[cached]
fn box_number_from_label(label: Label) -> u8 {
    debug_assert!(label.is_ascii());
    let mut answer: u32 = 0;
    for byte in label.bytes() {
        answer += byte as u32;
        answer *= 17;
        answer %= 256
    }
    answer.try_into().expect("Expected result to be <256!")
}

#[derive(PartialEq, Eq, Debug)]
enum Operation {
    RemoveLens(Label),
    InsertLens(Label, u8),
}

impl Operation {
    fn box_number(&self) -> u8 {
        let label = match self {
            Operation::RemoveLens(label) => label,
            Operation::InsertLens(label, _) => label,
        };
        box_number_from_label(label.to_string())
    }
}

impl FromStr for Operation {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        match s.chars().collect::<Vec<char>>()[..] {
            [.., '-'] => Ok(Operation::RemoveLens(s[..s.len() - 1].to_string())),
            [.., '=', focal_length @ '1'..='9'] => Ok(Operation::InsertLens(
                s[..s.len() - 2].to_string(),
                focal_length.to_string().as_str().parse::<u8>()?,
            )),
            _ => bail!("Can't create an `Operation` from {s}"),
        }
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
struct Lens {
    focal_length: u8,
}

#[derive(PartialEq, Eq, Debug)]
struct Box {
    index_to_label: Vec<Label>,
    label_to_lens: HashMap<Label, Lens>,
}

impl Box {
    fn new() -> Self {
        Box {
            index_to_label: vec![],
            label_to_lens: HashMap::new(),
        }
    }

    fn apply_operation(&mut self, operation: Operation) {
        match operation {
            Operation::RemoveLens(label) => {
                if self.label_to_lens.remove(&label).is_some() {
                    let index = self
                        .index_to_label
                        .iter()
                        .position(|l| l == &label)
                        .unwrap_or_else(|| panic!(
                            "Expected {label} to be present in `index_to_label`, given it was present in `label_to_lens`!"
                        ));
                    self.index_to_label.remove(index);
                }
            }
            Operation::InsertLens(label, focal_length) => {
                if self
                    .label_to_lens
                    .insert(label.to_owned(), Lens { focal_length })
                    .is_none()
                {
                    self.index_to_label.push(label)
                }
            }
        }
    }

    fn focusing_power(&self, box_number: usize) -> usize {
        self.index_to_label
            .iter()
            .enumerate()
            .map(|(i, label)| {
                (box_number + 1) * (i + 1) * (self.label_to_lens[label].focal_length as usize)
            })
            .sum()
    }

    #[cfg(test)]
    fn lenses_copy(&self) -> Vec<(String, Lens)> {
        self.index_to_label
            .iter()
            .map(|label| (label.to_owned(), self.label_to_lens[label]))
            .collect()
    }

    #[cfg(test)]
    fn is_empty(&self) -> bool {
        self.index_to_label.is_empty()
    }
}

struct BoxArray {
    boxes: [Box; 256],
}

impl BoxArray {
    fn new() -> Self {
        BoxArray {
            boxes: std::array::from_fn(|_| Box::new()),
        }
    }

    fn apply_operation(&mut self, step: Operation) {
        self.boxes[step.box_number() as usize].apply_operation(step)
    }

    fn total_focusing_power(&self) -> usize {
        self.boxes
            .iter()
            .enumerate()
            .map(|(i, b)| b.focusing_power(i))
            .sum()
    }

    #[cfg(test)]
    fn non_empty_boxes(&self) -> Vec<usize> {
        self.boxes
            .iter()
            .enumerate()
            .filter(|(_, b)| !b.is_empty())
            .map(|(i, _)| i)
            .collect()
    }
}

fn parse_input(input: &str) -> Result<Vec<Operation>> {
    input.split(',').map(|s| s.parse()).collect()
}

fn solve(filename: &str) -> usize {
    let input =
        read_to_string(filename).unwrap_or_else(|_| panic!("Expected {filename} to exist!"));
    let steps = parse_input(&input).unwrap();
    let mut box_array = BoxArray::new();
    for step in steps {
        box_array.apply_operation(step)
    }
    box_array.total_focusing_power()
}

fn main() {
    println!("{}", solve("input.txt"));
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::{parse_input, BoxArray, Lens, Operation};

    #[test]
    fn test_box_array_initialisation() {
        let mut box_array = BoxArray::new();
        assert_eq!(box_array.boxes.len(), 256);
        assert_eq!(box_array.boxes[0], box_array.boxes[1]);
        box_array.boxes[0]
            .label_to_lens
            .insert("foo".to_string(), Lens { focal_length: 42 });
        assert_ne!(box_array.boxes[0], box_array.boxes[1])
    }

    #[test]
    fn test_input_parsing() {
        let example_input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        let steps = parse_input(example_input).unwrap();
        assert_eq!(steps.len(), 11);
        let (mut inserts, mut removals) = (0, 0);
        for step in &steps {
            match step {
                Operation::InsertLens(_, _) => inserts += 1,
                Operation::RemoveLens(_) => removals += 1,
            }
        }
        assert_eq!(inserts, 8);
        assert_eq!(removals, 3);
        assert_eq!(steps[0], Operation::InsertLens("rn".to_string(), 1));
        assert_eq!(steps[1], Operation::RemoveLens("cm".to_string()));
        assert_eq!(
            steps[steps.len() - 1],
            Operation::InsertLens("ot".to_string(), 7)
        );
    }

    fn operation(input: &str) -> Operation {
        Operation::from_str(input).unwrap()
    }

    fn lens_vec(data: &[(&str, u8)]) -> Vec<(String, Lens)> {
        data.iter()
            .map(|(k, v)| (k.to_string(), Lens { focal_length: *v }))
            .collect()
    }

    #[test]
    fn test_operation_application() {
        let mut box_array = BoxArray::new();
        assert_eq!(box_array.non_empty_boxes(), vec![]);

        box_array.apply_operation(operation("rn=1"));
        assert_eq!(box_array.non_empty_boxes(), [0]);
        assert_eq!(box_array.boxes[0].lenses_copy(), lens_vec(&[("rn", 1)]));

        box_array.apply_operation(operation("cm-"));
        assert_eq!(box_array.non_empty_boxes(), [0]);
        assert_eq!(box_array.boxes[0].lenses_copy(), lens_vec(&[("rn", 1)]));

        box_array.apply_operation(operation("qp=3"));
        assert_eq!(box_array.non_empty_boxes(), [0, 1]);
        assert_eq!(box_array.boxes[0].lenses_copy(), lens_vec(&[("rn", 1)]));
        assert_eq!(box_array.boxes[1].lenses_copy(), lens_vec(&[("qp", 3)]));

        box_array.apply_operation(operation("cm=2"));
        assert_eq!(box_array.non_empty_boxes(), [0, 1]);
        assert_eq!(
            box_array.boxes[0].lenses_copy(),
            lens_vec(&[("rn", 1), ("cm", 2)])
        );
        assert_eq!(box_array.boxes[1].lenses_copy(), lens_vec(&[("qp", 3)]));

        box_array.apply_operation(operation("qp-"));
        assert_eq!(box_array.non_empty_boxes(), [0]);
        assert_eq!(
            box_array.boxes[0].lenses_copy(),
            lens_vec(&[("rn", 1), ("cm", 2)])
        );
        assert_eq!(box_array.boxes[1].lenses_copy(), vec![]);

        box_array.apply_operation(operation("pc=4"));
        assert_eq!(box_array.non_empty_boxes(), [0, 3]);
        assert_eq!(
            box_array.boxes[0].lenses_copy(),
            lens_vec(&[("rn", 1), ("cm", 2)])
        );
        assert_eq!(box_array.boxes[3].lenses_copy(), lens_vec(&[("pc", 4)]));

        box_array.apply_operation(operation("ot=9"));
        assert_eq!(box_array.non_empty_boxes(), [0, 3]);
        assert_eq!(
            box_array.boxes[0].lenses_copy(),
            lens_vec(&[("rn", 1), ("cm", 2)])
        );
        assert_eq!(
            box_array.boxes[3].lenses_copy(),
            lens_vec(&[("pc", 4), ("ot", 9)])
        );

        box_array.apply_operation(operation("ab=5"));
        assert_eq!(box_array.non_empty_boxes(), [0, 3]);
        assert_eq!(
            box_array.boxes[0].lenses_copy(),
            lens_vec(&[("rn", 1), ("cm", 2)])
        );
        assert_eq!(
            box_array.boxes[3].lenses_copy(),
            lens_vec(&[("pc", 4), ("ot", 9), ("ab", 5)])
        );

        box_array.apply_operation(operation("pc-"));
        assert_eq!(box_array.non_empty_boxes(), [0, 3]);
        assert_eq!(
            box_array.boxes[0].lenses_copy(),
            lens_vec(&[("rn", 1), ("cm", 2)])
        );
        assert_eq!(
            box_array.boxes[3].lenses_copy(),
            lens_vec(&[("ot", 9), ("ab", 5)])
        );

        box_array.apply_operation(operation("pc=6"));
        assert_eq!(box_array.non_empty_boxes(), [0, 3]);
        assert_eq!(
            box_array.boxes[0].lenses_copy(),
            lens_vec(&[("rn", 1), ("cm", 2)])
        );
        assert_eq!(
            box_array.boxes[3].lenses_copy(),
            lens_vec(&[("ot", 9), ("ab", 5), ("pc", 6)])
        );

        box_array.apply_operation(operation("ot=7"));
        assert_eq!(box_array.non_empty_boxes(), [0, 3]);
        assert_eq!(
            box_array.boxes[0].lenses_copy(),
            lens_vec(&[("rn", 1), ("cm", 2)])
        );
        assert_eq!(
            box_array.boxes[3].lenses_copy(),
            lens_vec(&[("ot", 7), ("ab", 5), ("pc", 6)])
        );
    }
}
