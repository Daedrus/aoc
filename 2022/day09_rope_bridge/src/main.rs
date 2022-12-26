use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::env;
use regex::Regex;
use std::collections::HashSet;

fn snap(current_pos_head: (i32, i32), current_pos_tail: &mut (i32, i32), visited_pos: &mut HashSet<(i32, i32)>, register_visited: bool) {
    //println!("Snapping {:?} {:?}", current_pos_head, current_pos_tail);
    match (current_pos_head.0 - current_pos_tail.0, current_pos_head.1 - current_pos_tail.1) {
        // If they overlap, don't do anything
        (0,0) => {
            return;
        },
        // If they are on the same line
        (0,2) => {
            current_pos_tail.1 += 1;
        },
        (0,-2) => {
            current_pos_tail.1 -= 1;
        },
        // If they are on the same column
        (2,0) => {
            current_pos_tail.0 += 1;
        },
        (-2,0) => {
            current_pos_tail.0 -= 1;
        },
        // If head is NE
        (2,1) | (1,2) | (2,2) => {
            current_pos_tail.0 += 1;
            current_pos_tail.1 += 1;
        },
        // If head is SE
        (-1,2) | (-2,1) | (-2,2) => {
            current_pos_tail.0 -= 1;
            current_pos_tail.1 += 1;
        },
        // If head is SW
        (-1,-2) | (-2,-1) | (-2,-2) => {
            current_pos_tail.0 -= 1;
            current_pos_tail.1 -= 1;
        },
        // If head is NW
        (1,-2) | (2,-1) | (2,-2) => {
            current_pos_tail.0 += 1;
            current_pos_tail.1 -= 1;
        },
        (0,1) | (1,0) | (0,-1) | (-1,0) | (1,1) | (1,-1) | (-1,1) | (-1,-1) => {
            // println!("TOUCHING");
        },
        _ => {
            println!("PROBLEM {:?} {:?}", current_pos_head, current_pos_tail);
            panic!();
        }
    }

    //println!("Snapped to {:?} {:?}", current_pos_head, current_pos_tail);

    if register_visited {
        visited_pos.insert(*current_pos_tail);
    }
}

//Part 1
//const ROPE_SIZE: usize = 2;
const ROPE_SIZE: usize = 10;

fn main() {
    let args: Vec<String> = env::args().collect();

    let cmd_up    = Regex::new(r"U (\d+)").unwrap();
    let cmd_down  = Regex::new(r"D (\d+)").unwrap();
    let cmd_left  = Regex::new(r"L (\d+)").unwrap();
    let cmd_right = Regex::new(r"R (\d+)").unwrap();

    let mut visited_pos: HashSet<(i32,i32)> = HashSet::new();
    visited_pos.insert((0,0));

    let mut rope: [(i32,i32); ROPE_SIZE] = [(0,0); ROPE_SIZE];

    if let Ok(lines) = read_lines(Path::new(&args[1])) {
        for line in lines {
            if let Ok(terminal_line) = line {
                //println!("{:?}", terminal_line);
                if cmd_up.is_match(&terminal_line) {
                    let result = cmd_up.captures_iter(&terminal_line).nth(0).unwrap();
                    let dist = result[1].parse::<u32>().unwrap();

                    for _ in 0..dist {
                        rope[0].0 += 1;
                        for knot in 0..(rope.len()-1) {
                            snap(rope[knot], &mut rope[knot+1], &mut visited_pos, knot == ROPE_SIZE-2);
                        }
                        //println!("AFTER ONE MOVE {:?}", rope);
                    }
                    //println!("AFTER ONE COMMAND {:?}", rope);

                } else if cmd_down.is_match(&terminal_line) {
                    let result = cmd_down.captures_iter(&terminal_line).nth(0).unwrap();
                    let dist = result[1].parse::<u32>().unwrap();

                    for _ in 0..dist {
                        rope[0].0 -= 1;
                        for knot in 0..(rope.len()-1) {
                            snap(rope[knot], &mut rope[knot+1], &mut visited_pos, knot == ROPE_SIZE-2);
                        }
                    }
                    //println!("AFTER ONE COMMAND {:?}", rope);

                } else if cmd_left.is_match(&terminal_line) {
                    let result = cmd_left.captures_iter(&terminal_line).nth(0).unwrap();
                    let dist = result[1].parse::<u32>().unwrap();

                    for _ in 0..dist {
                        rope[0].1 -= 1;
                        for knot in 0..(rope.len()-1) {
                            snap(rope[knot], &mut rope[knot+1], &mut visited_pos, knot == ROPE_SIZE-2);
                        }
                    }
                    //println!("AFTER ONE COMMAND {:?}", rope);

                } else if cmd_right.is_match(&terminal_line) {
                    let result = cmd_right.captures_iter(&terminal_line).nth(0).unwrap();
                    let dist = result[1].parse::<u32>().unwrap();

                    for _ in 0..dist {
                        rope[0].1 += 1;
                        for knot in 0..(rope.len()-1) {
                            snap(rope[knot], &mut rope[knot+1], &mut visited_pos, knot == ROPE_SIZE-2);
                        }
                    }
                    //println!("AFTER ONE COMMAND {:?}", rope);

                } else {
                    println!("PROBLEM");
                }
            }
        }
    }

    println!("{:?}", visited_pos.len());
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
