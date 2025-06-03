use log::info;
use serde_json::Value;
use std::{
    fs::File,
    io::{self, BufRead, BufReader, Seek},
};

// Sum all numbers but skip the objects whose values fulfill the is_bad_value
// predicate. This way we can reuse the function for both parts.
fn sum_all_numbers(node: &Value, is_bad_value: fn(&Value) -> bool) -> i64 {
    match node {
        Value::Null => 0,
        Value::Bool(_) => 0,
        Value::Number(number) => number.as_i64().unwrap(),
        Value::String(_) => 0,
        Value::Array(array) => array
            .iter()
            .fold(0, |acc, elem| acc + sum_all_numbers(elem, is_bad_value)),
        Value::Object(object) => {
            if object.values().any(is_bad_value) {
                0
            } else {
                object
                    .values()
                    .fold(0, |acc, elem| acc + sum_all_numbers(elem, is_bad_value))
            }
        }
    }
}

fn part1(input: &mut impl BufRead) -> String {
    let json_tree: Value = serde_json::from_reader(input).expect("Could not parse JSON file");

    sum_all_numbers(&json_tree, |_| false).to_string()
}

fn part2(input: &mut impl BufRead) -> String {
    let json_tree: Value = serde_json::from_reader(input).expect("Could not parse JSON file");

    sum_all_numbers(
        &json_tree,
        |value| matches!(value, Value::String(s) if s.eq("red")),
    )
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
    use std::io::Cursor;

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn part1_tests() {
        assert_eq!(part1(&mut Cursor::new("[1,2,3]")), "6");
        assert_eq!(part1(&mut Cursor::new("{\"a\":2,\"b\":4}")), "6");
        assert_eq!(part1(&mut Cursor::new("[[[3]]]")), "3");
        assert_eq!(part1(&mut Cursor::new("{\"a\":{\"b\":4},\"c\":-1}")), "3");
        assert_eq!(part1(&mut Cursor::new("{\"a\":[-1,1]}")), "0");
        assert_eq!(part1(&mut Cursor::new("[-1,{\"a\":1}]")), "0");
        assert_eq!(part1(&mut Cursor::new("[]")), "0");
        assert_eq!(part1(&mut Cursor::new("{}")), "0");

        init();
    }

    #[test]
    fn part2_tests() {
        assert_eq!(part2(&mut Cursor::new("[1,2,3]")), "6");
        assert_eq!(
            part2(&mut Cursor::new("[1,{\"c\":\"red\",\"b\":2},3]")),
            "4"
        );
        assert_eq!(
            part2(&mut Cursor::new("{\"d\":\"red\",\"e\":[1,2,3,4],\"f\":5}")),
            "0"
        );
        assert_eq!(part2(&mut Cursor::new("[1,\"red\",5]")), "6");

        init();
    }

    #[test]
    fn check_answers() {
        init();

        let f = File::open("input").unwrap();
        let mut reader = BufReader::new(f);

        assert_eq!(part1(&mut reader), "156366");
        reader.rewind().unwrap();
        assert_eq!(part2(&mut reader), "96852");
    }
}
