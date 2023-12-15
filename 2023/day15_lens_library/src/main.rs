use log::{debug, info};
use regex::Regex;
use std::{
    fs::File,
    io::{self, BufRead, BufReader, Seek},
};

#[derive(Debug)]
enum Operation {
    Dash,
    Equals(usize),
}

#[derive(Debug)]
struct Step {
    label: String,
    operation: Operation,
}

#[derive(Clone, Debug)]
struct Slot {
    label: String,
    focal_length: usize,
}

impl TryFrom<&str> for Step {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let step_equals_pattern = Regex::new(r"([a-z]+)=([0-9])").unwrap();
        let step_dash_pattern = Regex::new(r"([a-z]+)-").unwrap();

        if let Some(result) = step_equals_pattern.captures(value) {
            Ok(Step {
                label: result.get(1).unwrap().as_str().to_string(),
                operation: Operation::Equals(
                    result.get(2).unwrap().as_str().parse::<usize>().unwrap(),
                ),
            })
        } else if let Some(result) = step_dash_pattern.captures(value) {
            Ok(Step {
                label: result.get(1).unwrap().as_str().to_string(),
                operation: Operation::Dash,
            })
        } else {
            Err("Step pattern not recognized")
        }
    }
}

impl ToString for Operation {
    fn to_string(&self) -> String {
        match self {
            Operation::Dash => "-".to_string(),
            Operation::Equals(focal_length) => format!("={}", focal_length),
        }
    }
}

impl ToString for Step {
    fn to_string(&self) -> String {
        format!("{}{}", &self.label, &self.operation.to_string())
    }
}

fn get_hash(input: &str) -> usize {
    input
        .chars()
        .fold(0, |acc, c| ((c as usize + acc) * 17) % 256)
}

fn parse_input(input: &mut impl BufRead) -> Vec<Step> {
    let mut input_str = String::default();
    let _ = input.read_to_string(&mut input_str);

    input_str
        .trim()
        .split(',')
        .map(|s| s.try_into().unwrap())
        .collect::<Vec<Step>>()
}

fn part1(input: &mut impl BufRead) -> String {
    let steps = parse_input(input);

    debug!("{:?}", steps);

    steps
        .iter()
        .map(|step| get_hash(&step.to_string()))
        .sum::<usize>()
        .to_string()
}

fn part2(input: &mut impl BufRead) -> String {
    let steps = parse_input(input);
    let mut boxes: Vec<Vec<Slot>> = vec![Vec::new(); 256];

    steps.iter().for_each(|step| {
        let box_number = get_hash(&step.label);
        if let Some((index, _)) = boxes[box_number]
            .iter()
            .enumerate()
            .find(|(_, slot)| slot.label == step.label)
        {
            match step.operation {
                Operation::Dash => {
                    boxes[box_number].remove(index);
                }
                Operation::Equals(focal_length) => {
                    boxes[box_number][index].focal_length = focal_length;
                }
            }
        } else {
            match step.operation {
                Operation::Dash => {}
                Operation::Equals(focal_length) => boxes[box_number].push(Slot {
                    label: step.label.clone(),
                    focal_length,
                }),
            }
        }
    });

    boxes
        .iter()
        .enumerate()
        .map(|(box_index, box_contents)| {
            box_contents
                .iter()
                .enumerate()
                .map(|(slot_index, slot)| (box_index + 1) * (slot_index + 1) * slot.focal_length)
                .sum::<usize>()
        })
        .sum::<usize>()
        .to_string()
}

fn main() -> io::Result<()> {
    env_logger::init();

    let f = File::open("input")?;
    let mut reader = BufReader::new(f);

    info!("Part 1 answer: {}", part1(&mut reader));

    reader.rewind().unwrap();

    info!("Part 2 answer: {}", part2(&mut reader));

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::*;
    use std::io::Cursor;

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn check_answers() {
        init();

        let f = File::open("input").unwrap();
        let mut reader = BufReader::new(f);

        assert_eq!(part1(&mut reader), "516469");
        reader.rewind().unwrap();
        assert_eq!(part2(&mut reader), "221627");
    }

    #[test]
    fn part1_tests() {
        init();

        assert_eq!(
            part1(&mut Cursor::new(
                "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7"
            )),
            "1320"
        );

        assert_eq!(get_hash("HASH"), 52);
    }

    #[test]
    fn part2_tests() {
        init();

        assert_eq!(
            part2(&mut Cursor::new(
                "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7"
            )),
            "145"
        );
    }
}
