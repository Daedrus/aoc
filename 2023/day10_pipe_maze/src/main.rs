use log::debug;
use log::info;
use ndarray::Array2;
use std::collections::VecDeque;
use std::{
    fs::File,
    io::{self, BufRead, BufReader, Seek},
};

#[derive(Debug)]
struct Tile {
    kind: char,
    visited: bool,
}

#[derive(Debug)]
struct Tiles {
    pipes_and_ground: Box<Array2<Tile>>,
}

impl<R> From<&mut R> for Tiles
where
    R: BufRead + Seek,
{
    fn from(input: &mut R) -> Self {
        let grid_size_y = input.lines().next().unwrap().unwrap().len();
        let grid_size_x = input.lines().count() + 1;
        input.rewind().unwrap();

        let pipes_and_ground = Box::new(
            Array2::from_shape_vec(
                (grid_size_x, grid_size_y),
                input
                    .lines()
                    .flat_map(|line| {
                        line.unwrap()
                            .chars()
                            .map(|c| Tile {
                                kind: c,
                                visited: false,
                            })
                            .collect::<Vec<Tile>>()
                    })
                    .collect(),
            )
            .unwrap(),
        );

        Tiles { pipes_and_ground }
    }
}

fn get_valid_neighbours(
    (position_x, position_y): (usize, usize),
    tiles: &mut Tiles,
) -> Vec<(usize, usize)> {
    fn check_north_neighbour((position_x, position_y): (usize, usize), tiles: &Tiles) -> bool {
        position_x > 0
            && ['7', 'F', '|'].contains(&tiles.pipes_and_ground[(position_x - 1, position_y)].kind)
    }

    fn check_south_neighbour((position_x, position_y): (usize, usize), tiles: &Tiles) -> bool {
        position_x < tiles.pipes_and_ground.nrows()
            && ['L', 'J', '|'].contains(&tiles.pipes_and_ground[(position_x + 1, position_y)].kind)
    }

    fn check_west_neighbour((position_x, position_y): (usize, usize), tiles: &Tiles) -> bool {
        position_y > 0
            && ['L', 'F', '-'].contains(&tiles.pipes_and_ground[(position_x, position_y - 1)].kind)
    }

    fn check_east_neighbour((position_x, position_y): (usize, usize), tiles: &Tiles) -> bool {
        position_y < tiles.pipes_and_ground.ncols()
            && ['7', 'J', '-'].contains(&tiles.pipes_and_ground[(position_x, position_y + 1)].kind)
    }

    let mut valid_neighbours: Vec<(usize, usize)> = Vec::new();

    match tiles.pipes_and_ground[(position_x, position_y)].kind {
        '|' => {
            if check_north_neighbour((position_x, position_y), tiles) {
                valid_neighbours.push((position_x - 1, position_y));
            }
            if check_south_neighbour((position_x, position_y), tiles) {
                valid_neighbours.push((position_x + 1, position_y));
            }
        }
        '-' => {
            if check_west_neighbour((position_x, position_y), tiles) {
                valid_neighbours.push((position_x, position_y - 1));
            }
            if check_east_neighbour((position_x, position_y), tiles) {
                valid_neighbours.push((position_x, position_y + 1));
            }
        }
        'L' => {
            if check_north_neighbour((position_x, position_y), tiles) {
                valid_neighbours.push((position_x - 1, position_y));
            }
            if check_east_neighbour((position_x, position_y), tiles) {
                valid_neighbours.push((position_x, position_y + 1));
            }
        }
        'J' => {
            if check_north_neighbour((position_x, position_y), tiles) {
                valid_neighbours.push((position_x - 1, position_y));
            }
            if check_west_neighbour((position_x, position_y), tiles) {
                valid_neighbours.push((position_x, position_y - 1));
            }
        }
        '7' => {
            if check_west_neighbour((position_x, position_y), tiles) {
                valid_neighbours.push((position_x, position_y - 1));
            }
            if check_south_neighbour((position_x, position_y), tiles) {
                valid_neighbours.push((position_x + 1, position_y));
            }
        }
        'F' => {
            if check_east_neighbour((position_x, position_y), tiles) {
                valid_neighbours.push((position_x, position_y + 1));
            }
            if check_south_neighbour((position_x, position_y), tiles) {
                valid_neighbours.push((position_x + 1, position_y));
            }
        }
        'S' => {
            let mut has_north_neighbour: bool = false;
            let mut has_south_neighbour: bool = false;
            let mut has_east_neighbour: bool = false;
            let mut has_west_neighbour: bool = false;

            if check_north_neighbour((position_x, position_y), tiles) {
                valid_neighbours.push((position_x - 1, position_y));
                has_north_neighbour = true;
            }
            if check_south_neighbour((position_x, position_y), tiles) {
                valid_neighbours.push((position_x + 1, position_y));
                has_south_neighbour = true;
            }
            if check_west_neighbour((position_x, position_y), tiles) {
                valid_neighbours.push((position_x, position_y - 1));
                has_west_neighbour = true;
            }
            if check_east_neighbour((position_x, position_y), tiles) {
                valid_neighbours.push((position_x, position_y + 1));
                has_east_neighbour = true;
            }

            // Replace 'S' with the actual pipe symbol while we're here,
            // so that we don't have to do any special handling for it in part 2
            tiles.pipes_and_ground[(position_x, position_y)].kind = match (
                has_north_neighbour,
                has_south_neighbour,
                has_east_neighbour,
                has_west_neighbour,
            ) {
                (true, true, false, false) => '|',
                (false, false, true, true) => '-',
                (true, false, true, false) => 'L',
                (true, false, false, true) => 'J',
                (false, true, false, true) => '7',
                (false, true, true, false) => 'F',
                _ => unreachable!(),
            };
        }
        _ => unreachable!(),
    };

    valid_neighbours
}

