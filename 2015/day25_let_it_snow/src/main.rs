use log::info;
use nom::{bytes::complete::tag, character::complete, error::Error, IResult, Parser};
use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

fn parse_input(input: &mut impl BufRead) -> (u64, u64) {
    type InputLine<'a> = (&'a str, u64, &'a str, u64, &'a str);
    fn parse_line(input: &str) -> IResult<&str, InputLine, Error<&str>> {
        (
            tag("To continue, please consult the code grid in the manual.  Enter the code at row "),
            complete::u64,
            tag(", column "),
            complete::u64,
            tag("."),
        )
            .parse(input)
    }

    let row_column_line = input
        .lines()
        .take(1)
        .map(|line| line.unwrap())
        .collect::<String>();

    let (_, (_, row, _, column, _)) = parse_line(row_column_line.as_str()).unwrap();

    (row, column)
}

fn get_code_index(row: u64, column: u64) -> u64 {
    let diagonal: f64 = (row + column - 1) as f64;
    (diagonal * (diagonal + 1.0) / 2.0) as u64 - row + 1
}

fn part1(input: &mut impl BufRead) -> String {
    let (row, column) = parse_input(input);
    let mut code: u64 = 20151125;

    for _ in 1..get_code_index(row, column) {
        code = (code * 252533) % 33554393;
    }

    code.to_string()
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

        assert_eq!(part1(&mut reader), "2650453");
    }
}
