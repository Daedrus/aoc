use itertools::Itertools;
use log::{debug, info};
use std::mem::swap;
use std::{
    fs::File,
    io::{self, BufRead, BufReader, Seek},
};

fn parse_input(input: &mut impl BufRead) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line| {
            line.unwrap()
                .split_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect()
}

fn first_and_last_values(histories: &[Vec<i32>]) -> (Vec<Vec<i32>>, Vec<Vec<i32>>) {
    let mut first_values: Vec<Vec<i32>> = vec![];
    let mut last_values: Vec<Vec<i32>> = vec![];

    // For all histories, collect all first and last values obtained by
    // following the algorithm described in the problem. Eventually all
    // extrapolated values can be reduced to computations on these values.
    histories.iter().for_each(|history| {
        // If we consider the first example
        //
        //                                   Iteration
        //                               #1      #2     #3
        // \0\  3    6   9   12   /15/   <- old
        //   \3\   3   3   3   /3/       <- new  <- old
        //     \0\   0   0   /0/                 <- new <- old
        //                                              <- new
        //
        // 0, 3, 0 will end up in current_history_first_values, in this order
        // 15, 3, 0 will end up in current_history_last_values, in this order
        let mut new_differences: Vec<i32> = vec![];
        let mut old_differences: Vec<i32> = history.clone();

        let mut current_history_first_values = vec![];
        let mut current_history_last_values = vec![];

        current_history_first_values.push(*old_differences.first().unwrap());
        current_history_last_values.push(*old_differences.last().unwrap());

        while old_differences.len() > 1 && !old_differences.iter().all(|d| *d == 0) {
            old_differences
                .iter()
                .tuple_windows()
                .for_each(|(v1, v2)| new_differences.push(v2 - v1));

            debug!("{:?}", old_differences);
            debug!("{:?}", new_differences);
            debug!("");

            current_history_first_values.push(*new_differences.first().unwrap());
            current_history_last_values.push(*new_differences.last().unwrap());

            swap(&mut old_differences, &mut new_differences);
            new_differences.clear();
        }

        first_values.push(current_history_first_values);
        last_values.push(current_history_last_values);
    });

    (last_values, first_values)
}

fn part1(input: &mut impl BufRead) -> String {
    let histories = parse_input(input);

    debug!("{:?}", histories);

    // If we consider a generic array with index v_i_j
    // where:
    // i = iteration number (0 is the original history array)
    // j = index in the array (x is the extrapolated value we want to deduce)
    //
    // v_0_0 v_0_1 v_0_2 ............ v_0_n  v_0_x
    //    v_1_0 v_1_1 v_1_2 ....... v_1_n-1 v_1_x
    //      v_2_0 v_2_1 v_2_2 ... v_2_n-2  v_2_x
    //        ......................
    //          0  0  0   ...    0
    //
    // According to the problem:
    //  v_2_x - v_2_n-2 = 0
    //  v_1_x - v_1_n-1 = v_2_x
    //  v_0_x - v_0_n = v_1_x
    //
    // And we're intersted in v_0_x.
    //
    // v_0_x = v_1_x + v_0_n
    //       = v_2_x + v_1_n-1 + v_0_n
    //       = v_2_n-2 + v_1_n-1 + v_0_n
    //       ...
    //
    // This generalizes to the sum of all "last values".
    //
    // The "last_values" array is formed of (v_0_n, v_1_n-1, v_2_n-2 ... 0)
    first_and_last_values(&histories)
        .0
        .iter()
        .map(|last_values| last_values.iter().sum::<i32>())
        .sum::<i32>()
        .to_string()
}

fn part2(input: &mut impl BufRead) -> String {
    let histories = parse_input(input);

    // If we consider a generic array with index v_i_j
    // where:
    // i = iteration number (0 is the original history array)
    // j = index in the array (x is the extrapolated value we want to deduce)
    //
    // v_0_x v_0_0 v_0_1 v_0_2 ............... v_0_n
    //   v_1_x v_1_0 v_1_1 v_1_2 ........... v_1_n-1
    //     v_2_x v_2_0 v_2_1 v_2_2 ....... v_2_n-2
    //       v_3_x v_3_0 v_3_1 v_3_2 ... v_3_n-2
    //        ............................
    //          0  0  0    ...    0
    //
    // According to the problem:
    //  v_3_0 - v_3_x = 0
    //  v_2_0 - v_2_x = v_3_x
    //  v_1_0 - v_1_x = v_2_x
    //  v_0_0 - v_0_x = v_1_x
    //
    // And we're intersted in v_0_x.
    //
    // v_0_x = v_0_0 - v_1_x
    //       = v_0_0 - (v_1_0 - v_2_x)
    //       = v_0_0 - (v_1_0 - (v_2_0 - v_3_x))
    //       = v_0_0 - (v_1_0 - (v_2_0 - v_3_0))
    //       = v_0_0 - (v_1_0 - v_2_0 + v_3_0)
    //       = v_0_0 - v_1_0 + v_2_0 - v_3_0
    //       ...
    //
    // This generalizes to the sum of all "first values" but with alternating
    // sign depending on the index (odd indexes are subtracted, even ones added)
    //
    // The "first_values" array is formed of (v_0_0, v_1_0, v_2_0 ... 0)
    first_and_last_values(&histories)
        .1
        .iter()
        .map(|first_values| {
            first_values.iter().enumerate().fold(
                0,
                |acc, (index, v)| if index % 2 == 0 { acc + v } else { acc - v },
            )
        })
        .sum::<i32>()
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

        assert_eq!(part1(&mut reader), "1581679977");
        reader.rewind().unwrap();
        assert_eq!(part2(&mut reader), "889");
    }

    #[test]
    fn part1_tests() {
        init();

        let f = File::open("input.example").unwrap();
        let mut reader = BufReader::new(f);

        assert_eq!(part1(&mut reader), "114");
    }

    #[test]
    fn part2_tests() {
        init();

        let f = File::open("input.example").unwrap();
        let mut reader = BufReader::new(f);

        assert_eq!(part2(&mut reader), "2");
    }
}
