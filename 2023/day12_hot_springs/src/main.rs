use log::{info, debug};
use std::{
    fs::File,
    io::{self, BufRead, BufReader}, collections::VecDeque,
};

#[derive(Debug)]
struct Row {
    springs: String,
    damaged_pattern: String,
}

fn parse_input(input: &mut impl BufRead) -> Vec<Row> {
    input
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let x: Vec<&str> = line.split(' ').collect();

            Row {
                springs: x[0].to_string(),
                damaged_pattern: x[1].to_string(),
            }
        })
        .collect()
}

impl Row {
    fn count_arrangements(&self) -> usize {
        fn count_arrangements_rec(
            current_arrangement: String,
            index: usize,
            damaged_pattern: &mut VecDeque<usize>,
            need_operational: bool,
            need_damaged: bool)
        -> usize {
            debug!("{}, {}, {:?}, {}", current_arrangement, index, damaged_pattern, need_operational);

            if index == current_arrangement.len() && damaged_pattern.is_empty() {
                1
            } else if index == current_arrangement.len() || (damaged_pattern.is_empty() && current_arrangement[index..].contains('#')) {
                0
            } else if damaged_pattern.is_empty() && !current_arrangement[index..].contains('#') {
                1
            } else {
                let current_spring = current_arrangement.chars().nth(index).unwrap();

                match current_spring {
                    // Operational spring
                    '.' => {
                        if need_damaged {
                            0
                        } else {
                            count_arrangements_rec(current_arrangement, index + 1, &mut damaged_pattern.clone(), false, false)
                        }
                    }
                    // Damaged spring
                    '#' => {
                        if need_operational {
                            0
                        } else if damaged_pattern[0] == 1 {
                            damaged_pattern.pop_front();
                            count_arrangements_rec(current_arrangement, index + 1, &mut damaged_pattern.clone(), true, false)
                        } else {
                            damaged_pattern[0] -= 1;
                            count_arrangements_rec(current_arrangement, index + 1, &mut damaged_pattern.clone(), false, true)
                        }
                    }
                    // Unknown spring
                    '?' => {
                        let mut new_arrangement_with_operational_spring = current_arrangement.clone();
                        new_arrangement_with_operational_spring.replace_range(index..index+1, ".");
                        let mut new_arrangement_with_damaged_spring = current_arrangement.clone();
                        new_arrangement_with_damaged_spring.replace_range(index..index+1, "#");
                        count_arrangements_rec(new_arrangement_with_operational_spring, index, damaged_pattern, need_operational, need_damaged) +
                        count_arrangements_rec(new_arrangement_with_damaged_spring, index, &mut damaged_pattern.clone(), need_operational, need_damaged)
                    }
                    _ => unreachable!()
                }
            }
        }

        count_arrangements_rec(
            self.springs.clone(),
            0,
            &mut self.damaged_pattern.split(',').map(|c| c.parse::<usize>().unwrap()).collect::<VecDeque<usize>>(),
            false,
            false)
    }
}

fn part1(input: &mut impl BufRead) -> String {
    let rows = parse_input(input);

    debug!("{:?}", rows);

    rows.iter().map(|row| row.count_arrangements()).sum::<usize>().to_string()
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

        assert_eq!(part1(&mut reader), "7251");
    }

    #[test]
    fn part1_tests() {
        init();

        let f = File::open("input.example").unwrap();
        let mut reader = BufReader::new(f);

        assert_eq!(part1(&mut reader), "21");
    }
}
