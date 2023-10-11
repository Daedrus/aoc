use itertools::Itertools;
use log::info;
use std::{
    fs::File,
    io::{self, BufRead, BufReader, Seek},
};

// Given an amount of liters and a bunch of containers, return all of
// the different combinations of containers which fit exactly the
// amount of liters
fn fill_containers(liters: u32, containers: &[u32]) -> Vec<Vec<u32>> {
    // State space generation/search, in each node we have the following branches:
    // - fill the current container
    //   - if no liters left, solution is found
    //   - if liters left, proceed to fill the remaining containers
    // - ignore the current container and proceed to fill the rest
    //
    // current_choices is used to remember the containers we filled so far
    fn fill_containers_rec(
        liters: u32,
        sorted_containers: &[u32],
        index: usize,
        current_choices: &mut Vec<u32>,
        solutions: &mut Vec<Vec<u32>>,
    ) {
        if sorted_containers.len() != index {
            // Fill the current container
            match liters.cmp(&sorted_containers[index]) {
                std::cmp::Ordering::Equal => {
                    // No liters left after filling the container, we found a solution
                    let mut solution = current_choices.clone();
                    solution.push(sorted_containers[index]);
                    solutions.push(solution);
                }
                std::cmp::Ordering::Greater => {
                    // Proceed to fill the other containers using the remaining liters
                    let mut new_choices = current_choices.clone();
                    new_choices.push(sorted_containers[index]);
                    fill_containers_rec(
                        liters - sorted_containers[index],
                        sorted_containers,
                        index + 1,
                        &mut new_choices,
                        solutions,
                    );
                }
                std::cmp::Ordering::Less => {
                    // Not enough liters to fill the container, don't do anything
                }
            }

            // Try and see if there are other solutions obtained by skipping the current container
            //
            // Rely on the fact that we sort the containers array so avoid generating/searching
            // the state space if the current liters amount doesn't fill the smallest remaining
            // container
            if liters >= *sorted_containers.iter().last().unwrap() {
                fill_containers_rec(
                    liters,
                    sorted_containers,
                    index + 1,
                    current_choices,
                    solutions,
                );
            }
        }
    }

    // Create a new slice with the containers sorted in descending order
    let mut sorted_containers = vec![0; containers.len()];
    sorted_containers.clone_from_slice(containers);
    sorted_containers.sort();
    sorted_containers.reverse();

    let mut solutions: Vec<Vec<u32>> = Vec::new();

    fill_containers_rec(
        liters,
        &sorted_containers,
        0,
        &mut Vec::new(),
        &mut solutions,
    );

    solutions
}

fn part1(input: &mut impl BufRead, liters: u32) -> String {
    let containers: Vec<u32> = input
        .lines()
        .map(|line| line.unwrap().parse().unwrap())
        .collect();

    fill_containers(liters, &containers).len().to_string()
}

fn part2(input: &mut impl BufRead, liters: u32) -> String {
    let containers: Vec<u32> = input
        .lines()
        .map(|line| line.unwrap().parse().unwrap())
        .collect();

    fill_containers(liters, &containers)
        .iter()
        .min_set_by_key(|solution| solution.len())
        .len()
        .to_string()
}

fn main() -> io::Result<()> {
    env_logger::init();

    let f = File::open("input")?;
    let mut reader = BufReader::new(f);

    info!("Part 1 answer: {}", part1(&mut reader, 150));

    reader.rewind().unwrap();

    info!("Part 2 answer: {}", part2(&mut reader, 150));

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::*;

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn part1_tests() {
        init();

        let f = File::open("input.example").unwrap();
        let mut reader = BufReader::new(f);

        assert_eq!(part1(&mut reader, 25), "4");
    }

    #[test]
    fn part2_tests() {
        init();

        let f = File::open("input.example").unwrap();
        let mut reader = BufReader::new(f);

        assert_eq!(part2(&mut reader, 25), "3");
    }

    #[test]
    fn check_answers() {
        init();

        let f = File::open("input").unwrap();
        let mut reader = BufReader::new(f);

        assert_eq!(part1(&mut reader, 150), "654");
        reader.rewind().unwrap();
        assert_eq!(part2(&mut reader, 150), "57");
    }
}
