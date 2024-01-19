use log::{info, debug};
use nom::{
    bytes::complete::tag,
    character::complete::{self, newline, alphanumeric1},
    multi::separated_list1,
    sequence::{tuple, terminated},
    IResult,
};
use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

type Seed = u64;

#[derive(Debug)]
struct Range {
    destination_range_start: u64,
    source_range_start: u64,
    range_length: u64,
}

type Map = Vec<Range>;

fn parse_input(input: &mut impl BufRead, seed_line: bool) -> (Vec<Seed>, Vec<Map>) {

    fn range_parser(input: &str) -> IResult<&str, Range> {
        tuple((
            terminated(complete::u64, tag(" ")),
            terminated(complete::u64, tag(" ")),
            complete::u64,
        ))(input).map(|(s, (destination_range_start, source_range_start, range_length))| {
                (
                    s,
                    Range { destination_range_start, source_range_start, range_length }
                )
            })
    }

    fn map_parser(input: &str) -> IResult<&str, Map> {
        tuple((
            terminated(
                tuple((
                    alphanumeric1,
                    tag("-to-"),
                    alphanumeric1,
                    tag(" map"),
                )), tag(":")),
            newline,
            separated_list1(newline, range_parser),
            newline,
        ))(input).map(|(s, (_, _, ranges, _))| {
                (
                    s,
                    ranges
                )
            })
    }

    let mut lines: String = Default::default();
    input.read_to_string(&mut lines).unwrap();

    let (lines, (_, mut seeds, _, _)) = tuple::<_, _, nom::error::Error<_>, _>((
        tag("seeds: "),
        separated_list1(tag(" "), complete::u64),
        newline,
        newline,
    ))(lines.as_str()).unwrap();

    if !seed_line {
        let mut range1: Vec<u64> = (seeds[0]..seeds[0]+seeds[1]).collect();
        let mut range2: Vec<u64> = (seeds[2]..seeds[2]+seeds[3]).collect();

        range1.append(&mut range2);

        seeds = range1;
    }

    let (_, maps) = separated_list1(newline, map_parser)(lines).unwrap();

    (seeds, maps)
}

fn part1(input: &mut impl BufRead) -> String {
    let (seeds, maps) = parse_input(input, true);

    debug!("{:?}", seeds);

    maps.iter().for_each(|map| {
        debug!("{:?}", map);
    });

    seeds
        .iter()
        .map(|seed| {
            let mut location: u64 = *seed;

            maps
                .iter()
                .for_each(|map| {
                    match map
                        .iter()
                        .find(|range| location >= range.source_range_start &&
                                      location < range.source_range_start + range.range_length)
                        {
                            Some(range) => {
                                location = range.destination_range_start + (location - range.source_range_start);
                            }
                            None => {},
                        };

                });

            debug!("FINAL {}, {}", seed, location);
            location
        })
        .min().unwrap().to_string()
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

        assert_eq!(part1(&mut reader), "331445006");
    }

    #[test]
    fn part1_tests() {
        init();

        let f = File::open("input.example").unwrap();
        let mut reader = BufReader::new(f);

        assert_eq!(part1(&mut reader), "35");
    }
}
