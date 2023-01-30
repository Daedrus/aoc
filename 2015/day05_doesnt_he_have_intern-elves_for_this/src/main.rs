use itertools::Itertools;
use log::{debug, info};
use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead, BufReader, Seek},
};

fn part1<T: BufRead>(input: &mut T) -> usize {
    input
        .lines()
        .filter(|line| {
            let mut vowel_count = 0;
            let mut double_letter = false;
            let mut contains_ab_cd_pq_xy = false;

            let line = line.as_ref().unwrap();
            let mut tuple = line.chars().tuple_windows::<(char, char)>().peekable();

            while let Some((c1, c2)) = tuple.next() {
                if "aeiou".contains(c1) {
                    vowel_count += 1;
                }
                if tuple.peek().is_none() && "aeiou".contains(c2) {
                    vowel_count += 1;
                }

                if c1 == c2 {
                    double_letter = true;
                }

                let pair = c1.to_string() + &c2.to_string();
                if ["ab", "cd", "pq", "xy"].contains(&pair.as_str()) {
                    contains_ab_cd_pq_xy = true;
                }
            }

            debug!("{} {} {}", vowel_count, double_letter, contains_ab_cd_pq_xy);

            vowel_count >= 3 && double_letter && !contains_ab_cd_pq_xy
        })
        .count()
}

fn part2<T: BufRead>(input: &mut T) -> usize {
    input
        .lines()
        .filter(|line| {
            let mut repeating_letter = false;
            let mut pair_appears_twice = false;
            // Keep track of each pair and the position it is encountered in the string
            let mut char_pairs: HashMap<String, usize> = HashMap::new();

            let line = line.as_ref().unwrap();

            debug!("{:?}", line);

            // We slide a three character window through the string
            let tuple = line
                .chars()
                .tuple_windows::<(char, char, char)>()
                .enumerate();

            tuple.for_each(|(i, (c1, c2, c3))| {
                if c1 == c3 {
                    repeating_letter = true;
                }

                // We only add the c1+c2 pair in the first window, since otherwise
                // c1+c2 is the same as c2+c3 in the previous window and we don't
                // want to add the same pair twice.
                let pair = c1.to_string() + &c2.to_string();
                if i == 0 {
                    char_pairs.insert(pair, i);
                }

                // If insert() returns a value then the pair has been seen before
                let pair = c2.to_string() + &c3.to_string();
                if let Some(pos) = char_pairs.insert(pair.clone(), i + 1) {
                    // Current position of c2+c3 is i+1.
                    // If the previous position is i (aka i + 1 - 1) then the pairs
                    // are overlapping so it doesn't count as appearing twice.
                    if pos != i {
                        pair_appears_twice = true;
                        debug!("{} appears twice, in positions: {} {}", pair, i, pos);
                    } else {
                        // If the pairs overlap store the earlier position so that
                        // we can correctly handle strings such as "aaaa".
                        char_pairs.insert(pair, i);
                    }
                }
            });

            debug!(
                "{:?} {} {}",
                char_pairs, repeating_letter, pair_appears_twice
            );

            repeating_letter && pair_appears_twice
        })
        .count()
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

        assert_eq!(part1(&mut Cursor::new("ugknbfddgicrmopn")), 1);
        assert_eq!(part1(&mut Cursor::new("aaa")), 1);
        assert_eq!(part1(&mut Cursor::new("jchzalrnumimnmhp")), 0);
        assert_eq!(part1(&mut Cursor::new("haegwjzuvuyypxyu")), 0);
        assert_eq!(part1(&mut Cursor::new("dvszwmarrgswjxmb")), 0);
    }

    #[test]
    fn part2_tests() {
        init();

        assert_eq!(part2(&mut Cursor::new("qjhvhtzxzqqjkmpb")), 1);
        assert_eq!(part2(&mut Cursor::new("xxyxx")), 1);
        assert_eq!(part2(&mut Cursor::new("uurcxstgmygtbstg")), 0);
        assert_eq!(part2(&mut Cursor::new("ieodomkazucvgmuy")), 0);
        assert_eq!(part2(&mut Cursor::new("aaa")), 0);
        assert_eq!(part2(&mut Cursor::new("aaaa")), 1);
        assert_eq!(part2(&mut Cursor::new("aaaxyx")), 0);
        assert_eq!(part2(&mut Cursor::new("xyxaaa")), 0);
    }
}
