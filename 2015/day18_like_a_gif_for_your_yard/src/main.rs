use log::info;
use ndarray::Array2;
use std::{
    fs::File,
    io::{self, BufRead, BufReader, Seek},
};

#[derive(Debug, Clone, PartialEq, Eq)]
enum LightState {
    On,
    Off,
}

impl From<char> for LightState {
    fn from(value: char) -> Self {
        match value {
            '#' => LightState::On,
            '.' => LightState::Off,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
struct Grid {
    lights: Box<Array2<LightState>>,
    old_lights: Box<Array2<LightState>>,
}

impl Grid {
    fn step(&mut self) {
        fn count_neighbours_on((i, j): (usize, usize), old_lights: &Array2<LightState>) -> usize {
            let neighbour_offsets: [(isize, isize); 8] = [
                (-1, -1),
                (-1, 0),
                (-1, 1),
                (0, -1),
                (0, 1),
                (1, -1),
                (1, 0),
                (1, 1),
            ];

            let grid_size = old_lights.nrows() as isize;
            neighbour_offsets
                .iter()
                .filter(|(x_offset, y_offset)| {
                    let neighbour_pos_x = i as isize + x_offset;
                    let neighbour_pos_y = j as isize + y_offset;
                    if neighbour_pos_x >= 0
                        && neighbour_pos_x < grid_size
                        && neighbour_pos_y >= 0
                        && neighbour_pos_y < grid_size
                    {
                        old_lights[(neighbour_pos_x as usize, neighbour_pos_y as usize)]
                            == LightState::On
                    } else {
                        false
                    }
                })
                .count()
        }

        fn handle_light(
            (i, j): (usize, usize),
            lights: &mut Array2<LightState>,
            old_lights: &Array2<LightState>,
        ) {
            lights[(i, j)] = match old_lights[(i, j)] {
                LightState::On => match count_neighbours_on((i, j), old_lights) {
                    2 | 3 => LightState::On,
                    _ => LightState::Off,
                },
                LightState::Off => match count_neighbours_on((i, j), old_lights) {
                    3 => LightState::On,
                    _ => LightState::Off,
                },
            }
        }

        let grid_size = self.lights.nrows();

        std::mem::swap(&mut self.lights, &mut self.old_lights);

        for i in 0..grid_size {
            for j in 0..grid_size {
                handle_light((i, j), &mut self.lights, &self.old_lights);
            }
        }
    }

    fn light_corners(&mut self) {
        let grid_size = self.lights.nrows();

        for corner in [
            (0, 0),
            (0, grid_size - 1),
            (grid_size - 1, 0),
            (grid_size - 1, grid_size - 1),
        ] {
            self.lights[corner] = LightState::On;
        }
    }

    fn count_lights_on(&self) -> usize {
        self.lights
            .iter()
            .filter(|light_state| **light_state == LightState::On)
            .count()
    }
}

impl<R> From<&mut R> for Grid
where
    R: BufRead + Seek,
{
    fn from(input: &mut R) -> Self {
        // Read the first line to get the size of the grid, then rewind
        // We assume, of course, that the input is properly formatted,
        // aka all lines are of the same size
        let grid_size = input.lines().next().unwrap().unwrap().len();
        input.rewind().unwrap();

        let lights = Box::new(
            Array2::from_shape_vec(
                (grid_size, grid_size),
                input
                    .lines()
                    .flat_map(|line| {
                        line.unwrap()
                            .chars()
                            .map(|c| c.into())
                            .collect::<Vec<LightState>>()
                    })
                    .collect(),
            )
            .unwrap(),
        );
        let old_lights = lights.clone();

        Grid { lights, old_lights }
    }
}

fn part1<R>(input: &mut R, steps: usize) -> String
where
    R: BufRead + Seek,
{
    let mut grid: Grid = input.into();

    for _ in 1..=steps {
        grid.step();
    }

    grid.count_lights_on().to_string()
}

fn part2<R>(input: &mut R, steps: usize) -> String
where
    R: BufRead + Seek,
{
    let mut grid: Grid = input.into();

    grid.light_corners();
    for _ in 1..=steps {
        grid.step();
        grid.light_corners();
    }

    grid.count_lights_on().to_string()
}

fn main() -> io::Result<()> {
    env_logger::init();

    let f = File::open("input")?;
    let mut reader = BufReader::new(f);

    info!("Part 1 answer: {}", part1(&mut reader, 100));

    reader.rewind().unwrap();

    info!("Part 2 answer: {}", part2(&mut reader, 100));

    Ok(())
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

        let f = File::open("input.example").unwrap();
        let mut reader = BufReader::new(f);

        assert_eq!(part1(&mut reader, 4), "4");
    }

    #[test]
    fn part2_tests() {
        init();

        let f = File::open("input.example").unwrap();
        let mut reader = BufReader::new(f);

        assert_eq!(part2(&mut reader, 5), "17");
    }

    #[test]
    fn check_answers() {
        init();

        let f = File::open("input").unwrap();
        let mut reader = BufReader::new(f);

        assert_eq!(part1(&mut reader, 100), "814");
        reader.rewind().unwrap();
        assert_eq!(part2(&mut reader, 100), "924");
    }
}
