use itertools::Itertools;
use log::{debug, info};
use ndarray::Array2;
use std::{cmp, i64};
use std::{
    fs::File,
    io::{self, BufRead, BufReader, Seek},
};

#[derive(Debug)]
struct Image {
    empty_space_and_galaxies: Box<Array2<char>>,
}

impl<R> From<&mut R> for Image
where
    R: BufRead + Seek,
{
    fn from(input: &mut R) -> Self {
        // Read the first line to get the size of the grid, then rewind
        // We assume, of course, that the input is properly formatted,
        // aka all lines are of the same size, and that the grid is square
        let grid_size = input.lines().next().unwrap().unwrap().len();
        input.rewind().unwrap();

        let empty_space_and_galaxies = Box::new(
            Array2::from_shape_vec(
                (grid_size, grid_size),
                input
                    .lines()
                    .flat_map(|line| line.unwrap().chars().collect::<Vec<char>>())
                    .collect(),
            )
            .unwrap(),
        );

        Image {
            empty_space_and_galaxies,
        }
    }
}

fn taxicab_distance(
    (galaxy1_row, galaxy1_col): (usize, usize),
    (galaxy2_row, galaxy2_col): (usize, usize),
    expanding_rows: &[usize],
    expanding_cols: &[usize],
    expansion_size: usize,
) -> i64 {
    let rows_between_galaxies = expanding_rows
        .iter()
        .filter(|row| {
            **row >= cmp::min(galaxy1_row, galaxy2_row)
                && **row <= cmp::max(galaxy1_row, galaxy2_row)
        })
        .count();
    let cols_between_galaxies = expanding_cols
        .iter()
        .filter(|col| {
            **col >= cmp::min(galaxy1_col, galaxy2_col)
                && **col <= cmp::max(galaxy1_col, galaxy2_col)
        })
        .count();

    let dist = (galaxy1_row as i64 - galaxy2_row as i64).abs()
        + (galaxy1_col as i64 - galaxy2_col as i64).abs()
        + (rows_between_galaxies * (expansion_size - 1)) as i64
        + (cols_between_galaxies * (expansion_size - 1)) as i64;

    debug!("{:?}", dist);

    dist
}

fn sum_of_lengths(image: &Image, expansion_size: usize) -> i64 {
    let expanding_rows = image
        .empty_space_and_galaxies
        .rows()
        .into_iter()
        .enumerate()
        .filter_map(|(row_index, row)| {
            if row.iter().all(|space| *space == '.') {
                Some(row_index)
            } else {
                None
            }
        })
        .collect::<Vec<usize>>();

    let expanding_cols = image
        .empty_space_and_galaxies
        .columns()
        .into_iter()
        .enumerate()
        .filter_map(|(col_index, col)| {
            if col.iter().all(|space| *space == '.') {
                Some(col_index)
            } else {
                None
            }
        })
        .collect::<Vec<usize>>();

    debug!("{:?}", expanding_rows);
    debug!("{:?}", expanding_cols);

    image
        .empty_space_and_galaxies
        .indexed_iter()
        .filter_map(|(coords, x)| if *x == '#' { Some(coords) } else { None })
        .combinations(2)
        .map(|galaxies| {
            taxicab_distance(
                galaxies[0],
                galaxies[1],
                &expanding_rows,
                &expanding_cols,
                expansion_size,
            )
        })
        .sum::<i64>()
}

fn part1<R>(input: &mut R) -> String
where
    R: BufRead + Seek,
{
    let image: Image = input.into();

    sum_of_lengths(&image, 2).to_string()
}

fn part2<R>(input: &mut R, expansion_size: usize) -> String
where
    R: BufRead + Seek,
{
    let image: Image = input.into();

    sum_of_lengths(&image, expansion_size).to_string()
}

fn main() -> io::Result<()> {
    env_logger::init();

    let f = File::open("input")?;
    let mut reader = BufReader::new(f);

    info!("Part 1 answer: {}", part1(&mut reader));

    reader.rewind().unwrap();

    info!("Part 2 answer: {}", part2(&mut reader, 1000000));

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

        assert_eq!(part1(&mut reader), "9509330");
        reader.rewind().unwrap();
        assert_eq!(part2(&mut reader, 1000000), "635832237682");
    }

    #[test]
    fn part1_tests() {
        init();

        let f = File::open("input.example").unwrap();
        let mut reader = BufReader::new(f);

        assert_eq!(part1(&mut reader), "374");
    }

    #[test]
    fn part2_tests() {
        init();

        let f = File::open("input.example").unwrap();
        let mut reader = BufReader::new(f);

        assert_eq!(part2(&mut reader, 10), "1030");
        reader.rewind().unwrap();
        assert_eq!(part2(&mut reader, 100), "8410");
    }
}
