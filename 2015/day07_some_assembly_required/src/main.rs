use log::{debug, info};
use nom::Parser;
#[cfg(not(test))]
use rand::{distr::Alphanumeric, Rng};
use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead, BufReader, Seek},
};

#[derive(Debug, PartialEq, Eq, Clone)]
struct Wire {
    name: String,
    value: Option<u16>,
}

// In the input file, the input wires to a gate can either be a
// number or a string. Handle both of these as a "Wire" and give
// random names to number inputs.
impl From<&str> for Wire {
    fn from(input: &str) -> Self {
        fn generate_random_string() -> String {
            #[cfg(not(test))]
            {
                rand::rng()
                    .sample_iter(&Alphanumeric)
                    .take(10)
                    .map(char::from)
                    .collect::<String>()
            }
            #[cfg(test)]
            {
                "TEST".to_string()
            }
        }

        match input.parse::<u16>() {
            Ok(value) => Wire {
                name: generate_random_string(),
                value: Some(value),
            },
            Err(_) => Wire {
                name: input.to_string(),
                value: None,
            },
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Gate {
    PassThrough {
        wire_in: Wire,
        wire_out: Wire,
    },
    And {
        wire_in_1: Wire,
        wire_in_2: Wire,
        wire_out: Wire,
    },
    LeftShift {
        wire_in_1: Wire,
        wire_in_2: Wire,
        wire_out: Wire,
    },
    Not {
        wire_in: Wire,
        wire_out: Wire,
    },
    Or {
        wire_in_1: Wire,
        wire_in_2: Wire,
        wire_out: Wire,
    },
    RightShift {
        wire_in_1: Wire,
        wire_in_2: Wire,
        wire_out: Wire,
    },
}

impl Gate {
    fn reset(&mut self) {
        match self {
            Gate::PassThrough { wire_out, .. }
            | Gate::And { wire_out, .. }
            | Gate::LeftShift { wire_out, .. }
            | Gate::Not { wire_out, .. }
            | Gate::Or { wire_out, .. }
            | Gate::RightShift { wire_out, .. } => {
                wire_out.value = None;
            }
        };
    }
}

impl From<&str> for Gate {
    fn from(input: &str) -> Self {
        use nom::{
            branch::alt,
            bytes::complete::tag,
            character::complete,
            sequence::{preceded, separated_pair},
            IResult,
        };

        // Assume that whitespaces in the input "behave" so that we don't
        // have to sprinkle complete::whitespace1 parsers everywhere
        fn pass_through(input: &str) -> IResult<&str, Gate> {
            separated_pair(
                alt((complete::digit1, complete::alpha1)),
                tag(" -> "),
                complete::alpha1,
            )
            .parse(input)
            .map(|(s, (wire_in, wire_out))| {
                (
                    s,
                    Gate::PassThrough {
                        wire_in: wire_in.into(),
                        wire_out: wire_out.into(),
                    },
                )
            })
        }

        fn and_or_shift(input: &str) -> IResult<&str, Gate> {
            separated_pair(
                (
                    alt((complete::digit1, complete::alpha1)),
                    alt((tag(" AND "), tag(" OR "), tag(" LSHIFT "), tag(" RSHIFT "))),
                    alt((complete::digit1, complete::alpha1)),
                ),
                tag(" -> "),
                complete::alpha1,
            )
            .parse(input)
            .map(|(s, ((wire_in_1, op, wire_in_2), wire_out))| {
                (
                    s,
                    match op {
                        " AND " => Gate::And {
                            wire_in_1: wire_in_1.into(),
                            wire_in_2: wire_in_2.into(),
                            wire_out: wire_out.into(),
                        },
                        " OR " => Gate::Or {
                            wire_in_1: wire_in_1.into(),
                            wire_in_2: wire_in_2.into(),
                            wire_out: wire_out.into(),
                        },
                        " LSHIFT " => Gate::LeftShift {
                            wire_in_1: wire_in_1.into(),
                            wire_in_2: wire_in_2.into(),
                            wire_out: wire_out.into(),
                        },
                        " RSHIFT " => Gate::RightShift {
                            wire_in_1: wire_in_1.into(),
                            wire_in_2: wire_in_2.into(),
                            wire_out: wire_out.into(),
                        },
                        _ => unreachable!(),
                    },
                )
            })
        }

        fn not(input: &str) -> IResult<&str, Gate> {
            separated_pair(
                preceded(tag("NOT "), complete::alpha1),
                tag(" -> "),
                complete::alpha1,
            )
            .parse(input)
            .map(|(s, (wire_in, wire_out))| {
                (
                    s,
                    Gate::Not {
                        wire_in: wire_in.into(),
                        wire_out: wire_out.into(),
                    },
                )
            })
        }

        alt((pass_through, and_or_shift, not))
            .parse(input)
            .unwrap()
            .1
    }
}

// Recursively figure out the signal value for the gate corresponding
// to the input name.
fn compute_signal_value(
    name: &str,
    wire_names_to_gates: &mut HashMap<String, Gate>,
) -> Option<u16> {
    let ret_val: Option<u16> = if let Some(gate) = wire_names_to_gates.get(name) {
        // Each gate follows the same logic
        // - if the output wire has a value, return that
        // - otherwise obtain the values of all input wires by recursively
        // calling the function (if the input values are not already known)
        // and then setting the output value on the output wire accordingly
        match gate {
            Gate::PassThrough { wire_in, wire_out } => {
                if wire_out.value.is_some() {
                    wire_out.value
                } else if wire_in.value.is_some() {
                    wire_in.value
                } else {
                    compute_signal_value(&wire_in.name.clone(), wire_names_to_gates)
                }
            }
            Gate::And {
                wire_in_1,
                wire_in_2,
                wire_out,
            } => {
                if wire_out.value.is_some() {
                    wire_out.value
                } else {
                    // Clone the input wires so that we can drop the reference to
                    // wire_names_to_gates. This is fine (albeit ugly) since we're
                    // not interested in storing the input wires' values
                    let wire_in_1 = wire_in_1.clone();
                    let wire_in_2 = wire_in_2.clone();

                    Some(
                        if let Some(value) = wire_in_1.value {
                            value
                        } else {
                            compute_signal_value(&wire_in_1.name, wire_names_to_gates).unwrap()
                        } & if let Some(value) = wire_in_2.value {
                            value
                        } else {
                            compute_signal_value(&wire_in_2.name, wire_names_to_gates).unwrap()
                        },
                    )
                }
            }
            Gate::LeftShift {
                wire_in_1,
                wire_in_2,
                wire_out,
            } => {
                if wire_out.value.is_some() {
                    wire_out.value
                } else {
                    let wire_in_1 = wire_in_1.clone();
                    let wire_in_2 = wire_in_2.clone();

                    Some(
                        if let Some(value) = wire_in_1.value {
                            value
                        } else {
                            compute_signal_value(&wire_in_1.name, wire_names_to_gates).unwrap()
                        } << if let Some(value) = wire_in_2.value {
                            value
                        } else {
                            compute_signal_value(&wire_in_2.name, wire_names_to_gates).unwrap()
                        },
                    )
                }
            }
            Gate::Not { wire_in, wire_out } => {
                if wire_out.value.is_some() {
                    wire_out.value
                } else if let Some(value) = wire_in.value {
                    Some(!value)
                } else {
                    Some(!compute_signal_value(&wire_in.name.clone(), wire_names_to_gates).unwrap())
                }
            }
            Gate::Or {
                wire_in_1,
                wire_in_2,
                wire_out,
            } => {
                if wire_out.value.is_some() {
                    wire_out.value
                } else {
                    let wire_in_1 = wire_in_1.clone();
                    let wire_in_2 = wire_in_2.clone();

                    Some(
                        if let Some(value) = wire_in_1.value {
                            value
                        } else {
                            compute_signal_value(&wire_in_1.name, wire_names_to_gates).unwrap()
                        } | if let Some(value) = wire_in_2.value {
                            value
                        } else {
                            compute_signal_value(&wire_in_2.name, wire_names_to_gates).unwrap()
                        },
                    )
                }
            }
            Gate::RightShift {
                wire_in_1,
                wire_in_2,
                wire_out,
            } => {
                if wire_out.value.is_some() {
                    wire_out.value
                } else {
                    let wire_in_1 = wire_in_1.clone();
                    let wire_in_2 = wire_in_2.clone();

                    Some(
                        if let Some(value) = wire_in_1.value {
                            value
                        } else {
                            compute_signal_value(&wire_in_1.name, wire_names_to_gates).unwrap()
                        } >> if let Some(value) = wire_in_2.value {
                            value
                        } else {
                            compute_signal_value(&wire_in_2.name, wire_names_to_gates).unwrap()
                        },
                    )
                }
            }
        }
    } else {
        None
    };

    if let Some(gate) = wire_names_to_gates.get_mut(name) {
        match gate {
            Gate::PassThrough { wire_out, .. }
            | Gate::And { wire_out, .. }
            | Gate::LeftShift { wire_out, .. }
            | Gate::Not { wire_out, .. }
            | Gate::Or { wire_out, .. }
            | Gate::RightShift { wire_out, .. } => {
                wire_out.value = ret_val;
            }
        };
    };

    ret_val
}

fn parse_input(input: &mut impl BufRead) -> HashMap<String, Gate> {
    let mut wire_names_to_gates: HashMap<String, Gate> = HashMap::new();

    input.lines().for_each(|line| {
        let gate: Gate = line.as_ref().unwrap().as_str().into();

        // Create a mapping between each wire name and the gate that produces its signal
        // We know that "each wire can only get a signal from one source"
        match &gate {
            Gate::PassThrough { wire_out, .. }
            | Gate::And { wire_out, .. }
            | Gate::LeftShift { wire_out, .. }
            | Gate::Not { wire_out, .. }
            | Gate::Or { wire_out, .. }
            | Gate::RightShift { wire_out, .. } => {
                wire_names_to_gates.insert(wire_out.name.to_string(), gate);
            }
        };
    });

    wire_names_to_gates
}

fn part1(input: &mut impl BufRead) -> String {
    let mut wire_names_to_gates = parse_input(input);

    wire_names_to_gates.iter().for_each(|mapping| {
        debug!("{:?}", mapping);
    });

    compute_signal_value("a", &mut wire_names_to_gates)
        .unwrap()
        .to_string()
}

fn part2(input: &mut impl BufRead) -> String {
    let mut wire_names_to_gates = parse_input(input);

    let a_signal_value = compute_signal_value("a", &mut wire_names_to_gates)
        .unwrap()
        .to_string();

    wire_names_to_gates.values_mut().for_each(|gate| {
        gate.reset();
    });

    // Replace b's gate with a PassThrough gate with a's value as input
    wire_names_to_gates.insert("b".to_string(), (a_signal_value + " -> b").as_str().into());

    wire_names_to_gates.iter().for_each(|mapping| {
        debug!("{:?}", mapping);
    });

    compute_signal_value("a", &mut wire_names_to_gates)
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
    fn parser_tests() {
        assert_eq!(
            Gate::from("123 -> x"),
            Gate::PassThrough {
                wire_in: Wire {
                    name: "TEST".to_string(),
                    value: Some(123)
                },
                wire_out: Wire {
                    name: "x".to_string(),
                    value: None
                }
            }
        );

        assert_eq!(
            Gate::from("abc -> x"),
            Gate::PassThrough {
                wire_in: Wire {
                    name: "abc".to_string(),
                    value: None
                },
                wire_out: Wire {
                    name: "x".to_string(),
                    value: None
                }
            }
        );

        assert_eq!(
            Gate::from("x AND y -> d"),
            Gate::And {
                wire_in_1: Wire {
                    name: "x".to_string(),
                    value: None
                },
                wire_in_2: Wire {
                    name: "y".to_string(),
                    value: None
                },
                wire_out: Wire {
                    name: "d".to_string(),
                    value: None
                }
            }
        );

        assert_eq!(
            Gate::from("x OR y -> e"),
            Gate::Or {
                wire_in_1: Wire {
                    name: "x".to_string(),
                    value: None
                },
                wire_in_2: Wire {
                    name: "y".to_string(),
                    value: None
                },
                wire_out: Wire {
                    name: "e".to_string(),
                    value: None
                }
            }
        );

        assert_eq!(
            Gate::from("x LSHIFT 2 -> f"),
            Gate::LeftShift {
                wire_in_1: Wire {
                    name: "x".to_string(),
                    value: None
                },
                wire_in_2: Wire {
                    name: "TEST".to_string(),
                    value: Some(2)
                },
                wire_out: Wire {
                    name: "f".to_string(),
                    value: None
                }
            }
        );

        assert_eq!(
            Gate::from("y RSHIFT 2 -> g"),
            Gate::RightShift {
                wire_in_1: Wire {
                    name: "y".to_string(),
                    value: None
                },
                wire_in_2: Wire {
                    name: "TEST".to_string(),
                    value: Some(2)
                },
                wire_out: Wire {
                    name: "g".to_string(),
                    value: None
                }
            }
        );

        assert_eq!(
            Gate::from("NOT x -> h"),
            Gate::Not {
                wire_in: Wire {
                    name: "x".to_string(),
                    value: None
                },
                wire_out: Wire {
                    name: "h".to_string(),
                    value: None
                }
            }
        );
    }

    #[test]
    fn part1_tests() {
        init();

        let f = File::open("input.example").unwrap();
        let mut reader = BufReader::new(f);

        let mut wire_names_to_gates = parse_input(&mut reader);

        assert_eq!(
            compute_signal_value("d", &mut wire_names_to_gates),
            Some(72)
        );
        assert_eq!(
            compute_signal_value("e", &mut wire_names_to_gates),
            Some(507)
        );
        assert_eq!(
            compute_signal_value("f", &mut wire_names_to_gates),
            Some(492)
        );
        assert_eq!(
            compute_signal_value("g", &mut wire_names_to_gates),
            Some(114)
        );
        assert_eq!(
            compute_signal_value("h", &mut wire_names_to_gates),
            Some(65412)
        );
        assert_eq!(
            compute_signal_value("i", &mut wire_names_to_gates),
            Some(65079)
        );
        assert_eq!(
            compute_signal_value("x", &mut wire_names_to_gates),
            Some(123)
        );
        assert_eq!(
            compute_signal_value("y", &mut wire_names_to_gates),
            Some(456)
        );
    }

    #[test]
    fn check_answers() {
        init();

        let f = File::open("input").unwrap();
        let mut reader = BufReader::new(f);

        assert_eq!(part1(&mut reader), "46065");
        reader.rewind().unwrap();
        assert_eq!(part2(&mut reader), "14134");
    }
}
