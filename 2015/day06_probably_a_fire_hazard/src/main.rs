use log::{debug, info};
use std::{
    fs::File,
    io::{self, BufRead, BufReader, Seek},
    ops::Not,
};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum LightState {
    On,
    Off,
}

impl Not for LightState {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            LightState::Off => LightState::On,
            LightState::On => LightState::Off,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum LightAction {
    TurnOn,
    TurnOff,
    Toggle,
}

impl From<&str> for LightAction {
    fn from(input: &str) -> Self {
        match input {
            "turn on" => LightAction::TurnOn,
            "turn off" => LightAction::TurnOff,
            "toggle" => LightAction::Toggle,
            _ => unimplemented!(""),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct Instruction {
    light_action: LightAction,
    coord_pair1: (usize, usize),
    coord_pair2: (usize, usize),
}

impl From<&str> for Instruction {
    fn from(input: &str) -> Self {
        use nom::{
            branch::alt,
            bytes::complete::tag,
            character::complete,
            sequence::{preceded, separated_pair, terminated, tuple},
            IResult,
        };

        fn light_action(input: &str) -> IResult<&str, &str> {
            alt((tag("turn on"), tag("toggle"), tag("turn off")))(input)
        }

        fn range(input: &str) -> IResult<&str, (u32, u32)> {
            separated_pair(
                preceded(complete::multispace0, complete::u32),
                tag(","),
                terminated(complete::u32, complete::multispace0),
            )(input)
        }

        let (_, (action, (x0, x1), _, (y0, y1))) =
            tuple((light_action, range, tag("through"), range))(input).unwrap();

        debug!("{:?}", action);
        debug!("{} {}", x0, x1);
        debug!("{} {}", y0, y1);

        Instruction {
            light_action: action.into(),
            coord_pair1: (x0 as usize, x1 as usize),
            coord_pair2: (y0 as usize, y1 as usize),
        }
    }
}

fn part1<T: BufRead>(input: &mut T) -> usize {
    const GRID_SIZE: usize = 1000;
    let mut grid: [[LightState; GRID_SIZE]; GRID_SIZE] = [[LightState::Off; GRID_SIZE]; GRID_SIZE];

    input.lines().for_each(|line| {
        let instruction: Instruction = line.unwrap().as_str().into();

        #[allow(clippy::needless_range_loop)]
        for i in (instruction.coord_pair1.0)..=(instruction.coord_pair2.0) {
            for j in (instruction.coord_pair1.1)..=(instruction.coord_pair2.1) {
                match instruction.light_action {
                    LightAction::TurnOn => grid[i][j] = LightState::On,
                    LightAction::TurnOff => grid[i][j] = LightState::Off,
                    LightAction::Toggle => grid[i][j] = !grid[i][j],
                }
            }
        }
    });

    grid.iter()
        .flat_map(|r| r.iter())
        .filter(|&&l| l == LightState::On)
        .count()
}

fn part2<T: BufRead>(input: &mut T) -> usize {
    const GRID_SIZE: usize = 1000;
    // We use Vec here since an array of usize on the stack would overflow it
    let mut grid: Vec<Vec<usize>> = vec![vec![0; GRID_SIZE]; GRID_SIZE];

    input.lines().for_each(|line| {
        let instruction: Instruction = line.unwrap().as_str().into();

        #[allow(clippy::needless_range_loop)]
        for i in (instruction.coord_pair1.0)..=(instruction.coord_pair2.0) {
            for j in (instruction.coord_pair1.1)..=(instruction.coord_pair2.1) {
                match instruction.light_action {
                    LightAction::TurnOn => grid[i][j] += 1,
                    LightAction::TurnOff => {
                        if grid[i][j] > 0 {
                            grid[i][j] -= 1
                        }
                    }
                    LightAction::Toggle => grid[i][j] += 2,
                }
            }
        }
    });

    grid.iter().fold(0, |sum, r| sum + r.iter().sum::<usize>())
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
    fn parser_tests() {
        init();

        assert_eq!(
            Instruction::from("turn on 0,0 through 999,999"),
            Instruction {
                light_action: LightAction::TurnOn,
                coord_pair1: (0, 0),
                coord_pair2: (999, 999),
            }
        );

        assert_eq!(
            Instruction::from("toggle 0,0 through 999,0"),
            Instruction {
                light_action: LightAction::Toggle,
                coord_pair1: (0, 0),
                coord_pair2: (999, 0),
            }
        );

        assert_eq!(
            Instruction::from("turn off 499,499 through 500,500"),
            Instruction {
                light_action: LightAction::TurnOff,
                coord_pair1: (499, 499),
                coord_pair2: (500, 500),
            }
        );
    }

    #[test]
    fn part1_tests() {
        init();

        assert_eq!(part1(&mut Cursor::new("turn on 0,0 through 0,9")), 10);
        assert_eq!(part1(&mut Cursor::new("toggle 0,0 through 0,19")), 20);
        assert_eq!(part1(&mut Cursor::new("turn off 0,0 through 0,19")), 0);
    }

    #[test]
    fn part2_tests() {
        init();

        assert_eq!(part2(&mut Cursor::new("turn on 0,0 through 0,9")), 10);
        assert_eq!(part2(&mut Cursor::new("toggle 0,0 through 0,19")), 40);
        assert_eq!(part2(&mut Cursor::new("turn off 0,0 through 0,19")), 0);
    }
}
