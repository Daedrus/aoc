use log::{debug, info};
use ndarray::Array2;
use std::{
    fs::File,
    io::{self, BufRead, BufReader}, collections::VecDeque,
};

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn get_offset(&self) -> (isize, isize) {
        match self {
            Direction::North => (-1, 0),
            Direction::South => (1, 0),
            Direction::East => (0, 1),
            Direction::West => (0, -1),
        }
    }
}

fn parse_input(input: &mut impl BufRead) -> Array2<char> {
    let lines: Vec<Vec<char>> = input
        .lines()
        .map(|line_result| line_result.unwrap().chars().collect::<Vec<char>>())
        .collect();

    Array2::from_shape_vec(
        (lines.len(), lines[0].len()),
        lines.into_iter().flatten().collect::<Vec<char>>(),
    )
    .unwrap()
}

fn walk_x_steps(garden: &Array2<char>, number_of_steps: usize) -> usize {

    let (start_position, _) = garden.indexed_iter().find(|(_, plot)| **plot == 'S').unwrap();

    let mut reached_garden_plots = 0;

    let mut plots: VecDeque<((usize, usize), usize)> = VecDeque::new();
    plots.push_front((start_position, 0));

    while let Some(((plot_x, plot_y), steps)) = plots.pop_back() {
        if steps == number_of_steps {
            reached_garden_plots += 1;
        } else {
            for direction in [Direction::North, Direction::South, Direction::East, Direction::West] {
                let neighbour_node_x = plot_x as isize + direction.get_offset().0;
                let neighbour_node_y = plot_y as isize + direction.get_offset().1;
                if neighbour_node_x < 0 || neighbour_node_x >= garden.dim().0 as isize ||
                   neighbour_node_y < 0 || neighbour_node_y >= garden.dim().1 as isize {
                    continue;
                }

                if garden[(neighbour_node_x as usize, neighbour_node_y as usize)] != '#' &&
                   !plots.contains(&((neighbour_node_x as usize, neighbour_node_y as usize), steps + 1))
                {
                    plots.push_front(((neighbour_node_x as usize, neighbour_node_y as usize), steps + 1));
                }
            }
        }
    }

    reached_garden_plots
}

fn part1(input: &mut impl BufRead, number_of_steps: usize) -> String {
    let garden = parse_input(input);

    debug!("{:?}", garden);

    walk_x_steps(&garden, number_of_steps).to_string()
}

fn main() -> io::Result<()> {
    env_logger::init();

    let f = File::open("input")?;
    let mut reader = BufReader::new(f);

    info!("Part 1 answer: {}", part1(&mut reader, 64));

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

        assert_eq!(part1(&mut reader, 64), "3574");
    }

    #[test]
    fn part1_tests() {
        init();

        let f = File::open("input.example").unwrap();
        let mut reader = BufReader::new(f);

        assert_eq!(part1(&mut reader, 6), "16");
    }
}
