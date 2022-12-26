use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::env;
use std::collections::BinaryHeap;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut cals = BinaryHeap::new();
    let mut curr_calories:u32 = 0;

    if let Ok(lines) = read_lines(Path::new(&args[1])) {
        for line in lines {
            if let Ok(calories) = line {
                if calories.is_empty() {
                    cals.push(curr_calories);
                    curr_calories = 0;
                } else {
                    curr_calories += calories.to_string().parse::<u32>().unwrap();
                }
            }
        }
    }

    // Part 1
    // println!("{}", cals.pop().unwrap());

    println!("{}", cals.pop().unwrap() + cals.pop().unwrap() + cals.pop().unwrap());
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
