use log::info;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete,
    sequence::{pair, terminated},
    IResult, Parser,
};
use std::{
    fs::File,
    io::{self, BufRead, BufReader, Seek},
    ops::{Index, IndexMut},
};

#[derive(Debug, PartialEq, Eq)]
enum Register {
    A,
    B,
}

impl From<char> for Register {
    fn from(value: char) -> Self {
        match value {
            'a' => Register::A,
            'b' => Register::B,
            _ => unreachable!("Register is not supported"),
        }
    }
}

#[derive(Debug)]
struct Cpu {
    a: u64,
    b: u64,
    pc: i64,
}

impl Index<&Register> for Cpu {
    type Output = u64;

    fn index(&self, index: &Register) -> &Self::Output {
        match index {
            Register::A => &self.a,
            Register::B => &self.b,
        }
    }
}

impl IndexMut<&Register> for Cpu {
    fn index_mut(&mut self, index: &Register) -> &mut Self::Output {
        match index {
            Register::A => &mut self.a,
            Register::B => &mut self.b,
        }
    }
}

impl Cpu {
    fn execute_instruction(&mut self, instr: &Instruction) {
        match instr {
            Instruction::Hlf(reg) => {
                self[reg] /= 2;
                self.pc += 1;
            }
            Instruction::Tpl(reg) => {
                self[reg] *= 3;
                self.pc += 1;
            }
            Instruction::Inc(reg) => {
                self[reg] += 1;
                self.pc += 1;
            }
            Instruction::Jmp(offset) => self.pc += offset,
            Instruction::Jie(reg, offset) => {
                if self[reg] % 2 == 0 {
                    self.pc += offset;
                } else {
                    self.pc += 1;
                }
            }
            Instruction::Jio(reg, offset) => {
                if self[reg] == 1 {
                    self.pc += offset;
                } else {
                    self.pc += 1;
                }
            }
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
enum Instruction {
    Hlf(Register),
    Tpl(Register),
    Inc(Register),
    Jmp(i64),
    Jie(Register, i64),
    Jio(Register, i64),
}

impl From<&str> for Instruction {
    fn from(input: &str) -> Self {
        fn hlf_tpl_inc_instruction(input: &str) -> IResult<&str, Instruction> {
            pair(
                alt((tag("hlf "), tag("tpl "), tag("inc "))),
                complete::one_of("ab"),
            )
            .parse(input)
            .map(|(s, (instr, reg))| {
                (
                    s,
                    match instr {
                        "hlf " => Instruction::Hlf(reg.into()),
                        "tpl " => Instruction::Tpl(reg.into()),
                        "inc " => Instruction::Inc(reg.into()),
                        _ => unreachable!(),
                    },
                )
            })
        }

        fn jie_jio_instruction(input: &str) -> IResult<&str, Instruction> {
            (
                alt((tag("jio "), tag("jie "))),
                terminated(complete::one_of("ab"), tag(", ")),
                complete::i64,
            )
                .parse(input)
                .map(|(s, (instr, reg, offset))| {
                    (
                        s,
                        match instr {
                            "jio " => Instruction::Jio(reg.into(), offset),
                            "jie " => Instruction::Jie(reg.into(), offset),
                            _ => unreachable!(),
                        },
                    )
                })
        }

        fn jmp_instruction(input: &str) -> IResult<&str, Instruction> {
            pair(tag("jmp "), complete::i64)
                .parse(input)
                .map(|(s, (instr, offset))| {
                    (
                        s,
                        match instr {
                            "jmp " => Instruction::Jmp(offset),
                            _ => unreachable!(),
                        },
                    )
                })
        }

        alt((
            hlf_tpl_inc_instruction,
            jie_jio_instruction,
            jmp_instruction,
        ))
        .parse(input)
        .unwrap()
        .1
    }
}

fn run_program_with_starting_value_for_a(input: &mut impl BufRead, value: u64) -> String {
    let mut cpu = Cpu {
        a: value,
        b: 0,
        pc: 0,
    };
    let instructions: Vec<Instruction> = input
        .lines()
        .map(|line| line.as_ref().unwrap().as_str().into())
        .collect();

    loop {
        if cpu.pc >= 0 && cpu.pc < instructions.len().try_into().unwrap() {
            cpu.execute_instruction(&instructions[TryInto::<usize>::try_into(cpu.pc).unwrap()]);
        } else {
            break;
        }
    }

    cpu.b.to_string()
}

fn part1(input: &mut impl BufRead) -> String {
    run_program_with_starting_value_for_a(input, 0)
}

fn part2(input: &mut impl BufRead) -> String {
    run_program_with_starting_value_for_a(input, 1)
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
        init();

        assert_eq!(Instruction::from("hlf a"), Instruction::Hlf('a'.into()));
        assert_eq!(Instruction::from("hlf b"), Instruction::Hlf('b'.into()));
        assert_eq!(Instruction::from("tpl a"), Instruction::Tpl('a'.into()));
        assert_eq!(Instruction::from("tpl b"), Instruction::Tpl('b'.into()));
        assert_eq!(Instruction::from("inc a"), Instruction::Inc('a'.into()));
        assert_eq!(Instruction::from("inc b"), Instruction::Inc('b'.into()));
        assert_eq!(Instruction::from("jmp +23"), Instruction::Jmp(23));
        assert_eq!(Instruction::from("jmp -23"), Instruction::Jmp(-23));
        assert_eq!(
            Instruction::from("jie a, +10"),
            Instruction::Jie('a'.into(), 10)
        );
        assert_eq!(
            Instruction::from("jie b, -10"),
            Instruction::Jie('b'.into(), -10)
        );
        assert_eq!(
            Instruction::from("jio a, -5"),
            Instruction::Jio('a'.into(), -5)
        );
        assert_eq!(
            Instruction::from("jio b, 5"),
            Instruction::Jio('b'.into(), 5)
        );
    }

    #[test]
    fn check_answers() {
        init();

        let f = File::open("input").unwrap();
        let mut reader = BufReader::new(f);

        assert_eq!(part1(&mut reader), "184");
        reader.rewind().unwrap();
        assert_eq!(part2(&mut reader), "231");
    }
}
