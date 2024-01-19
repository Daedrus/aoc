use ndarray::Array2;
use nom::{bytes::complete::tag, character::complete::alphanumeric1};
use nom::sequence::delimited;
use log::{info, debug};
use nom::{
    character::complete::{self, multispace1, alpha1},
    sequence::tuple,
};
use std::usize;
use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
struct DigInstruction {
    direction: Direction,
    meters: u32,
}

impl From<&str> for DigInstruction {
    fn from(input: &str) -> Self {
        let (_, (direction, _, meters, _, _)) =
            tuple::<_, _, nom::error::Error<_>, _>((
                alpha1,
                multispace1,
                complete::u32,
                multispace1,
                delimited(tag("(#"), alphanumeric1, tag(")")),
            ))(input)
            .unwrap();

        DigInstruction {
            direction: match direction {
                "U" => Direction::Up,
                "D" => Direction::Down,
                "L" => Direction::Left,
                "R" => Direction::Right,
                _ => unreachable!()
            },
            meters,
        }
    }
}

fn parse_input(input: &mut impl BufRead) -> Vec<DigInstruction> {
    input
        .lines()
        .map(|line| {
            let line = line.unwrap();

            line.as_str().into()
        })
        .collect()
}

fn fill((current_position_x, current_position_y): (usize, usize), dig_site: &mut Box<Array2<char>>) {
    let mut to_visit: Vec<(usize, usize)> = Vec::new();

    to_visit.push((current_position_x, current_position_y));

    while let Some((node_x, node_y)) = to_visit.pop() {
        dig_site[(node_x, node_y)] = '*';

        if node_x > 0 && dig_site[(node_x - 1, node_y)] == '.' {
            to_visit.push((node_x - 1, node_y));
        }
        if node_x < dig_site.dim().0 - 1 && dig_site[(node_x + 1, node_y)] == '.' {
            to_visit.push((node_x + 1, node_y));
        }
        if node_y > 0 && dig_site[(node_x, node_y - 1)] == '.' {
            to_visit.push((node_x, node_y - 1));
        }
        if node_y < dig_site.dim().1 - 1 && dig_site[(node_x, node_y + 1)] == '.' {
            to_visit.push((node_x, node_y + 1));
        }
    }
}

fn part1(input: &mut impl BufRead) -> String {
    let dig_instructions = parse_input(input);
    let mut dig_site: Box<ndarray::prelude::ArrayBase<ndarray::OwnedRepr<char>, ndarray::prelude::Dim<[usize; 2]>>> = Box::new(Array2::from_elem((1000,1000), '.'));

    debug!("{:?}", dig_instructions);

    let mut max_x = usize::MIN;
    let mut max_y = usize::MIN;

    let (mut current_position_x, mut current_position_y) = (300, 200);
    dig_site[(current_position_x, current_position_y)] = '#';

    dig_instructions
        .iter()
        .for_each(|instruction| {
            match instruction.direction {
                Direction::Up => {
                    for i in current_position_x - instruction.meters as usize..=current_position_x {
                        dig_site[(i, current_position_y)] = '#';
                    }
                    current_position_x -= instruction.meters as usize;
                    max_x = max_x.max(current_position_x);
                },
                Direction::Down => {
                    for i in current_position_x..=current_position_x + instruction.meters as usize {
                        dig_site[(i, current_position_y)] = '#';
                    }
                    current_position_x += instruction.meters as usize;
                    max_x = max_x.max(current_position_x);
                },
                Direction::Left => {
                    for j in current_position_y - instruction.meters as usize..=current_position_y {
                        dig_site[(current_position_x, j)] = '#';
                    }
                    current_position_y -= instruction.meters as usize;
                    max_y = max_y.max(current_position_y);
                },
                Direction::Right => {
                    for j in current_position_y..=current_position_y + instruction.meters as usize {
                        dig_site[(current_position_x, j)] = '#';
                    }
                    current_position_y += instruction.meters as usize;
                    max_y = max_y.max(current_position_y);
                },
            }
        });

    fill((0, 0), &mut dig_site);


    debug!("{} {}", max_x, max_y);
    debug!("{:?}", dig_site);

    dig_site.iter().filter(|elem| **elem == '#' || **elem == '.').count().to_string()
}

fn main() -> io::Result<()> {
    env_logger::init();

    let f = File::open("input")?;
    let mut reader = BufReader::new(f);

    info!("Part 1 answer: {}", part1(&mut reader));

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::*;

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn check_answers() {
        init();

        let f = File::open("input").unwrap();
        let mut reader = BufReader::new(f);

        assert_eq!(part1(&mut reader), "46359");
    }

    #[test]
    fn part1_tests() {
        init();

        let f = File::open("input.example").unwrap();
        let mut reader = BufReader::new(f);

        assert_eq!(part1(&mut reader), "62");
    }
}
