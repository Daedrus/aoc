use itertools::Itertools;
use log::{debug, info};
use nom::{bytes::complete::tag, character::complete, sequence::separated_pair, Parser};
use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{self, BufRead, BufReader, Seek},
};

fn parse_input(input: &mut impl BufRead) -> (HashSet<String>, HashMap<(String, String), u32>) {
    let mut locations: HashSet<String> = HashSet::new();
    let mut distances: HashMap<(String, String), u32> = HashMap::new();

    // fn parse_distance(input: &str) -> IResult<&str, ((&str, &str), u32)> {
    //     separated_pair(
    //         separated_pair(complete::alpha1, tag(" to "), complete::alpha1),
    //         tag(" = "),
    //         complete::u32,
    //     )(input)
    // }

    input.lines().for_each(|line| {
        let line = line.unwrap();
        let parser = separated_pair(
            separated_pair(
                // These type annotations are not needed in parse_distance
                complete::alpha1::<&str, nom::error::Error<&str>>,
                tag(" to "),
                complete::alpha1::<&str, nom::error::Error<&str>>,
            ),
            tag(" = "),
            complete::u32,
        )
        .parse(line.as_str());

        let (_, ((location1, location2), distance)) = parser.unwrap();

        locations.insert(location1.to_string());
        locations.insert(location2.to_string());
        distances.insert((location1.to_string(), location2.to_string()), distance);
        distances.insert((location2.to_string(), location1.to_string()), distance);
    });

    debug!("{:?}", locations);
    debug!("{:?}", distances);

    (locations, distances)
}

fn part1(input: &mut impl BufRead) -> String {
    let (locations, distances) = parse_input(input);

    locations
        .iter()
        .permutations(locations.len())
        .map(|permutation| {
            permutation
                .into_iter()
                .tuple_windows::<(&String, &String)>()
                .fold(0, |dist, (l1, l2)| {
                    dist + distances.get(&(l1.to_string(), l2.to_string())).unwrap()
                })
        })
        .min()
        .unwrap()
        .to_string()
}

fn part2(input: &mut impl BufRead) -> String {
    let (locations, distances) = parse_input(input);

    locations
        .iter()
        .permutations(locations.len())
        .map(|permutation| {
            permutation
                .into_iter()
                .tuple_windows::<(&String, &String)>()
                .fold(0, |dist, (l1, l2)| {
                    dist + distances.get(&(l1.to_string(), l2.to_string())).unwrap()
                })
        })
        .max()
        .unwrap()
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

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn part1_tests() {
        init();

        let f = File::open("input.example").unwrap();
        let mut reader = BufReader::new(f);

        assert_eq!(part1(&mut reader), "605");
    }

    #[test]
    fn part2_tests() {
        init();

        let f = File::open("input.example").unwrap();
        let mut reader = BufReader::new(f);

        assert_eq!(part2(&mut reader), "982");
    }

    #[test]
    fn check_answers() {
        init();

        let f = File::open("input").unwrap();
        let mut reader = BufReader::new(f);

        assert_eq!(part1(&mut reader), "141");
        reader.rewind().unwrap();
        assert_eq!(part2(&mut reader), "736");
    }
}
