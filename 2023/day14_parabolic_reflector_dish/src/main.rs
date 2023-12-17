use log::{debug, info};
use ndarray::Array2;
use std::{
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

#[derive(Hash, PartialEq, PartialOrd, Ord, Eq, Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}

fn tilt_in_direction(
    direction: Direction,
    platform: &Array2<char>,
    platform_after_tilt: &mut Array2<char>,
) {
    // We use the usize::MAX value as a marker that the variable
    // has not been set (this works since the input indexes are
    // not that large).
    let mut rock = usize::MAX;

    match direction {
        Direction::North => {
            for col in 0..platform.dim().1 {
                for row in 0..platform.dim().0 {
                    match platform[(row, col)] {
                        '#' => {
                            rock = row;
                        }
                        'O' => {
                            if rock == usize::MAX {
                                platform_after_tilt[(0, col)] = 'O';
                                rock = 0;
                            } else {
                                platform_after_tilt[(rock + 1, col)] = 'O';
                                rock += 1;
                            }
                        }
                        '.' => {}
                        _ => unreachable!(),
                    };
                }

                rock = usize::MAX;
            }
        }
        Direction::South => {
            for col in 0..platform.dim().1 {
                for row in (0..platform.dim().0).rev() {
                    match platform[(row, col)] {
                        '#' => {
                            rock = row;
                        }
                        'O' => {
                            if rock == usize::MAX {
                                platform_after_tilt[(platform.dim().0 - 1, col)] = 'O';
                                rock = platform.dim().0 - 1;
                            } else {
                                platform_after_tilt[(rock - 1, col)] = 'O';
                                rock -= 1;
                            }
                        }
                        '.' => {}
                        _ => unreachable!(),
                    };
                }

                rock = usize::MAX;
            }
        }
        Direction::West => {
            for row in 0..platform.dim().0 {
                for col in 0..platform.dim().1 {
                    match platform[(row, col)] {
                        '#' => {
                            rock = col;
                        }
                        'O' => {
                            if rock == usize::MAX {
                                platform_after_tilt[(row, 0)] = 'O';
                                rock = 0;
                            } else {
                                platform_after_tilt[(row, rock + 1)] = 'O';
                                rock += 1;
                            }
                        }
                        '.' => {}
                        _ => unreachable!(),
                    };
                }

                rock = usize::MAX;
            }
        }
        Direction::East => {
            for row in 0..platform.dim().0 {
                for col in (0..platform.dim().1).rev() {
                    match platform[(row, col)] {
                        '#' => {
                            rock = col;
                        }
                        'O' => {
                            if rock == usize::MAX {
                                platform_after_tilt[(row, platform.dim().1 - 1)] = 'O';
                                rock = platform.dim().1 - 1;
                            } else {
                                platform_after_tilt[(row, rock - 1)] = 'O';
                                rock -= 1;
                            }
                        }
                        '.' => {}
                        _ => unreachable!(),
                    };
                }

                rock = usize::MAX;
            }
        }
    }
}

fn part1<R>(input: &mut R) -> String
where
    R: BufRead + Seek,
{
    let platform = parse_input(input);

    let mut platform_after_tilt = Array2::from_shape_fn(platform.raw_dim(), |(i, j)| {
        if platform[(i, j)] == '#' {
            '#'
        } else {
            '.'
        }
    });

    tilt_in_direction(Direction::North, &platform, &mut platform_after_tilt);

    platform_after_tilt
        .indexed_iter()
        .map(|((i, _), c)| {
            if *c == 'O' {
                platform_after_tilt.dim().0 - i
            } else {
                0
            }
        })
        .sum::<usize>()
        .to_string()
}

fn part2<R>(input: &mut R) -> String
where
    R: BufRead + Seek,
{
    let platform = parse_input(input);

    let iterations = 1_000_000_000;

    let mut platform_after_tilt_north = Array2::from_shape_fn(platform.raw_dim(), |(i, j)| {
        if platform[(i, j)] == '#' {
            '#'
        } else {
            '.'
        }
    });

    let mut platform_after_tilt_west = Array2::from_shape_fn(platform.raw_dim(), |(i, j)| {
        if platform[(i, j)] == '#' {
            '#'
        } else {
            '.'
        }
    });

    let mut platform_after_tilt_south = Array2::from_shape_fn(platform.raw_dim(), |(i, j)| {
        if platform[(i, j)] == '#' {
            '#'
        } else {
            '.'
        }
    });

    let mut platform_after_tilt_east = platform.clone();
    let mut cycle_length = 0;
    let mut cycle_start = 0;

    let mut state_hash: Vec<(Array2<char>, Direction)> = Vec::new();

    for n in 1..=iterations {
        tilt_in_direction(
            Direction::North,
            &platform_after_tilt_east,
            &mut platform_after_tilt_north,
        );

        tilt_in_direction(
            Direction::West,
            &platform_after_tilt_north,
            &mut platform_after_tilt_west,
        );
        tilt_in_direction(
            Direction::South,
            &platform_after_tilt_west,
            &mut platform_after_tilt_south,
        );

        for row in 0..platform.dim().0 {
            for col in 0..platform.dim().1 {
                if platform_after_tilt_east[(row, col)] == 'O' {
                    platform_after_tilt_east[(row, col)] = '.';
                }
            }
        }

        tilt_in_direction(
            Direction::East,
            &platform_after_tilt_south,
            &mut platform_after_tilt_east,
        );

        for row in 0..platform.dim().0 {
            for col in 0..platform.dim().1 {
                if platform_after_tilt_north[(row, col)] == 'O' {
                    platform_after_tilt_north[(row, col)] = '.';
                }
                if platform_after_tilt_west[(row, col)] == 'O' {
                    platform_after_tilt_west[(row, col)] = '.';
                }
                if platform_after_tilt_south[(row, col)] == 'O' {
                    platform_after_tilt_south[(row, col)] = '.';
                }
            }
        }

        if state_hash.contains(&(platform_after_tilt_east.clone(), Direction::North)) {
            cycle_start = state_hash
                .iter()
                .position(|elem| elem == &(platform_after_tilt_east.clone(), Direction::North))
                .unwrap();
            cycle_length = n - cycle_start - 1;
            break;
        } else {
            state_hash.push((platform_after_tilt_east.clone(), Direction::North));
        }
    }

    let final_cycle = (iterations - cycle_start) % cycle_length + cycle_start - 1;

    debug!("cycle start {}", cycle_start);
    debug!("cycle length {}", cycle_length);
    debug!("final cycle {}", final_cycle);

    state_hash
        .get(final_cycle)
        .unwrap()
        .0
        .indexed_iter()
        .map(|((i, _), c)| {
            if *c == 'O' {
                platform_after_tilt_east.dim().0 - i
            } else {
                0
            }
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

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn check_answers() {
        init();

        let f = File::open("input").unwrap();
        let mut reader = BufReader::new(f);

        assert_eq!(part1(&mut reader), "108641");
        reader.rewind().unwrap();
        assert_eq!(part2(&mut reader), "84328");
    }

    #[test]
    fn part1_tests() {
        init();

        let f = File::open("input.example").unwrap();
        let mut reader = BufReader::new(f);

        assert_eq!(part1(&mut reader), "136");
    }

    #[test]
    fn part2_tests() {
        init();

        let f = File::open("input.example").unwrap();
        let mut reader = BufReader::new(f);

        assert_eq!(part2(&mut reader), "64");
    }
}
