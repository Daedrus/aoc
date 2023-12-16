use log::{debug, info};
use ndarray::Array2;
use std::{
    collections::VecDeque,
    fs::File,
    io::{self, BufRead, BufReader, Seek},
};

fn parse_input<R>(input: &mut R) -> Array2<char>
where
    R: BufRead + Seek,
{
    let grid_size = input.lines().next().unwrap().unwrap().len();
    input.rewind().unwrap();

    Array2::from_shape_vec(
        (grid_size, grid_size),
        input
            .lines()
            .flat_map(|line| line.unwrap().chars().collect::<Vec<char>>())
            .collect(),
    )
    .unwrap()
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl From<&Direction> for u8 {
    fn from(value: &Direction) -> Self {
        match value {
            Direction::North => 0,
            Direction::South => 1,
            Direction::East => 2,
            Direction::West => 3,
        }
    }
}

fn bounce_beam(
    grid: &Array2<char>,
    (beam_x, beam_y): (usize, usize),
    beam_direction: Direction,
) -> Array2<u8> {
    let mut visited_grid = Array2::zeros((grid.dim().0, grid.dim().1));
    let mut beam_path: VecDeque<((usize, usize), Direction)> = VecDeque::new();

    beam_path.push_back(((beam_x, beam_y), beam_direction.clone()));

    while !beam_path.is_empty() {
        let ((current_beam_x, current_beam_y), current_beam_direction) =
            beam_path.pop_front().unwrap();

        // If we have visited this node from this direction already then we
        // don't want to do it again
        if visited_grid[(current_beam_x, current_beam_y)] & (1 << u8::from(&current_beam_direction))
            != 0
        {
            continue;
        } else {
            visited_grid[(current_beam_x, current_beam_y)] |=
                1 << u8::from(&current_beam_direction);
        }

        debug!(
            "{} {} {:?}",
            current_beam_x, current_beam_y, current_beam_direction
        );

        match grid[(current_beam_x, current_beam_y)] {
            '.' => {
                let (next_beam_x, next_beam_y) = match current_beam_direction {
                    Direction::North if current_beam_x > 0 => (current_beam_x - 1, current_beam_y),
                    Direction::South if current_beam_x < grid.dim().0 - 1 => {
                        (current_beam_x + 1, current_beam_y)
                    }
                    Direction::East if current_beam_y < grid.dim().1 - 1 => {
                        (current_beam_x, current_beam_y + 1)
                    }
                    Direction::West if current_beam_y > 0 => (current_beam_x, current_beam_y - 1),
                    _ => continue,
                };

                beam_path.push_back(((next_beam_x, next_beam_y), current_beam_direction));
            }
            '/' => {
                let (next_beam_x, next_beam_y, next_beam_direction) = match current_beam_direction {
                    Direction::North if current_beam_y < grid.dim().1 - 1 => {
                        (current_beam_x, current_beam_y + 1, Direction::East)
                    }
                    Direction::South if current_beam_y > 0 => {
                        (current_beam_x, current_beam_y - 1, Direction::West)
                    }
                    Direction::East if current_beam_x > 0 => {
                        (current_beam_x - 1, current_beam_y, Direction::North)
                    }
                    Direction::West if current_beam_x < grid.dim().0 - 1 => {
                        (current_beam_x + 1, current_beam_y, Direction::South)
                    }
                    _ => continue,
                };

                beam_path.push_back(((next_beam_x, next_beam_y), next_beam_direction));
            }
            '\\' => {
                let (next_beam_x, next_beam_y, next_beam_direction) = match current_beam_direction {
                    Direction::North if current_beam_y > 0 => {
                        (current_beam_x, current_beam_y - 1, Direction::West)
                    }
                    Direction::South if current_beam_y < grid.dim().1 - 1 => {
                        (current_beam_x, current_beam_y + 1, Direction::East)
                    }
                    Direction::East if current_beam_x < grid.dim().0 - 1 => {
                        (current_beam_x + 1, current_beam_y, Direction::South)
                    }
                    Direction::West if current_beam_x > 0 => {
                        (current_beam_x - 1, current_beam_y, Direction::North)
                    }
                    _ => continue,
                };

                beam_path.push_back(((next_beam_x, next_beam_y), next_beam_direction));
            }
            '-' => {
                match current_beam_direction {
                    Direction::North | Direction::South => {
                        if current_beam_y < grid.dim().1 - 1 {
                            beam_path
                                .push_back(((current_beam_x, current_beam_y + 1), Direction::East));
                        }
                        if current_beam_y > 0 {
                            beam_path
                                .push_back(((current_beam_x, current_beam_y - 1), Direction::West));
                        }
                    }
                    Direction::East if current_beam_y < grid.dim().1 - 1 => {
                        beam_path
                            .push_back(((current_beam_x, current_beam_y + 1), Direction::East));
                    }
                    Direction::West if current_beam_y > 0 => {
                        beam_path
                            .push_back(((current_beam_x, current_beam_y - 1), Direction::West));
                    }
                    _ => continue,
                };
            }
            '|' => {
                match current_beam_direction {
                    Direction::North if current_beam_x > 0 => {
                        beam_path
                            .push_back(((current_beam_x - 1, current_beam_y), Direction::North));
                    }
                    Direction::South if current_beam_x < grid.dim().0 - 1 => {
                        beam_path
                            .push_back(((current_beam_x + 1, current_beam_y), Direction::South));
                    }
                    Direction::East | Direction::West => {
                        if current_beam_x > 0 {
                            beam_path.push_back((
                                (current_beam_x - 1, current_beam_y),
                                Direction::North,
                            ));
                        }
                        if current_beam_x < grid.dim().0 - 1 {
                            beam_path.push_back((
                                (current_beam_x + 1, current_beam_y),
                                Direction::South,
                            ));
                        }
                    }
                    _ => continue,
                };
            }
            _ => unreachable!(),
        }
    }

    visited_grid
}

fn part1<R>(input: &mut R) -> String
where
    R: BufRead + Seek,
{
    let grid = parse_input(input);

    debug!("{:?}", grid);

    bounce_beam(&grid, (0, 0), Direction::East)
        .iter()
        .filter(|visited| **visited != 0)
        .count()
        .to_string()
}

fn part2<R>(input: &mut R) -> String
where
    R: BufRead + Seek,
{
    let grid = parse_input(input);

    debug!("{:?}", grid);

    (0..grid.dim().1)
        .map(|y| {
            bounce_beam(&grid, (0, y), Direction::South)
                .iter()
                .filter(|visited| **visited != 0)
                .count()
        })
        .chain((0..grid.dim().1).map(|y| {
            bounce_beam(&grid, (grid.dim().0 - 1, y), Direction::North)
                .iter()
                .filter(|visited| **visited != 0)
                .count()
        }))
        .chain((0..grid.dim().0).map(|x| {
            bounce_beam(&grid, (x, 0), Direction::East)
                .iter()
                .filter(|visited| **visited != 0)
                .count()
        }))
        .chain((0..grid.dim().0).map(|x| {
            bounce_beam(&grid, (x, grid.dim().1 - 1), Direction::West)
                .iter()
                .filter(|visited| **visited != 0)
                .count()
        }))
        .max()
        .unwrap()
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

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn check_answers() {
        init();

        let f = File::open("input").unwrap();
        let mut reader = BufReader::new(f);

        assert_eq!(part1(&mut reader), "8034");
        reader.rewind().unwrap();
        assert_eq!(part2(&mut reader), "8225");
    }

    #[test]
    fn part1_tests() {
        init();

        let f = File::open("input.example").unwrap();
        let mut reader = BufReader::new(f);

        assert_eq!(part1(&mut reader), "46");
    }

    #[test]
    fn part2_tests() {
        init();

        let f = File::open("input.example").unwrap();
        let mut reader = BufReader::new(f);

        assert_eq!(part2(&mut reader), "51");
    }
}
