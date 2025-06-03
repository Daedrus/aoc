use log::{debug, info};
use nom::{
    branch::alt,
    bytes::complete::{tag, take},
    character::complete::alpha1,
    combinator::eof,
    multi::many0,
    sequence::{delimited, preceded, terminated},
    IResult, Parser,
};
use std::{
    fs::File,
    io::{self, BufRead, BufReader, Seek},
};

#[derive(Debug)]
struct StringFragment {
    characters_of_code: usize,
    characters_in_memory: usize,
    characters_in_encoded: usize,
}

fn ascii_code(input: &str) -> IResult<&str, &str> {
    preceded(tag("\\x"), take(2usize)).parse(input)
}

fn parse_string(input: &str) -> IResult<&str, Vec<StringFragment>> {
    delimited(
        tag("\""),
        many0(alt((
            ascii_code.map(|_| StringFragment {
                characters_of_code: 4,
                characters_in_memory: 1,
                characters_in_encoded: 5,
            }),
            tag("\\\"").map(|_| StringFragment {
                characters_of_code: 2,
                characters_in_memory: 1,
                characters_in_encoded: 4,
            }),
            tag("\\\\").map(|_| StringFragment {
                characters_of_code: 2,
                characters_in_memory: 1,
                characters_in_encoded: 4,
            }),
            alpha1.map(|s: &str| StringFragment {
                characters_of_code: s.len(),
                characters_in_memory: s.len(),
                characters_in_encoded: s.len(),
            }),
        ))),
        terminated(tag("\""), eof),
    )
    .parse(input)
    .map(|(s, mut v)| {
        // Manually add the beginning and end double quotes
        v.push(StringFragment {
            characters_of_code: 2,
            characters_in_memory: 0,
            characters_in_encoded: 6,
        });
        (s, v)
    })
}

fn get_character_stats(input_string: &str) -> (usize, usize, usize) {
    let string_fragments = parse_string(input_string).unwrap().1;

    debug!("{}, {:?}", input_string, string_fragments);

    string_fragments.into_iter().fold((0, 0, 0), |acc, x| {
        (
            acc.0 + x.characters_of_code,
            acc.1 + x.characters_in_memory,
            acc.2 + x.characters_in_encoded,
        )
    })
}

fn part1(input: &mut impl BufRead) -> String {
    input
        .lines()
        .map(|line| {
            let (characters_of_code, characters_in_memory, _) =
                get_character_stats(line.as_ref().unwrap());

            characters_of_code - characters_in_memory
        })
        .sum::<usize>()
        .to_string()
}

fn part2(input: &mut impl BufRead) -> String {
    input
        .lines()
        .map(|line| {
            let (characters_of_code, _, characters_in_encoded) =
                get_character_stats(line.as_ref().unwrap());

            characters_in_encoded - characters_of_code
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
    fn part1_tests() {
        init();

        assert_eq!(part1(&mut Cursor::new("\"\"")), "2");
        assert_eq!(part1(&mut Cursor::new("\"abc\"")), "2");
        assert_eq!(part1(&mut Cursor::new("\"aaa\\\"aaa\"")), "3");
        assert_eq!(part1(&mut Cursor::new("\"\\x27\"")), "5");
    }

    #[test]
    fn part2_tests() {
        init();

        assert_eq!(part2(&mut Cursor::new("\"\"")), "4");
        assert_eq!(part2(&mut Cursor::new("\"abc\"")), "4");
        assert_eq!(part2(&mut Cursor::new("\"aaa\\\"aaa\"")), "6");
        assert_eq!(part2(&mut Cursor::new("\"\\x27\"")), "5");
    }

    #[test]
    fn check_answers() {
        init();

        let f = File::open("input").unwrap();
        let mut reader = BufReader::new(f);

        assert_eq!(part1(&mut reader), "1371");
        reader.rewind().unwrap();
        assert_eq!(part2(&mut reader), "2117");
    }
}
