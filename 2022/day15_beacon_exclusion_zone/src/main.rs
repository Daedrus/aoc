use regex::Regex;
use std::cmp::max;

// const INPUT: &str = include_str!("../input.example");
const INPUT: &str = include_str!("../input");

#[derive(Debug)]
struct SensorBeaconPair {
    sensor_pos: (i64, i64),
    beacon_pos: (i64, i64),
    taxicab_distance: i64
}

fn taxicab_distance(point1: (i64, i64), point2: (i64, i64)) -> i64 {
    (point1.0 - point2.0).abs() + (point1.1 - point2.1).abs()
}

fn intersects(row: i64, sensor_beacon_pairs: &Vec<SensorBeaconPair>) -> i64 {
    let mut intervals: Vec<(i64,i64)> = Vec::new();
    let mut merged_intervals: Vec<(i64,i64)> = Vec::new();
    for sensor_beacon_pair in sensor_beacon_pairs {
        if row >= sensor_beacon_pair.sensor_pos.0 - sensor_beacon_pair.taxicab_distance &&
            row <= sensor_beacon_pair.sensor_pos.0 + sensor_beacon_pair.taxicab_distance {
            let left = sensor_beacon_pair.sensor_pos.1 - sensor_beacon_pair.taxicab_distance + (row - sensor_beacon_pair.sensor_pos.0).abs();
            let right = sensor_beacon_pair.taxicab_distance - (row - sensor_beacon_pair.sensor_pos.0).abs() + sensor_beacon_pair.sensor_pos.1;
            intervals.push((left, right));
        }
    }

    // Merge overlapping intervals algorithm
    // I'll admit I did not know this algorithm so I looked it up, but I knew there had to be one
    intervals.sort_by_key(|x| x.0);

    for interval in intervals {
        if merged_intervals.is_empty() || merged_intervals.last().unwrap().1 < interval.0 {
            merged_intervals.push(interval);
        } else {
            let merged_interval_right = merged_intervals.last().unwrap().1; 
            merged_intervals.last_mut().unwrap().1 = max(merged_interval_right, interval.1);
        }
    }

    let mut merged_intervals_len = 0;
    for merged_interval in &merged_intervals {
        merged_intervals_len += merged_interval.1 - merged_interval.0;
    }

    if merged_intervals.len() == 2 &&
        merged_intervals[1].0 - merged_intervals[0].1 == 2 &&
        row >= 0 && row <= 4000000 &&
        merged_intervals[0].1 + 1 >= 0 && merged_intervals[0].1 + 1 <= 4000000 {
        // println!("candidate {} {:?}", row, merged_intervals);
        println!("{}", (merged_intervals[0].1 + 1) * 4000000 + row);
    }

    merged_intervals_len
}

fn main() {
    let sensor_beacon_regex = Regex::new(r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)").unwrap();

    let mut sensor_beacon_pairs: Vec<SensorBeaconPair> = Vec::new();

    let mut lines = INPUT.lines();

    let mut leftmost_covered_column = i64::MAX;
    let mut rightmost_covered_column = i64::MIN;
    let mut topmost_covered_row = i64::MAX;
    let mut bottommost_covered_row = i64::MIN;

    while let Some(line) = lines.next() {
        let result = sensor_beacon_regex.captures_iter(&line).nth(0).unwrap();
        let sensor_pos_row = result[2].parse::<i64>().unwrap();
        let sensor_pos_column = result[1].parse::<i64>().unwrap();
        let beacon_pos_row = result[4].parse::<i64>().unwrap();
        let beacon_pos_column = result[3].parse::<i64>().unwrap();

        let sensor_beacon_pair = SensorBeaconPair{
            sensor_pos:(sensor_pos_row, sensor_pos_column),
            beacon_pos:(beacon_pos_row, beacon_pos_column),
            taxicab_distance: taxicab_distance((sensor_pos_row, sensor_pos_column), (beacon_pos_row, beacon_pos_column))
        };

        if leftmost_covered_column > sensor_pos_column - sensor_beacon_pair.taxicab_distance {
            leftmost_covered_column = sensor_pos_column - sensor_beacon_pair.taxicab_distance;
        }
        if rightmost_covered_column < sensor_pos_column + sensor_beacon_pair.taxicab_distance {
            rightmost_covered_column = sensor_pos_column + sensor_beacon_pair.taxicab_distance;
        }
        if topmost_covered_row > sensor_pos_row - sensor_beacon_pair.taxicab_distance {
            topmost_covered_row = sensor_pos_row - sensor_beacon_pair.taxicab_distance;
        }
        if bottommost_covered_row < sensor_pos_row + sensor_beacon_pair.taxicab_distance {
            bottommost_covered_row = sensor_pos_row + sensor_beacon_pair.taxicab_distance;
        }

        sensor_beacon_pairs.push(sensor_beacon_pair);
    }

    // println!("{}", leftmost_covered_column);
    // println!("{}", rightmost_covered_column);
    // println!("{}", topmost_covered_row);
    // println!("{}", bottommost_covered_row);

    // example input
    // topmost_covered_row = 10;
    // bottommost_covered_row = 10;

    // Part 1
    // topmost_covered_row = 2000000;
    // bottommost_covered_row = 2000000;

    let mut cant_have_beacon = 0;

    let mut current_row = topmost_covered_row;

    while current_row <= bottommost_covered_row {
        cant_have_beacon += intersects(current_row, &sensor_beacon_pairs);
        current_row += 1;
    }

    println!("{}", cant_have_beacon);
}
