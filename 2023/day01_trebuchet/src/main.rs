use log::{debug, info};
use std::{
    fs::File,
    io::{self, BufRead, BufReader, Seek},
};

fn part1(input: &mut impl BufRead) -> String {
    input
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let first_digit_pos = line.find(|c: char| c.is_ascii_digit()).unwrap();
            let last_digit_pos = line.rfind(|c: char| c.is_ascii_digit()).unwrap();

            let first_digit = line.chars().nth(first_digit_pos).unwrap();
            let last_digit = line.chars().nth(last_digit_pos).unwrap();

            format!("{}{}", first_digit, last_digit)
                .parse::<u32>()
                .unwrap()
        })
        .sum::<u32>()
        .to_string()
}

fn find_digit(line: &str, forward_search: bool) -> u32 {
    let spelled_out_digits = [
        /* Would it be possible to have this here and have the array be an
        array of Patterns that could be passed into find and rfind? */
        // |c: char| c.is_ascii_digit()
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    /* Find all positions of all digits (both ascii ones and
    spelled out). This means we're going to traverse each input
    string 10 times. */

    // First we find the positions of all spelled out digits
    let mut all_digits_pos: Vec<(usize, u32)> = spelled_out_digits
        .iter()
        /* Index the results, this is important for identifying which call the
        result belongs to. (pos, 1) will belong to the `[r]find("one")` call
        (pos, 2) will belong to the `[r]find("two")` call and so on */
        .zip(1..=9u32)
        .map(|(s, i)| {
            if forward_search {
                (line.find(s), i)
            } else {
                (line.rfind(s), i)
            }
        })
        // We're not interested in empty results so take them out
        .filter(|(s, _)| s.is_some())
        // Convert the results from (Option(pos), index) to (pos, index)
        .map(|(pos, i)| (pos.unwrap(), i))
        .collect();

    // Then look for an ascii digit and store the result as (pos, 0)
    if let Some(pos) = if forward_search {
        line.find(|c: char| c.is_ascii_digit())
    } else {
        line.rfind(|c: char| c.is_ascii_digit())
    } {
        all_digits_pos.push((pos, 0))
    }

    /* Depending on the search direction look for the left-most (its position
    is the minimum of all found positions) or right-most (its position is the
    maximum of all found positions) digit*/
    let digit_pos = if forward_search {
        all_digits_pos
            .iter()
            .min_by(|(a, _), (b, _)| a.cmp(b))
            .unwrap()
    } else {
        all_digits_pos
            .iter()
            .max_by(|(a, _), (b, _)| a.cmp(b))
            .unwrap()
    };

    match digit_pos {
        /* If the left or right-most digit has index 0 then that means it
        is an ascii digit so handle it just like in part1 */
        (pos, 0) => line.chars().nth(*pos).unwrap().to_digit(10).unwrap(),
        /* Otherwise one of the spelled out digits was left or right-most
        and its value is the index itself */
        (_, index) => *index,
    }
}

fn part2(input: &mut impl BufRead) -> String {
    input
        .lines()
        .map(|line| {
            let line = line.unwrap();

            let first_digit = find_digit(&line, true);
            let second_digit = find_digit(&line, false);

            debug!("{}", first_digit * 10 + second_digit);

            first_digit * 10 + second_digit
        })
        .sum::<u32>()
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

        assert_eq!(part1(&mut reader), "54940");
        reader.rewind().unwrap();
        assert_eq!(part2(&mut reader), "54208");
    }

    #[test]
    fn part1_tests() {
        init();

        let f = File::open("input.example1").unwrap();
        let mut reader = BufReader::new(f);

        assert_eq!(part1(&mut reader), "142");
    }

    #[test]
    fn part2_tests() {
        init();

        let f = File::open("input.example2").unwrap();
        let mut reader = BufReader::new(f);

        assert_eq!(part2(&mut reader), "281");
    }
}
