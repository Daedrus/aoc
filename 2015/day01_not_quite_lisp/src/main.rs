use log::{debug, info};
use std::{
    fs::File,
    io::{self, BufRead, BufReader, Seek},
};

fn part1(input: &mut impl BufRead) -> String {
    input
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .chars()
        .fold(0, |acc, c| {
            debug!("{} {}", acc, c);
            match c {
                ')' => acc - 1,
                '(' => acc + 1,
                _ => unreachable!(),
            }
        })
        .to_string()
}

fn part2(input: &mut impl BufRead) -> String {
    match input
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .chars()
        .zip((1..).into_iter())
        .try_fold(0, |acc, (c, pos)| {
            debug!("{} {}", pos, c);
            let mut next = acc;
            match (c, pos) {
                (')', _) => {
                    next -= 1;
                }
                ('(', _) => {
                    next += 1;
                }
                _ => unreachable!(),
            };
            if next == -1 {
                Err(pos)
            } else {
                Ok(next)
            }
        }) {
        Err(pos) => pos.to_string(),
        Ok(_) => "-1".to_string(),
    }
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
    use std::io::Cursor;

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn part1_tests() {
        init();

        assert_eq!(part1(&mut Cursor::new("(())")), "0");
        assert_eq!(part1(&mut Cursor::new("()()")), "0");
        assert_eq!(part1(&mut Cursor::new("(((")), "3");
        assert_eq!(part1(&mut Cursor::new("(()(()(")), "3");
        assert_eq!(part1(&mut Cursor::new("))(((((")), "3");
        assert_eq!(part1(&mut Cursor::new("())")), "-1");
        assert_eq!(part1(&mut Cursor::new("))(")), "-1");
        assert_eq!(part1(&mut Cursor::new(")))")), "-3");
        assert_eq!(part1(&mut Cursor::new(")())())")), "-3");
    }

    #[test]
    fn part2_tests() {
        init();

        assert_eq!(part2(&mut Cursor::new(")")), "1");
        assert_eq!(part2(&mut Cursor::new("()())")), "5");
        assert_eq!(part2(&mut Cursor::new("((((")), "-1");
    }

    #[test]
    fn check_answers() {
        init();

        let f = File::open("input").unwrap();
        let mut reader = BufReader::new(f);

        assert_eq!(part1(&mut reader), "138");
        reader.rewind().unwrap();
        assert_eq!(part2(&mut reader), "1771");
    }
}
