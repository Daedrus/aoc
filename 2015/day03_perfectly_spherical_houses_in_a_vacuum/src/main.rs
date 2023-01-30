use itertools::Itertools;
use log::info;
use std::{
    collections::HashSet,
    fs::File,
    hash::Hash,
    io::{self, BufRead, BufReader, Seek},
};

#[derive(Debug, Eq, Hash, PartialEq, Copy, Clone)]
struct Position {
    x: isize,
    y: isize,
}

fn part1<T: BufRead>(input: &mut T) -> String {
    let mut visited_houses: HashSet<Position> = HashSet::new();
    let mut current_position: Position = Position { x: 0, y: 0 };

    visited_houses.insert(current_position);

    input
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .chars()
        .into_iter()
        .for_each(|instruction| {
            match instruction {
                '<' => current_position.x -= 1,
                '>' => current_position.x += 1,
                'v' => current_position.y -= 1,
                '^' => current_position.y += 1,
                _ => unreachable!(),
            };

            visited_houses.insert(current_position);
        });

    visited_houses.len().to_string()
}

fn part2<T: BufRead>(input: &mut T) -> String {
    const NUMBER_OF_SANTAS: usize = 2;

    let mut visited_houses: HashSet<Position> = HashSet::new();
    let mut current_santa_positions: [Position; NUMBER_OF_SANTAS] =
        [Position { x: 0, y: 0 }; NUMBER_OF_SANTAS];

    // All Santas start at the same position
    visited_houses.insert(current_santa_positions[0]);

    input
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .chars()
        .chunks(NUMBER_OF_SANTAS)
        .into_iter()
        .for_each(|instructions| {
            instructions.enumerate().for_each(|(i, c)| {
                match c {
                    '<' => current_santa_positions[i].x -= 1,
                    '>' => current_santa_positions[i].x += 1,
                    'v' => current_santa_positions[i].y -= 1,
                    '^' => current_santa_positions[i].y += 1,
                    _ => unreachable!(),
                };

                visited_houses.insert(current_santa_positions[i]);
            })
        });

    visited_houses.len().to_string()
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

        assert_eq!(part1(&mut Cursor::new(">")), "2");
        assert_eq!(part1(&mut Cursor::new("^>v<")), "4");
        assert_eq!(part1(&mut Cursor::new("^v^v^v^v^v")), "2");
    }

    #[test]
    fn part2_tests() {
        init();

        assert_eq!(part2(&mut Cursor::new("^v")), "3");
        assert_eq!(part2(&mut Cursor::new("^>v<")), "3");
        assert_eq!(part2(&mut Cursor::new("^v^v^v^v^v")), "11");

        // Test the scenario where only the first Santa has an
        // instruction at the end (assuming there are two of them)
        assert_eq!(part2(&mut Cursor::new("^v^")), "4");
    }
}
