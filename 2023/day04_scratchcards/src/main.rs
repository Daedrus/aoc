use log::{debug, info};
use nom::{
    bytes::complete::tag,
    character::complete::{self, multispace1},
    error::Error,
    multi::separated_list1,
    IResult, Parser,
};
use std::{
    collections::HashSet,
    fs::File,
    io::{self, BufRead, BufReader, Seek},
};

#[derive(Debug)]
struct Scratchcard {
    index: usize,
    numbers_you_have: HashSet<u32>,
    winning_numbers: HashSet<u32>,
}

impl From<&str> for Scratchcard {
    fn from(input: &str) -> Self {
        type InputLine<'a> = (
            &'a str,
            &'a str,
            u32,
            &'a str,
            &'a str,
            Vec<u32>,
            &'a str,
            &'a str,
            &'a str,
            Vec<u32>,
        );
        fn parse_line(input: &str) -> IResult<&str, InputLine<'_>, Error<&str>> {
            (
                tag("Card"),
                multispace1,
                complete::u32,
                tag(":"),
                multispace1,
                separated_list1(multispace1, complete::u32),
                multispace1,
                tag("|"),
                multispace1,
                separated_list1(multispace1, complete::u32),
            )
                .parse(input)
        }

        let (_, (_, _, index, _, _, numbers_you_have, _, _, _, winning_numbers)) =
            parse_line(input).unwrap();

        Scratchcard {
            // Have the index start from 0 to simplify some calculations below
            index: (index - 1).try_into().unwrap(),
            numbers_you_have: numbers_you_have.into_iter().collect(),
            winning_numbers: winning_numbers.into_iter().collect(),
        }
    }
}

fn parse_input(input: &mut impl BufRead) -> Vec<Scratchcard> {
    input
        .lines()
        .map(|line| line.unwrap().as_str().into())
        .collect()
}

fn part1(input: &mut impl BufRead) -> String {
    let scratchcards = parse_input(input);

    debug!("{:?}", scratchcards);

    scratchcards
        .iter()
        .map(|scratchcard| {
            let matching_numbers: u32 = scratchcard
                .numbers_you_have
                .intersection(&scratchcard.winning_numbers)
                .count()
                .try_into()
                .unwrap();

            if matching_numbers > 0 {
                2u32.pow(matching_numbers - 1)
            } else {
                0
            }
        })
        .sum::<u32>()
        .to_string()
}

fn part2(input: &mut impl BufRead) -> String {
    let scratchcards = parse_input(input);

    // We initially start with one of each scratchcard (the "original")
    let mut scratchcard_instances = vec![1; scratchcards.len()];

    scratchcards.iter().for_each(|scratchcard| {
        let matching_numbers = scratchcard
            .numbers_you_have
            .intersection(&scratchcard.winning_numbers)
            .count();
        let current_scratchard_instances = scratchcard_instances[scratchcard.index];

        debug!(
            "Card {} has {} matching numbers",
            scratchcard.index + 1,
            matching_numbers
        );

        for instances in &mut scratchcard_instances
            [scratchcard.index + 1..scratchcard.index + 1 + matching_numbers]
            .iter_mut()
        {
            *instances += current_scratchard_instances;
        }

        debug!("{:?}", scratchcard_instances);
    });

    scratchcard_instances.iter().sum::<usize>().to_string()
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
    fn check_answers() {
        init();

        let f = File::open("input").unwrap();
        let mut reader = BufReader::new(f);

        assert_eq!(part1(&mut reader), "33950");
        reader.rewind().unwrap();
        assert_eq!(part2(&mut reader), "14814534");
    }

    #[test]
    fn part1_tests() {
        init();

        let f = File::open("input.example").unwrap();
        let mut reader = BufReader::new(f);

        assert_eq!(part1(&mut reader), "13");
    }

    #[test]
    fn part2_tests() {
        init();

        let f = File::open("input.example").unwrap();
        let mut reader = BufReader::new(f);

        assert_eq!(part2(&mut reader), "30");
    }
}
