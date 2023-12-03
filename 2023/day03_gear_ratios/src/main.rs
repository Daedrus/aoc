use log::{debug, info};
use ndarray::Array2;
use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{self, BufRead, BufReader, Seek},
};

#[derive(Debug)]
struct EngineSchematic {
    numbers_and_symbols: Box<Array2<char>>,
}

impl<R> From<&mut R> for EngineSchematic
where
    R: BufRead + Seek,
{
    fn from(input: &mut R) -> Self {
        // Read the first line to get the size of the grid, then rewind
        // We assume, of course, that the input is properly formatted,
        // aka all lines are of the same size, and that the grid is square
        let grid_size = input.lines().next().unwrap().unwrap().len();
        input.rewind().unwrap();

        let numbers_and_symbols = Box::new(
            Array2::from_shape_vec(
                (grid_size, grid_size),
                input
                    .lines()
                    .flat_map(|line| line.unwrap().chars().collect::<Vec<char>>())
                    .collect(),
            )
            .unwrap(),
        );

        EngineSchematic {
            numbers_and_symbols,
        }
    }
}

impl EngineSchematic {
    fn analyze_schematic(&self) -> (u32, u32) {
        /* Given a coordinate (i, j) and an array of numbers and symbols
        find out if the element at the coordinate has any adjacent
        symbols (part1) and return a list of adjacent gears (part2) */
        fn analyze_neighbours(
            (i, j): (usize, usize),
            numbers_and_symbols: &Array2<char>,
        ) -> (bool, Vec<(usize, usize)>) {
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

            let mut adjacent_gears: Vec<(usize, usize)> = Vec::new();
            let mut has_adjacent_symbols: bool = false;

            let grid_size = numbers_and_symbols.nrows() as isize;
            neighbour_offsets.iter().for_each(|(x_offset, y_offset)| {
                let neighbour_pos_x = i as isize + x_offset;
                let neighbour_pos_y = j as isize + y_offset;
                if neighbour_pos_x >= 0
                    && neighbour_pos_x < grid_size
                    && neighbour_pos_y >= 0
                    && neighbour_pos_y < grid_size
                {
                    let neighbour =
                        numbers_and_symbols[(neighbour_pos_x as usize, neighbour_pos_y as usize)];

                    if neighbour == '*' {
                        adjacent_gears.push((neighbour_pos_x as usize, neighbour_pos_y as usize));
                    }
                    if neighbour.is_ascii_punctuation() && neighbour != '.' {
                        has_adjacent_symbols |= true;
                    }
                }
            });

            (has_adjacent_symbols, adjacent_gears)
        }

        let grid_size = self.numbers_and_symbols.nrows();

        /* Use these to keep track of the current number as we're analyzing
        each digit. They will be reset when we reach the end of a number
        (either we encounter a non digit or we reach the end of a row) */
        let mut is_part_number: bool = false;
        let mut current_number: String = Default::default();
        let mut current_number_adjacent_gears: HashSet<(usize, usize)> = HashSet::new();

        let mut sum_of_part_numbers: u32 = 0;

        /* Mapping between a gear (identified by its coordinate) and all
        the numbers it is adjacent with */
        let mut gears_to_part_numbers: HashMap<(usize, usize), Vec<u32>> = HashMap::new();

        for i in 0..grid_size {
            for j in 0..grid_size {
                /* If we found a digit then keep track of the (potential)
                larger number it is a part of and analyze its neighbours
                to see if has adjacent symbols or gears */
                if self.numbers_and_symbols[(i, j)].is_ascii_digit() {
                    let (has_adjacent_symbols, adjacent_gears) =
                        analyze_neighbours((i, j), &self.numbers_and_symbols);
                    current_number.push(self.numbers_and_symbols[(i, j)]);
                    current_number_adjacent_gears.extend(adjacent_gears.iter());
                    is_part_number |= has_adjacent_symbols;
                }

                /* If we found something that is not a digit or we've reached
                the end of the row then check if we have a part number. If
                yes, then add it to the sum and to the lists of part numbers
                associated with each gear it is a neighbour of. Then reset
                the variables we use to keep track of the current number. */
                if (!self.numbers_and_symbols[(i, j)].is_ascii_digit() || j == grid_size - 1)
                    && !current_number.is_empty()
                {
                    if is_part_number {
                        let part_number = current_number.parse::<u32>().unwrap();
                        sum_of_part_numbers += part_number;
                        current_number_adjacent_gears.iter().for_each(|gear| {
                            gears_to_part_numbers
                                .entry(*gear)
                                .and_modify(|part_numbers| part_numbers.push(part_number))
                                .or_insert_with(|| vec![part_number]);
                        });
                    }
                    is_part_number = false;
                    current_number_adjacent_gears.clear();
                    current_number.clear();
                }
            }
        }

        let sum_of_gear_ratios: u32 = gears_to_part_numbers
            .values()
            .filter(|part_numbers| part_numbers.len() == 2)
            .map(|part_numbers| part_numbers.iter().product::<u32>())
            .sum();

        (sum_of_part_numbers, sum_of_gear_ratios)
    }
}

fn part1<R>(input: &mut R) -> String
where
    R: BufRead + Seek,
{
    let grid: EngineSchematic = input.into();

    debug!("{:?}", grid);

    grid.analyze_schematic().0.to_string()
}

fn part2<R>(input: &mut R) -> String
where
    R: BufRead + Seek,
{
    let grid: EngineSchematic = input.into();

    grid.analyze_schematic().1.to_string()
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

        assert_eq!(part1(&mut reader), "527144");
        reader.rewind().unwrap();
        assert_eq!(part2(&mut reader), "81463996");
    }

    #[test]
    fn part1_tests() {
        init();

        let f = File::open("input.example").unwrap();
        let mut reader = BufReader::new(f);

        assert_eq!(part1(&mut reader), "4361");
    }

    #[test]
    fn part2_tests() {
        init();

        let f = File::open("input.example").unwrap();
        let mut reader = BufReader::new(f);

        assert_eq!(part2(&mut reader), "467835");
    }
}
