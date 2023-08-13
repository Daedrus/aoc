use itertools::iproduct;
use log::info;
use std::{
    collections::HashSet,
    fs::File,
    io::{self, BufRead, BufReader, Seek},
    ops::ControlFlow,
};

enum Direction {
    North,
    South,
    East,
    West,
}

enum Turn {
    Left,
    Right,
}

#[derive(Eq, Hash, PartialEq, Copy, Clone, Debug)]
struct Location {
    x: isize,
    y: isize,
}

struct Instruction {
    turn: Turn,
    blocks: isize,
}

impl From<&str> for Instruction {
    fn from(value: &str) -> Self {
        let turn = match value.chars().next().unwrap() {
            'L' => Turn::Left,
            'R' => Turn::Right,
            _ => unreachable!(),
        };

        let blocks = value[1..].parse::<isize>().unwrap();

        Instruction { turn, blocks }
    }
}

struct Player {
    direction: Direction,
    location: Location,
    visited_locations: HashSet<Location>,
}

impl Player {
    fn turn(&mut self, turn: &Turn) {
        match self.direction {
            Direction::North => match turn {
                Turn::Left => self.direction = Direction::West,
                Turn::Right => self.direction = Direction::East,
            },
            Direction::South => match turn {
                Turn::Left => self.direction = Direction::East,
                Turn::Right => self.direction = Direction::West,
            },
            Direction::East => match turn {
                Turn::Left => self.direction = Direction::North,
                Turn::Right => self.direction = Direction::South,
            },
            Direction::West => match turn {
                Turn::Left => self.direction = Direction::South,
                Turn::Right => self.direction = Direction::North,
            },
        }
    }

    // If we're interested in the visited locations then log them and return
    // immediately if we visit the same one twice
    fn walk(&mut self, blocks: &isize, log_locations: bool) -> Option<Location> {
        let location_range_x;
        let location_range_y;

        match self.direction {
            Direction::North => {
                location_range_x = self.location.x..=self.location.x;
                location_range_y = self.location.y + 1..=self.location.y + blocks;
                self.location.y += blocks;
            }
            Direction::South => {
                location_range_x = self.location.x..=self.location.x;
                location_range_y = self.location.y - blocks..=self.location.y - 1;
                self.location.y -= blocks;
            }
            Direction::East => {
                location_range_x = self.location.x + 1..=self.location.x + blocks;
                location_range_y = self.location.y..=self.location.y;
                self.location.x += blocks;
            }
            Direction::West => {
                location_range_x = self.location.x - blocks..=self.location.x - 1;
                location_range_y = self.location.y..=self.location.y;
                self.location.x -= blocks;
            }
        }

        if log_locations {
            match iproduct!(location_range_x, location_range_y).try_for_each(|(x, y)| {
                if self.visited_locations.insert(Location { x, y }) {
                    ControlFlow::Continue(())
                } else {
                    ControlFlow::Break(Location { x, y })
                }
            }) {
                ControlFlow::Continue(_) => None,
                ControlFlow::Break(location) => Some(location),
            }
        } else {
            None
        }
    }
}

fn find_easter_bunny_hq(input: &mut impl BufRead, log_locations: bool) -> String {
    let mut player = Player {
        direction: Direction::North,
        location: Location { x: 0, y: 0 },
        visited_locations: HashSet::new(),
    };

    input.lines().take(1).for_each(|line| {
        line.unwrap()
            .split(", ")
            .map(Instruction::from)
            .try_for_each(|instruction| {
                player.turn(&instruction.turn);
                match player.walk(&instruction.blocks, log_locations) {
                    Some(location) => {
                        player.location = location;
                        ControlFlow::Break(())
                    }
                    None => ControlFlow::Continue(()),
                }
            });
    });

    (player.location.x.abs() + player.location.y.abs()).to_string()
}

fn part1(input: &mut impl BufRead) -> String {
    find_easter_bunny_hq(input, false)
}

fn part2(input: &mut impl BufRead) -> String {
    find_easter_bunny_hq(input, true)
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
    fn check_answers() {
        init();

        let f = File::open("input").unwrap();
        let mut reader = BufReader::new(f);

        assert_eq!(part1(&mut reader), "298");
        reader.rewind().unwrap();
        assert_eq!(part2(&mut reader), "158");
    }

    #[test]
    fn part1_tests() {
        init();

        assert_eq!(part1(&mut Cursor::new("R2, L3")), "5");
        assert_eq!(part1(&mut Cursor::new("R2, R2, R2")), "2");
        assert_eq!(part1(&mut Cursor::new("R5, L5, R5, R3")), "12");
    }

    #[test]
    fn part2_tests() {
        init();

        assert_eq!(part2(&mut Cursor::new("R8, R4, R4, R8")), "4");
    }
}
