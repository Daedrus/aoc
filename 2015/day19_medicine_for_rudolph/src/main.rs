// This implementation is not complete, it is missing part2, that's why
// we allow unused imports here.
#![allow(unused_imports)]

use itertools::iproduct;
use log::{debug, info};
use nom::{bytes::complete::tag, character::complete, sequence::tuple};
use regex::Regex;
use std::{
    collections::{HashSet, VecDeque, HashMap},
    fs::File,
    io::{self, BufRead, BufReader, Seek},
};

#[derive(Debug)]
struct Rule {
    from: String,
    to: String,
}

fn parse_input(input: &mut impl BufRead) -> (Vec<Rule>, String) {
    // This is not that efficient, but I can't think of an easier way to skip
    // the last two lines in the file. Normally I'd add state to the parsing
    // stage and move from one state to another depending on the parsing
    // results. But this is aoc and the input files aren't that large to begin
    // with.

    // So start by reading the entire file in memory and put the lines in a
    // double ended queue.
    let lines = input
        .lines()
        .map(|line| line.unwrap())
        .collect::<VecDeque<String>>();

    // Then read the molecule string from the last line
    let molecule = lines.iter().last().unwrap();

    // Then read the rules, skip the last line and the empty line preceding it
    let rules = lines
        .iter()
        .rev()
        .skip(2)
        .map(|line| {
            let (_, (from, _, to)) = tuple::<_, _, nom::error::Error<_>, _>((
                complete::alpha1,
                tag(" => "),
                complete::alpha1,
            ))(line.as_str())
            .unwrap();

            Rule {
                from: from.to_string(),
                to: to.to_string(),
            }
        })
        .collect();

    (rules, molecule.to_string())
}

fn part1(input: &mut impl BufRead) -> String {
    let (rules, molecule) = parse_input(input);
    let mut new_molecules: HashSet<String> = HashSet::new();

    debug!("{:?}", rules);
    debug!("{:?}", molecule);

    rules.iter().for_each(|rule| {
        molecule.match_indices(&rule.from).for_each(|(index, _)| {
            let mut molecule_copy = molecule.clone();
            molecule_copy.replace_range(index..index + rule.from.len(), &rule.to);
            new_molecules.insert(molecule_copy);
        });
    });

    debug!("{:?}", new_molecules);

    new_molecules.len().to_string()
}

fn part2(_input: &mut impl BufRead) -> String {
    // let (rules, molecule) = parse_input(input);
    //
    // let mut inverted_rules = HashMap::new();
    // rules.iter().for_each(|rule| {
    //     inverted_rules.entry(&rule.to).or_insert(Vec::new()).push(&rule.from);
    // });
    //
    // debug!("{:?}", inverted_rules);
    //
    // // This is the first time I actually preferred a regex instead of nom.
    // // There doesn't seem to be any built-in way to detect character case
    // // in nom + I can't figure out how to express "zero or one repetitions".
    // let atoms = Regex::new(r"([A-Z][a-z]?)")
    //     .unwrap()
    //     .find_iter(&molecule)
    //     .map(|atom| atom.as_str())
    //     .collect::<Vec<&str>>();
    // let atoms_num = atoms.len();
    //
    // let mut cyk: Vec<Vec<Vec<String>>> = vec![vec![Vec::new(); atoms.len()]; atoms.len() + 1];
    // let mut backref: Vec<Vec<Vec<String>>> = cyk.clone();
    //
    // atoms.into_iter().enumerate().for_each(|(i, atom)| {
    //     cyk[0][i].push(atom.to_string());
    // });
    //
    // // 6 x
    // // 5 x x
    // // 4 x x x
    // // 3 x x x x
    // // 2 x x x x x
    // // 1 H O H O H O
    // // 0 H O H O H O
    // //   0 1 2 3 4 5
    //
    // let first_row = cyk[0].clone();
    // for (i, elem) in cyk[1].iter_mut().enumerate() {
    //     if let Some(rules) = inverted_rules.get(first_row[i].first().unwrap()) {
    //         rules.iter().for_each(|rule| {
    //             if rule.as_str() == "e" {
    //                 elem.push(first_row[i].first().unwrap().to_string());
    //             } else {
    //                 elem.push(rule.to_string());
    //             }
    //         })
    //     }
    // }
    //
    // debug!("{:?}", cyk);
    //
    // for y in 1..atoms_num {
    //     for x in 0..atoms_num-y {
    //         for i in 0..y {
    //             let need = iproduct!(&cyk[i][x], &cyk[y-i-1][x+i+1]);
    //             debug!("y:{}, x:{}, i:{}", y, x, i);
    //             debug!("cyk[{}][{}], cyk[{}][{}]", i, x, y-i-1, x+i+1);
    //             for n in need {
    //                 debug!("need:{:?}", n);
    //             }
    //         }
    //     }
    // }
    //
    // debug!("{:?}", cyk);
    //
    "".to_string()
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

        let f1 = File::open("input.example1").unwrap();
        let mut reader = BufReader::new(f1);

        assert_eq!(part1(&mut reader), "4");

        let f2 = File::open("input.example2").unwrap();
        let mut reader = BufReader::new(f2);

        assert_eq!(part1(&mut reader), "7");
    }

    #[test]
    fn part2_tests() {
        init();

        let f = File::open("input.example3").unwrap();
        let mut reader = BufReader::new(f);

        assert_eq!(part2(&mut reader), "");
    }

    #[test]
    fn check_answers() {
        init();

        let f = File::open("input").unwrap();
        let mut reader = BufReader::new(f);

        assert_eq!(part1(&mut reader), "509");
        reader.rewind().unwrap();
        assert_eq!(part2(&mut reader), "");
    }
}
