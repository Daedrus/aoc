use log::{debug, info};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete,
    multi::separated_list1,
    sequence::{pair, tuple},
    IResult,
};
use std::{
    cmp::max,
    fs::File,
    io::{self, BufRead, BufReader, Seek},
};

#[derive(Debug)]
struct Reveal {
    red_cubes: u32,
    green_cubes: u32,
    blue_cubes: u32,
}

#[derive(Debug)]
struct Game {
    index: u32,
    reveals: Vec<Reveal>,
}

const MAX_RED_CUBES: u32 = 12;
const MAX_GREEN_CUBES: u32 = 13;
const MAX_BLUE_CUBES: u32 = 14;

fn parse_input(input: &mut impl BufRead) -> Vec<Game> {
    fn color_parser(input: &str) -> IResult<&str, (u32, &str)> {
        pair(
            complete::u32,
            alt((tag(" red"), tag(" green"), tag(" blue"))),
        )(input)
    }

    fn reveal_parser(input: &str) -> IResult<&str, Reveal> {
        fn get_number_of_cubes(array_of_cubes: &[(u32, &str)], color: &str) -> u32 {
            array_of_cubes
                .iter()
                .find(|(_, c)| c == &color)
                .map_or(0, |(number_of_cubes, _)| *number_of_cubes)
        }

        separated_list1(tag(", "), color_parser)(input).map(|(s, v)| {
            (
                s,
                Reveal {
                    red_cubes: get_number_of_cubes(&v, " red"),
                    green_cubes: get_number_of_cubes(&v, " green"),
                    blue_cubes: get_number_of_cubes(&v, " blue"),
                },
            )
        })
    }

    input
        .lines()
        .map(|line| {
            let line = line.unwrap();

            let (_, (_, index, _, reveals)) = tuple::<_, _, nom::error::Error<_>, _>((
                tag("Game "),
                complete::u32,
                tag(": "),
                separated_list1(tag("; "), reveal_parser),
            ))(line.as_str())
            .unwrap();

            Game { index, reveals }
        })
        .collect()
}

fn part1(input: &mut impl BufRead) -> String {
    let games: Vec<Game> = parse_input(input);

    let possible_games_id_sum = games
        .iter()
        .filter(|game| {
            game.reveals.iter().all(|reveal| {
                reveal.red_cubes <= MAX_RED_CUBES
                    && reveal.green_cubes <= MAX_GREEN_CUBES
                    && reveal.blue_cubes <= MAX_BLUE_CUBES
            })
        })
        .fold(0, |acc, game| acc + game.index);

    debug!("{:?}", games);

    possible_games_id_sum.to_string()
}

fn part2(input: &mut impl BufRead) -> String {
    let games: Vec<Game> = parse_input(input);

    let power_of_sets_sum: u32 = games
        .iter()
        .map(|game| {
            game.reveals
                .iter()
                .fold((0, 0, 0), |(red_cubes, green_cubes, blue_cubes), reveal| {
                    (
                        max(red_cubes, reveal.red_cubes),
                        max(green_cubes, reveal.green_cubes),
                        max(blue_cubes, reveal.blue_cubes),
                    )
                })
        })
        .map(
            |(fewest_red_cubes, fewest_green_cubes, fewest_blue_cubes)| {
                fewest_red_cubes * fewest_green_cubes * fewest_blue_cubes
            },
        )
        .sum();

    power_of_sets_sum.to_string()
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

        assert_eq!(part1(&mut reader), "2476");
        reader.rewind().unwrap();
        assert_eq!(part2(&mut reader), "54911");
    }

    #[test]
    fn part1_tests() {
        init();

        let f = File::open("input.example").unwrap();
        let mut reader = BufReader::new(f);

        assert_eq!(part1(&mut reader), "8");
    }

    #[test]
    fn part2_tests() {
        init();

        let f = File::open("input.example").unwrap();
        let mut reader = BufReader::new(f);

        assert_eq!(part2(&mut reader), "2286");
    }
}