fn find_steps_to_farthest_point(tiles: &mut Tiles) -> (usize, Vec<(usize, usize)>) {
    let (starting_position, _) = tiles
        .pipes_and_ground
        .indexed_iter()
        .find(|((_, _), tile)| tile.kind == 'S')
        .unwrap();

    let mut pipe_loop: Vec<(usize, usize)> = vec![(starting_position)];
    let mut tiles_to_visit: VecDeque<((usize, usize), usize)> =
        VecDeque::from([(starting_position, 0)]);
    let mut max_steps: usize = usize::MIN;

    // Regular BFS
    loop {
        debug!("{:?}", tiles_to_visit);

        let (current_tile_pos, current_tile_steps) = tiles_to_visit.pop_front().unwrap();
        let neighbours = get_valid_neighbours(current_tile_pos, tiles);

        if max_steps < current_tile_steps {
            max_steps = current_tile_steps;
        }

        debug!("{:?}", neighbours);

        neighbours.iter().for_each(|neighbour_pos| {
            if !tiles.pipes_and_ground[*neighbour_pos].visited {
                tiles.pipes_and_ground[*neighbour_pos].visited = true;
                tiles_to_visit.push_back((*neighbour_pos, current_tile_steps + 1));
                pipe_loop.push(*neighbour_pos);
            }
        });

        if tiles_to_visit.is_empty() {
            break;
        }
    }

    (max_steps, pipe_loop)
}

fn part1<R>(input: &mut R) -> String
where
    R: BufRead + Seek,
{
    let mut tiles: Tiles = input.into();

    debug!("{:?}", tiles);

    let (steps, _) = find_steps_to_farthest_point(&mut tiles);

    steps.to_string()
}

fn part2<R>(input: &mut R) -> String
where
    R: BufRead + Seek,
{
    let mut tiles: Tiles = input.into();

    debug!("{:?}", tiles);

    let (_, pipe_loop) = find_steps_to_farthest_point(&mut tiles);

    // We'll reuse the visited field to mark the tiles outside of the loop
    // so reset it here since it's been used in find_steps_to_farthest_point
    for tile in tiles.pipes_and_ground.iter_mut() {
        tile.visited = false;
    }

    debug!("{:?}", pipe_loop);

    // Ray casting algorithm
    for i in 0..tiles.pipes_and_ground.nrows() {
        let mut number_of_intersections = 0;
        let mut line_start: char = ' ';

        for j in 0..tiles.pipes_and_ground.ncols() {
            if pipe_loop.contains(&(i, j)) {
                // The following are considered polygon edges
                // |
                // L7, L-7, L--7, L---7, ...
                // FJ, F-J, F--J, F---J, ...
                //
                // Anything else is not an edge
                match tiles.pipes_and_ground[(i, j)].kind {
                    '|' => {
                        number_of_intersections += 1;
                    }
                    '-' => {}
                    'L' => {
                        line_start = 'L';
                    }
                    'J' => {
                        if line_start == 'F' {
                            number_of_intersections += 1;
                            line_start = ' ';
                        }
                    }
                    '7' => {
                        if line_start == 'L' {
                            number_of_intersections += 1;
                            line_start = ' ';
                        }
                    }
                    'F' => {
                        line_start = 'F';
                    }
                    _ => unreachable!(),
                }
            } else if number_of_intersections % 2 != 0 {
                tiles.pipes_and_ground[(i, j)].visited = true;
            }
        }
    }

    tiles
        .pipes_and_ground
        .iter()
        .filter(|tile| tile.visited)
        .count()
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

        assert_eq!(part1(&mut reader), "7102");
        reader.rewind().unwrap();
        assert_eq!(part2(&mut reader), "363");
    }

    #[test]
    fn part1_tests() {
        init();

        let f = File::open("input.example1").unwrap();
        let mut reader = BufReader::new(f);

        assert_eq!(part1(&mut reader), "4");

        let f = File::open("input.example2").unwrap();
        let mut reader = BufReader::new(f);

        assert_eq!(part1(&mut reader), "8");
    }

    #[test]
    fn part2_tests() {
        init();

        let f = File::open("input.example3").unwrap();
        let mut reader = BufReader::new(f);

        assert_eq!(part2(&mut reader), "4");

        let f = File::open("input.example4").unwrap();
        let mut reader = BufReader::new(f);

        assert_eq!(part2(&mut reader), "8");

        let f = File::open("input.example5").unwrap();
        let mut reader = BufReader::new(f);

        assert_eq!(part2(&mut reader), "10");
    }
}
