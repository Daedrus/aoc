use itertools::Itertools;
use log::info;
use std::{
    fmt::Display,
    io::{BufRead, Cursor},
    ops::ControlFlow,
};

struct Password([char; 8]);

impl Password {
    fn inc(&mut self) {
        let Password(password) = self;
        let _ = password.iter_mut().rev().try_for_each(|c| {
            if *c == 'z' {
                *c = 'a';
                ControlFlow::Continue(())
            } else {
                *c = std::char::from_u32(*c as u32 + 1).unwrap();
                ControlFlow::Break(c)
            }
        });
    }

    // Avoid generating passwords which contain 'i', 'o', 'l'
    fn inc_smart(&mut self) {
        let Password(password) = self;
        let mut found_i_o_l = false;

        // Find the first i,o or l, increment it and set everything after to a
        password.iter_mut().for_each(|c| {
            if found_i_o_l {
                *c = 'a';
            } else if *c == 'i' || *c == 'o' || *c == 'l' {
                *c = std::char::from_u32(*c as u32 + 1).unwrap();
                found_i_o_l = true;
            }
        });

        // Default back to regular inc() if we didn't find any i,o,l
        // This also works if i,o,l is the last character in the password
        if !found_i_o_l {
            self.inc();
        }
    }

    fn is_valid(&self) -> bool {
        fn contains_i_o_l(password: &[char; 8]) -> bool {
            password.contains(&'i') || password.contains(&'o') || password.contains(&'l')
        }

        fn contains_three_letter_straight(password: &[char; 8]) -> bool {
            password
                .iter()
                .tuple_windows::<(&char, &char, &char)>()
                .fold(false, |res, (c1, c2, c3)| {
                    res || (*c2 as u32 == *c1 as u32 + 1 && *c3 as u32 == *c2 as u32 + 1)
                })
        }

        fn contains_at_least_two_nonoverlapping_pairs(password: &[char; 8]) -> bool {
            password
                .iter()
                .tuple_windows::<(&char, &char, &char)>()
                .enumerate()
                .filter(|(i, (c1, c2, c3))| {
                    if *i == 0 {
                        *c1 == *c2 && *c2 != *c3 || *c2 == *c3 && *c2 != *c1
                    } else {
                        *c2 == *c3 && *c2 != *c1
                    }
                })
                .count()
                >= 2
        }

        let Password(password) = self;

        !contains_i_o_l(password)
            && contains_three_letter_straight(password)
            && contains_at_least_two_nonoverlapping_pairs(password)
    }

    fn set_to_next_valid(&mut self) {
        self.inc();
        loop {
            if self.is_valid() {
                break;
            } else {
                self.inc_smart();
            }
        }
    }
}

impl TryFrom<&str> for Password {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.len() != 8 {
            Err("Passwords can only be 8 characters long")
        } else {
            Ok(Password(
                value.chars().collect::<Vec<char>>().try_into().unwrap(),
            ))
        }
    }
}

impl Display for Password {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Password(pass) = self;
        write!(f, "{}", pass.iter().collect::<String>())
    }
}

fn part1(input: &mut impl BufRead) -> String {
    let mut pass: Password = input
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .as_str()
        .try_into()
        .unwrap();

    pass.set_to_next_valid();

    pass.to_string()
}

fn part2(input: &mut impl BufRead) -> String {
    let mut pass: Password = input
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .as_str()
        .try_into()
        .unwrap();

    pass.set_to_next_valid();

    pass.to_string()
}

fn main() {
    env_logger::init();

    info!("Part 1 answer: {}", part1(&mut Cursor::new("hepxcrrq")));

    info!("Part 2 answer: {}", part2(&mut Cursor::new("hepxxyzz")));
}

#[cfg(test)]
mod tests {
    use crate::*;

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn test_password() {
        init();

        let mut pass: Password = "aaaaaaaa".try_into().unwrap();
        pass.inc_smart();
        assert_eq!(pass.to_string(), "aaaaaaab");

        pass = "aaaaaaaz".try_into().unwrap();
        pass.inc_smart();
        assert_eq!(pass.to_string(), "aaaaaaba");

        pass = "aaaaaazz".try_into().unwrap();
        pass.inc_smart();
        assert_eq!(pass.to_string(), "aaaaabaa");

        pass = "aaaazazz".try_into().unwrap();
        pass.inc_smart();
        assert_eq!(pass.to_string(), "aaaazbaa");

        pass = "zzzzzzzz".try_into().unwrap();
        pass.inc_smart();
        assert_eq!(pass.to_string(), "aaaaaaaa");

        pass = "aaaaibbb".try_into().unwrap();
        pass.inc_smart();
        assert_eq!(pass.to_string(), "aaaajaaa");

        pass = "aaaaaaao".try_into().unwrap();
        pass.inc_smart();
        assert_eq!(pass.to_string(), "aaaaaaap");

        pass = "lbbbbbbb".try_into().unwrap();
        pass.inc_smart();
        assert_eq!(pass.to_string(), "maaaaaaa");

        pass = "ghijklmn".try_into().unwrap();
        pass.inc_smart();
        assert_eq!(pass.to_string(), "ghjaaaaa");
    }

    #[test]
    fn test_valid_password() {
        init();

        assert_eq!(Password::try_from("hijklmmn").unwrap().is_valid(), false);
        assert_eq!(Password::try_from("abbceffg").unwrap().is_valid(), false);
        assert_eq!(Password::try_from("abbcegjk").unwrap().is_valid(), false);
        assert_eq!(Password::try_from("abcdffaa").unwrap().is_valid(), true);
        assert_eq!(Password::try_from("ghjaabcc").unwrap().is_valid(), true);

        let mut pass: Password = "abcdefgh".try_into().unwrap();
        pass.set_to_next_valid();
        assert_eq!(pass.to_string(), "abcdffaa");

        pass = "ghijklmn".try_into().unwrap();
        pass.set_to_next_valid();
        assert_eq!(pass.to_string(), "ghjaabcc");
    }

    #[test]
    fn check_answers() {
        init();

        assert_eq!(part1(&mut Cursor::new("hepxcrrq")), "hepxxyzz");
        assert_eq!(part2(&mut Cursor::new("hepxxyzz")), "heqaabcc");
    }
}
