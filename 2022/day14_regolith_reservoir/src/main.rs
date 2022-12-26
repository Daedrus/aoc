use regex::Regex;
use std::cmp::min;
use std::cmp::max;

//const INPUT: &str = include_str!("../input.example");
const INPUT: &str = include_str!("../input");
const WORLD_SIZE: usize = 1000;

enum SandState {
    Settled,
    Abyss,
    StartingPoint
}

fn simulate_sand(map: &mut[[char; WORLD_SIZE]; WORLD_SIZE]) -> SandState {
    let mut sand_pos = (0, 500);
    loop {
        if sand_pos.0 == WORLD_SIZE - 1 {
            return SandState::Abyss;
        }
        if map[sand_pos.0 + 1][sand_pos.1] == '.' {
            sand_pos.0 += 1;
        } else if map[sand_pos.0 + 1][sand_pos.1 - 1] == '.' {
            sand_pos.0 += 1;
            sand_pos.1 -= 1;
        } else if map[sand_pos.0 + 1][sand_pos.1 + 1] == '.' {
            sand_pos.0 += 1;
            sand_pos.1 += 1;
        } else {
            map[sand_pos.0][sand_pos.1] = 'o';
            if sand_pos == (0, 500) {
                return SandState::StartingPoint;
            } else {
                return SandState::Settled;
            }
        }
    }
}

fn main() {
    let path = Regex::new(r"(\d+),(\d+)").unwrap();

    let mut map: [[char; WORLD_SIZE]; WORLD_SIZE] =  [['.'; WORLD_SIZE]; WORLD_SIZE];

    let mut lines = INPUT.lines();
    let mut floor = 0;
    while let Some(line) = lines.next() {
        for segment in path.captures_iter(&line).zip(path.captures_iter(&line).skip(1)) {
            let endpoint1_column = segment.0[1].parse::<usize>().unwrap();
            let endpoint1_row = segment.0[2].parse::<usize>().unwrap();
            let endpoint2_column = segment.1[1].parse::<usize>().unwrap();
            let endpoint2_row = segment.1[2].parse::<usize>().unwrap();
            if endpoint1_column == endpoint2_column {
                for i in min(endpoint1_row, endpoint2_row)..=max(endpoint1_row, endpoint2_row) {
                    map[i][endpoint1_column] = '#';
                }
            } else if endpoint1_row == endpoint2_row {
                for j in min(endpoint1_column, endpoint2_column)..=max(endpoint1_column, endpoint2_column) {
                    map[endpoint1_row][j] = '#';
                }
                if floor < endpoint1_row {
                    floor = endpoint1_row;
                }
            } else {
                unreachable!();
            }
        }
    }

    floor += 2;
    map[0][500] = '+';

    // Part 1
    /*
    let mut settled_sand = 0;
    loop {
        match simulate_sand(&mut map) {
            SandState::Settled => { settled_sand += 1; },
            SandState::StartingPoint => unreachable!(),
            SandState::Abyss => break,
        }
    }
    */

    for j in 0..WORLD_SIZE {
        map[floor][j] = '#';
    }

    let mut settled_sand = 0;
    loop {
        match simulate_sand(&mut map) {
            SandState::Settled => { settled_sand += 1; },
            SandState::StartingPoint => { settled_sand += 1; break },
            SandState::Abyss => unreachable!(),
        }
    }

    for i in 0..WORLD_SIZE {
        for j in 0..WORLD_SIZE {
            print!{"{}", map[i][j]};
        }
        println!{}
    }

    println!{"{}", settled_sand};
    println!{"{}", floor};
}
