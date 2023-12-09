use log::{debug, info};
use nom::bytes::complete::tag;
use nom::character::complete::{alphanumeric1, newline};
use nom::multi::separated_list1;
use nom::sequence::tuple;
use nom::IResult;
use num::integer::lcm;
use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead, BufReader, Seek},
};

#[derive(Debug)]
enum Instruction {
    Left,
    Right,
}

type Network = HashMap<String, (String, String)>;

fn parse_input(input: &mut impl BufRead) -> (Vec<Instruction>, Network) {
    fn node_parser(input: &str) -> IResult<&str, (&str, &str, &str)> {
        tuple((
            alphanumeric1,
            tag(" = ("),
            alphanumeric1,
            tag(", "),
            alphanumeric1,
            tag(")"),
        ))(input)
        .map(|(s, (node, _, node_l, _, node_r, _))| (s, (node, node_l, node_r)))
    }

    let mut lines: String = Default::default();
    input.read_to_string(&mut lines).unwrap();

    let (lines, instructions) =
        tuple::<_, _, nom::error::Error<_>, _>((alphanumeric1, newline, newline))(lines.as_str())
            .map(|(s, (instructions, _, _))| {
                (
                    s,
                    instructions
                        .chars()
                        .map(|c| match c {
                            'L' => Instruction::Left,
                            'R' => Instruction::Right,
                            _ => unreachable!(),
                        })
                        .collect::<Vec<Instruction>>(),
                )
            })
            .unwrap();

    let (_, node_mappings) = separated_list1(newline, node_parser)(lines).unwrap();

    let mut network: Network = HashMap::new();
    node_mappings.iter().for_each(|(node, node_l, node_r)| {
        network.insert(node.to_string(), (node_l.to_string(), node_r.to_string()));
    });

    (instructions, network)
}

fn number_of_steps(
    start_node: &str,
    is_end_node: fn(&str) -> bool,
    instructions: &[Instruction],
    network: &Network,
) -> u64 {
    let mut instruction_iterator = instructions.iter().cycle();
    let mut current_node = start_node;
    let mut steps: u64 = 0;

    while !is_end_node(current_node) {
        let next_nodes = network.get(current_node).unwrap();
        match instruction_iterator.next().unwrap() {
            Instruction::Left => current_node = &next_nodes.0,
            Instruction::Right => current_node = &next_nodes.1,
        }
        steps += 1;
    }

    steps
}

fn part1(input: &mut impl BufRead) -> String {
    let (instructions, network) = parse_input(input);

    debug!("{:?}", instructions);
    debug!("{:?}", network);

    number_of_steps("AAA", |node| node == "ZZZ", &instructions, &network).to_string()
}

fn part2(input: &mut impl BufRead) -> String {
    let (instructions, network) = parse_input(input);

    network
        .keys()
        .filter(|node| node.ends_with('A'))
        .map(|node| number_of_steps(node, |node| node.ends_with('Z'), &instructions, &network))
        .fold(1, lcm)
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
    fn check_answers() {
        init();

        let f = File::open("input").unwrap();
        let mut reader = BufReader::new(f);

        assert_eq!(part1(&mut reader), "20659");
        reader.rewind().unwrap();
        assert_eq!(part2(&mut reader), "15690466351717");
    }

    #[test]
    fn part1_tests() {
        init();

        let f = File::open("input.example1").unwrap();
        let mut reader = BufReader::new(f);

        assert_eq!(part1(&mut reader), "2");

        let f = File::open("input.example2").unwrap();
        let mut reader = BufReader::new(f);

        assert_eq!(part1(&mut reader), "6");
    }

    #[test]
    fn part2_tests() {
        init();

        let f = File::open("input.example3").unwrap();
        let mut reader = BufReader::new(f);

        assert_eq!(part2(&mut reader), "6");
    }
}
