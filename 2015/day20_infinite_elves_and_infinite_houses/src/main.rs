use generator::{done, Gn};
use log::{debug, info};
use std::io::{BufRead, Cursor};

// Macro that defines a generator that generates all divisors
// of a given number
macro_rules! divisors {
    ($number: ident) => {{
        Gn::new_scoped(|mut s| {
            for i in 1..=($number as f64).sqrt() as u64 + 1 {
                if $number % i == 0 {
                    s.yield_(i);
                    if ($number / i != i) {
                        s.yield_($number / i);
                    }
                }
            }
            done!();
        })
    }};
}

fn part1(input: &mut impl BufRead) -> String {
    let num: u64 = input
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .parse::<u64>()
        .unwrap();

    for house in 1..=std::u64::MAX {
        if divisors!(house).sum::<u64>() * 10 >= num {
            return house.to_string();
        }
    }

    "".to_string()
}

fn part2(input: &mut impl BufRead) -> String {
    let num: u64 = input
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .parse::<u64>()
        .unwrap();

    for house in 1..=std::u64::MAX {
        if divisors!(house).filter(|d| *d * 50 >= house).sum::<u64>() * 11 >= num {
            return house.to_string();
        }
    }

    "".to_string()
}

fn main() {
    env_logger::init();

    info!("Part 1 answer: {}", part1(&mut Cursor::new("33100000")));

    info!("Part 2 answer: {}", part2(&mut Cursor::new("33100000")));
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

        assert_eq!(part1(&mut Cursor::new("33100000")), "776160");
        assert_eq!(part2(&mut Cursor::new("33100000")), "786240");
    }
}
