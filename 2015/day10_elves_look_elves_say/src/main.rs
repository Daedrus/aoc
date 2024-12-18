use itertools::Itertools;
use log::{debug, info};
use std::io::{BufRead, Cursor};

fn look_and_say_slow(input: &str) -> String {
    input
        .chars()
        .chunk_by(|&x| x)
        .into_iter()
        .map(|(_, r)| r.collect())
        .collect::<Vec<String>>()
        .into_iter()
        .fold("".to_string(), |acc, s| {
            let c = s.chars().next().unwrap();
            let len = s.len().to_string();
            format!("{acc}{len}{c}")
        })
}

fn look_and_say_fast(input: &str) -> String {
    let mut result = String::new();
    let mut chars = input.chars();
    let mut prev_char = chars.next().unwrap();
    let mut count = 1;

    for c in chars {
        if c == prev_char {
            count += 1;
        } else {
            result.push_str(&count.to_string());
            result.push(prev_char);
            count = 1;
            prev_char = c;
        }
    }

    result.push_str(&count.to_string());
    result.push(prev_char);

    result
}

fn part1(input: &mut impl BufRead) -> String {
    let mut s = input.lines().next().unwrap().unwrap();

    for _ in 0..40 {
        s = look_and_say_slow(&s);
    }

    s.len().to_string()
}

fn part2(input: &mut impl BufRead) -> String {
    let mut s = input.lines().next().unwrap().unwrap();

    for i in 0..50 {
        s = look_and_say_fast(&s);
        debug!("{} {}", i, s.len());
    }

    s.len().to_string()
}

fn main() {
    env_logger::init();

    info!("Part 1 answer: {}", part1(&mut Cursor::new("1321131112")));

    info!("Part 2 answer: {}", part2(&mut Cursor::new("1321131112")));
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

        assert_eq!(part1(&mut Cursor::new("1321131112")), "492982");
        assert_eq!(part2(&mut Cursor::new("1321131112")), "6989950");
    }
}
