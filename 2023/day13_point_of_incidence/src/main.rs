use log::{debug, info};
use ndarray::{Array2, ArrayView1};
use std::ops::Range;
use std::{
    fs::File,
    io::{self, BufRead, BufReader, Seek},
};

fn parse_input(input: &mut impl BufRead) -> Vec<Array2<char>> {
    let mut current_mirror_array = String::new();
    let mut current_mirror_cols = 0;
    let mut current_mirror_rows = 0;
    let mut mirror_arrays: Vec<Array2<char>> = Vec::new();

    for line in input.lines() {
        let line = line.unwrap();

        if !line.is_empty() {
            current_mirror_rows += 1;
            current_mirror_cols = line.len();
            current_mirror_array.push_str(&line);
        } else {
            mirror_arrays.push(
                Array2::from_shape_vec(
                    (current_mirror_rows, current_mirror_cols),
                    current_mirror_array.chars().collect::<Vec<char>>(),
                )
                .unwrap(),
            );
            current_mirror_rows = 0;
            current_mirror_cols = 0;
            current_mirror_array.clear();
        }
    }

    mirror_arrays.push(
        Array2::from_shape_vec(
            (current_mirror_rows, current_mirror_cols),
            current_mirror_array.chars().collect::<Vec<char>>(),
        )
        .unwrap(),
    );

    mirror_arrays
}

#[derive(Debug, Copy, Clone)]
enum ReflectionType {
    Row,
    Column,
    None,
}

// Given a mirror array and a reflection criteria, find the reflection line
// (this can be either a row or a column, or none)
//
// is_reflection is a function that takes two ranges of either rows or columns
// and checks if they are reflected
fn find_reflection_line(
    mirror_array: &Array2<char>,
    is_reflection: impl Fn(&Array2<char>, Range<usize>, Range<usize>, ReflectionType) -> bool,
) -> (ReflectionType, usize) {
    let number_of_rows = mirror_array.dim().0;
    let number_of_cols = mirror_array.dim().1;

    if let Some(horizontal_line) = (1..number_of_rows).find(|horizontal_line| {
        let rows_above = 0..*horizontal_line;
        let rows_below = *horizontal_line..number_of_rows;

        debug!("{:?} {:?}", rows_above, rows_below);

        is_reflection(mirror_array, rows_above, rows_below, ReflectionType::Row)
    }) {
        return (ReflectionType::Row, horizontal_line);
    }

    if let Some(vertical_line) = (1..number_of_cols).find(|vertical_line| {
        let columns_to_the_left = 0..*vertical_line;
        let columns_to_the_right = *vertical_line..number_of_cols;

        debug!("{:?} {:?}", columns_to_the_left, columns_to_the_right);

        is_reflection(
            mirror_array,
            columns_to_the_left,
            columns_to_the_right,
            ReflectionType::Column,
        )
    }) {
        return (ReflectionType::Column, vertical_line);
    }

    (ReflectionType::None, 0)
}

fn get_array_view(
    mirror_array: &Array2<char>,
    index: usize,
    reflection_type: ReflectionType,
) -> ArrayView1<'_, char> {
    match reflection_type {
        ReflectionType::Row => mirror_array.row(index),
        ReflectionType::Column => mirror_array.column(index),
        ReflectionType::None => unreachable!(),
    }
}

fn get_summary(
    mirror_arrays: &[Array2<char>],
    reflection_criteria: impl Fn(&Array2<char>, Range<usize>, Range<usize>, ReflectionType) -> bool,
) -> usize {
    mirror_arrays
        .iter()
        .map(|mirror_array| {
            let reflection_line = find_reflection_line(mirror_array, &reflection_criteria);

            println!("{:?}", reflection_line);

            match reflection_line {
                (ReflectionType::Row, line) => line * 100,
                (ReflectionType::Column, col) => col,
                (ReflectionType::None, _) => unreachable!(),
            }
        })
        .sum::<usize>()
}

// In part 1 the row/column ranges reflect when they are equal
fn part1_reflection_criteria(
    mirror_array: &Array2<char>,
    range1: Range<usize>,
    range2: Range<usize>,
    reflection_type: ReflectionType,
) -> bool {
    range1.rev().zip(range2).all(|(lower, higher)| {
        get_array_view(mirror_array, lower, reflection_type).to_vec()
            == get_array_view(mirror_array, higher, reflection_type).to_vec()
    })
}

// In part 2 the row/column ranges reflect when they differ by exactly
// one character
fn part2_reflection_criteria(
    mirror_array: &Array2<char>,
    range1: Range<usize>,
    range2: Range<usize>,
    reflection_type: ReflectionType,
) -> bool {
    range1
        .rev()
        .zip(range2)
        .map(|(lower, higher)| {
            let lower_set: String = get_array_view(mirror_array, lower, reflection_type)
                .iter()
                .collect();
            let higher_set: String = get_array_view(mirror_array, higher, reflection_type)
                .iter()
                .collect();

            lower_set
                .chars()
                .zip(higher_set.chars())
                .filter(|(c1, c2)| c1 != c2)
                .count()
        })
        .sum::<usize>()
        == 1
}

fn part1(input: &mut impl BufRead) -> String {
    let mirror_arrays = parse_input(input);

    debug!("{:?}", mirror_arrays);

    get_summary(&mirror_arrays, part1_reflection_criteria).to_string()
}

fn part2(input: &mut impl BufRead) -> String {
    let mirror_arrays = parse_input(input);

    debug!("{:?}", mirror_arrays);

    get_summary(&mirror_arrays, part2_reflection_criteria).to_string()
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

        assert_eq!(part1(&mut reader), "33520");
        reader.rewind().unwrap();
        assert_eq!(part2(&mut reader), "34824");
    }

    #[test]
    fn part1_tests() {
        init();

        let f = File::open("input.example").unwrap();
        let mut reader = BufReader::new(f);

        assert_eq!(part1(&mut reader), "405");
    }

    #[test]
    fn part2_tests() {
        init();

        let f = File::open("input.example").unwrap();
        let mut reader = BufReader::new(f);

        assert_eq!(part2(&mut reader), "400");
    }
}
