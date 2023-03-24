use log::info;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete,
    sequence::{pair, terminated, tuple},
    IResult,
};
use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead, BufReader, Seek},
};

#[derive(Debug)]
struct Aunt {
    id: u32,
    properties: HashMap<String, u32>,
}

impl From<&str> for Aunt {
    fn from(value: &str) -> Self {
        let mut properties = HashMap::new();

        fn property_parser(input: &str) -> IResult<&str, (&str, u32)> {
            pair(
                alt((
                    terminated(tag("children"), tag(": ")),
                    terminated(tag("cats"), tag(": ")),
                    terminated(tag("samoyeds"), tag(": ")),
                    terminated(tag("pomeranians"), tag(": ")),
                    terminated(tag("akitas"), tag(": ")),
                    terminated(tag("vizslas"), tag(": ")),
                    terminated(tag("goldfish"), tag(": ")),
                    terminated(tag("trees"), tag(": ")),
                    terminated(tag("cars"), tag(": ")),
                    terminated(tag("perfumes"), tag(": ")),
                )),
                complete::u32,
            )(input)
        }

        let (_, (_, id, _, property1, _, property2, _, property3)) =
            tuple::<_, _, nom::error::Error<_>, _>((
                tag("Sue "),
                complete::u32,
                tag(": "),
                property_parser,
                tag(", "),
                property_parser,
                tag(", "),
                property_parser,
            ))(value)
            .unwrap();

        [property1, property2, property3]
            .iter()
            .for_each(|(property, amount)| {
                properties.insert(property.to_string(), *amount);
            });

        Self { id, properties }
    }
}

// Make sure that the comparison works no matter which aunt has more properties
impl PartialEq for Aunt {
    fn eq(&self, other: &Self) -> bool {
        if self.properties.len() <= other.properties.len() {
            self.properties.iter().all(|(property, amount)| {
                other
                    .properties
                    .get(property)
                    .map_or(false, |&other_amount| other_amount == *amount)
            })
        } else {
            other
                .properties
                .iter()
                .all(|(other_property, other_amount)| {
                    self.properties
                        .get(other_property)
                        .map_or(false, |&amount| amount == *other_amount)
                })
        }
    }
}

fn parse_input(input: &mut impl BufRead) -> Vec<Aunt> {
    input
        .lines()
        .map(|line| line.unwrap().as_str().into())
        .collect()
}

// Find which aunt in aunts is aunt Sue, use aunt_comparsion_method for comparison
fn find_aunt_sue(aunts: &[Aunt], aunt_comparison_method: fn(&Aunt, &Aunt) -> bool) -> String {
    let aunt_sue = Aunt {
        id: u32::MAX,
        properties: HashMap::from([
            ("children".to_string(), 3),
            ("cats".to_string(), 7),
            ("samoyeds".to_string(), 2),
            ("pomeranians".to_string(), 3),
            ("akitas".to_string(), 0),
            ("vizslas".to_string(), 0),
            ("goldfish".to_string(), 5),
            ("trees".to_string(), 3),
            ("cars".to_string(), 2),
            ("perfumes".to_string(), 1),
        ]),
    };

    aunts
        .iter()
        .find(|aunt| aunt_comparison_method(aunt, &aunt_sue))
        .unwrap()
        .id
        .to_string()
}

fn part1(input: &mut impl BufRead) -> String {
    find_aunt_sue(&parse_input(input), |aunt1, aunt2| aunt1 == aunt2)
}

fn part2(input: &mut impl BufRead) -> String {
    find_aunt_sue(&parse_input(input), |aunt1, aunt2| {
        aunt1
            .properties
            .iter()
            .all(
                |(aunt1_property, aunt1_amount)| match aunt1_property.as_str() {
                    "cats" | "trees" => aunt2
                        .properties
                        .get(aunt1_property)
                        .map_or(false, |&aunt2_amount| aunt2_amount < *aunt1_amount),
                    "pomeranians" | "goldfish" => aunt2
                        .properties
                        .get(aunt1_property)
                        .map_or(false, |&aunt2_amount| aunt2_amount > *aunt1_amount),
                    _ => aunt2
                        .properties
                        .get(aunt1_property)
                        .map_or(false, |&aunt2_amount| aunt2_amount == *aunt1_amount),
                },
            )
    })
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

        assert_eq!(part1(&mut reader), "40");
        reader.rewind().unwrap();
        assert_eq!(part2(&mut reader), "241");
    }
}
