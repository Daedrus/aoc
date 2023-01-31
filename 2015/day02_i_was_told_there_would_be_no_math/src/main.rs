use log::{debug, info};
use std::{
    fs::File,
    io::{self, BufRead, BufReader, Seek},
};

struct Gift {
    length: usize,
    width: usize,
    height: usize,
}

fn gifts<T: BufRead>(input: &mut T) -> Vec<Gift> {
    input
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let mut gift = line.split('x').map(|d| d.parse::<usize>().unwrap());
            Gift {
                length: gift.next().unwrap(),
                width: gift.next().unwrap(),
                height: gift.next().unwrap(),
            }
        })
        .collect::<Vec<Gift>>()
}

fn part1(input: &mut impl BufRead) -> String {
    gifts(input)
        .iter()
        .map(|gift| {
            let areas = vec![
                gift.length * gift.width,
                gift.length * gift.height,
                gift.width * gift.height,
            ];

            debug!(
                "{}, {}, {}, {:?}",
                gift.length, gift.width, gift.height, areas
            );

            areas.iter().sum::<usize>() * 2 + areas.iter().min().unwrap()
        })
        .sum::<usize>()
        .to_string()
}

fn part2(input: &mut impl BufRead) -> String {
    gifts(input)
        .iter()
        .map(|gift| {
            let perimeters = vec![
                2 * (gift.length + gift.width),
                2 * (gift.length + gift.height),
                2 * (gift.width + gift.height),
            ];

            debug!(
                "{}, {}, {}, {:?}",
                gift.length, gift.width, gift.height, perimeters
            );

            perimeters.iter().min().unwrap() + gift.length * gift.width * gift.height
        })
        .sum::<usize>()
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
    use std::io::Cursor;

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn part1_tests() {
        init();

        assert_eq!(part1(&mut Cursor::new("2x3x4")), "58");
        assert_eq!(part1(&mut Cursor::new("1x1x10")), "43");
    }

    #[test]
    fn part2_tests() {
        init();

        assert_eq!(part2(&mut Cursor::new("2x3x4")), "34");
        assert_eq!(part2(&mut Cursor::new("1x1x10")), "14");
    }

    #[test]
    fn check_answers() {
        init();

        let f = File::open("input").unwrap();
        let mut reader = BufReader::new(f);

        assert_eq!(part1(&mut reader), "1598415");
        reader.rewind().unwrap();
        assert_eq!(part2(&mut reader), "3812909");
    }
}
