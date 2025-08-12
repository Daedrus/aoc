use log::{debug, info};
use nom::{
    bytes::complete::tag,
    character::complete::{self, newline, space1},
    error::Error,
    multi::separated_list1,
    IResult, Parser,
};
use std::{
    fs::File,
    io::{self, BufRead, BufReader, Seek},
};

fn parse_input(input: &mut impl BufRead) -> (Vec<u64>, Vec<u64>) {
    let mut lines: String = Default::default();
    input.read_to_string(&mut lines).unwrap();

    type InputLine<'a> = (&'a str, &'a str, Vec<u64>, char, &'a str, &'a str, Vec<u64>);
    fn parse_line(input: &str) -> IResult<&str, InputLine<'_>, Error<&str>> {
        (
            tag("Time:"),
            space1,
            separated_list1(space1, complete::u64),
            newline,
            tag("Distance:"),
            space1,
            separated_list1(space1, complete::u64),
        )
            .parse(input)
    }

    let (_, (_, _, times, _, _, _, distances)) = parse_line(lines.as_str()).unwrap();

    (times, distances)
}

fn compute_ways_to_beat_record(times: &[u64], distances: &[u64]) -> String {
    times
        .iter()
        .zip(distances.iter())
        .map(|(t, d)| {
            // The equation we solve is:
            //   x(t-x) > d
            //
            //   -x^2 + xt - d > 0
            //
            //   x^2 - xt + d < 0
            //
            //   1 * x^2 - t * x + d < 0
            //   ^       ^^^       ^
            //   a       b         c
            //
            // The roots of the quadratic equation are:
            //   x = (-b +- sqrt(b^2 - 4ac)) / 2
            //
            //   where a = 1, b = -t, c = d
            //
            //   x = (t +- sqrt(t^2 - 4d)) / 2
            //
            // We assume that there are no complex roots

            let t: f64 = *t as f64;
            let d: f64 = *d as f64;
            let leftmost_root: f64 = (t - f64::sqrt(t.powi(2) - 4.0 * d)) / 2.0;
            let rightmost_root: f64 = (t + f64::sqrt(t.powi(2) - 4.0 * d)) / 2.0;

            debug!("{} {}", leftmost_root, rightmost_root);

            // Since the leading coefficient (a) is 1 then the functions will
            // be concave downward which means they will be smaller than 0
            // between the roots. So the values that "beat the record" are
            // the integers betwen the ceiling of the leftmost root and the
            // floor of the rightmost root. Adjust -/+ 1 in case the roots are
            // integers.

            let rightmost_integer = if rightmost_root.fract() == 0.0 {
                rightmost_root - 1.0
            } else {
                rightmost_root.floor()
            };
            let leftmost_integer = if leftmost_root.fract() == 0.0 {
                leftmost_root + 1.0
            } else {
                leftmost_root.ceil()
            };

            rightmost_integer - leftmost_integer + 1.0
        })
        .product::<f64>()
        .to_string()
}

fn part1(input: &mut impl BufRead) -> String {
    let (times, distances) = parse_input(input);

    debug!("{:?}", times);
    debug!("{:?}", distances);

    compute_ways_to_beat_record(&times, &distances)
}

fn part2(input: &mut impl BufRead) -> String {
    let (mut times, mut distances) = parse_input(input);

    let concatenated_times: u64 = times
        .iter()
        .map(|t| t.to_string())
        .collect::<String>()
        .parse()
        .unwrap();
    let concatenated_distances: u64 = distances
        .iter()
        .map(|d| d.to_string())
        .collect::<String>()
        .parse()
        .unwrap();

    debug!("{}", concatenated_times);
    debug!("{}", concatenated_distances);

    times.clear();
    distances.clear();

    times.push(concatenated_times);
    distances.push(concatenated_distances);

    compute_ways_to_beat_record(&times, &distances)
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

        assert_eq!(part1(&mut reader), "345015");
        reader.rewind().unwrap();
        assert_eq!(part2(&mut reader), "42588603");
    }

    #[test]
    fn part1_tests() {
        init();

        let f = File::open("input.example").unwrap();
        let mut reader = BufReader::new(f);

        assert_eq!(part1(&mut reader), "288");
    }

    #[test]
    fn part2_tests() {
        init();

        let f = File::open("input.example").unwrap();
        let mut reader = BufReader::new(f);

        assert_eq!(part2(&mut reader), "71503");
    }
}
