use log::info;
use std::io::{BufRead, Cursor};

fn find_digest_with_prefix(secret_key: &str, prefix: &str) -> Option<String> {
    let mut number: usize = 0;

    while number != usize::MAX {
        let data = secret_key.to_owned() + &number.to_string();

        if format!("{:x}", md5::compute(data)).starts_with(prefix) {
            return Some(number.to_string());
        }

        number += 1;
    }

    None
}

fn part1(input: &mut impl BufRead) -> Option<String> {
    let secret_key = input.lines().next().unwrap().unwrap();

    find_digest_with_prefix(&secret_key, "00000")
}

fn part2(input: &mut impl BufRead) -> Option<String> {
    let secret_key = input.lines().next().unwrap().unwrap();

    find_digest_with_prefix(&secret_key, "000000")
}

fn main() {
    env_logger::init();

    info!(
        "Part 1 answer: {}",
        part1(&mut Cursor::new("iwrupvqb")).unwrap()
    );

    info!(
        "Part 2 answer: {}",
        part2(&mut Cursor::new("iwrupvqb")).unwrap()
    );
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

        assert_eq!(
            part1(&mut Cursor::new("abcdef")),
            Some("609043".to_string())
        );
        assert_eq!(
            part1(&mut Cursor::new("pqrstuv")),
            Some("1048970".to_string())
        );
    }

    #[test]
    fn check_answers() {
        init();

        assert_eq!(
            part1(&mut Cursor::new("iwrupvqb")),
            Some("346386".to_string())
        );
        assert_eq!(
            part2(&mut Cursor::new("iwrupvqb")),
            Some("9958218".to_string())
        );
    }
}
