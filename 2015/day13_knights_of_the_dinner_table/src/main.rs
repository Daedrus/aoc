use itertools::Itertools;
use log::{debug, info};
use nom::{branch::alt, bytes::complete::tag, character::complete, sequence::tuple};
use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{self, BufRead, BufReader, Seek},
};

fn parse_input(input: &mut impl BufRead) -> (HashSet<String>, HashMap<(String, String), i32>) {
    let mut people: HashSet<String> = HashSet::new();
    let mut happiness_changes: HashMap<(String, String), i32> = HashMap::new();

    input.lines().for_each(|line| {
        let line = line.unwrap();

        let (_, (person1, gain_or_lose, happiness_amount, _, person2, _)) =
            tuple::<_, _, nom::error::Error<_>, _>((
                complete::alpha1,
                alt((tag(" would gain "), tag(" would lose "))),
                complete::i32,
                tag(" happiness units by sitting next to "),
                complete::alpha1,
                nom::character::complete::char('.'),
            ))(line.as_str())
            .unwrap();

        people.insert(person1.to_string());
        people.insert(person2.to_string());
        happiness_changes.insert(
            (person1.to_string(), person2.to_string()),
            match gain_or_lose {
                " would gain " => happiness_amount,
                " would lose " => -happiness_amount,
                _ => unreachable!(),
            },
        );
    });

    debug!("{:?}", people);
    debug!("{:?}", happiness_changes);

    (people, happiness_changes)
}

fn calculate_happiness(
    people: &HashSet<String>,
    happiness_changes: &HashMap<(String, String), i32>,
) -> String {
    people
        .iter()
        .permutations(people.len())
        .map(|permutation| {
            permutation
                .iter()
                .circular_tuple_windows()
                .fold(0, |happiness, (p1, p2)| {
                    happiness
                        + happiness_changes
                            .get(&(p1.to_string(), p2.to_string()))
                            .unwrap()
                        + happiness_changes
                            .get(&(p2.to_string(), p1.to_string()))
                            .unwrap()
                })
        })
        .max()
        .unwrap()
        .to_string()
}

fn part1(input: &mut impl BufRead) -> String {
    let (people, happiness_changes) = parse_input(input);

    calculate_happiness(&people, &happiness_changes)
}

fn part2(input: &mut impl BufRead) -> String {
    let (mut people, mut happiness_changes) = parse_input(input);

    people.iter().for_each(|person| {
        happiness_changes.insert((person.to_string(), "Me".to_string()), 0);
        happiness_changes.insert(("Me".to_string(), person.to_string()), 0);
    });

    people.insert("Me".to_string());

    calculate_happiness(&people, &happiness_changes)
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

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn part1_tests() {
        init();

        let f = File::open("input.example").unwrap();
        let mut reader = BufReader::new(f);

        assert_eq!(part1(&mut reader), "330");
    }

    #[test]
    fn part2_tests() {
        init();

        let f = File::open("input.example").unwrap();
        let mut reader = BufReader::new(f);

        assert_eq!(part2(&mut reader), "286");
    }

    #[test]
    fn check_answers() {
        init();

        let f = File::open("input").unwrap();
        let mut reader = BufReader::new(f);

        assert_eq!(part1(&mut reader), "664");
        reader.rewind().unwrap();
        assert_eq!(part2(&mut reader), "640");
    }
}
