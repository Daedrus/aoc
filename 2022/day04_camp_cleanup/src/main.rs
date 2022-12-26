use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::env;
use regex::Regex;

fn main() {
    let args: Vec<String> = env::args().collect();

    let re = Regex::new(r"(\d+)-(\d+),(\d+)-(\d+)").unwrap();

    let mut fully_contained = 0;
    let mut overlapping = 0;

    if let Ok(lines) = read_lines(Path::new(&args[1])) {
        for line in lines {
            if let Ok(section_assignments) = line {
                let ranges = re.captures_iter(&section_assignments).nth(0).unwrap();
                let range1 = ranges[1].parse::<u32>().unwrap();
                let range2 = ranges[2].parse::<u32>().unwrap();
                let range3 = ranges[3].parse::<u32>().unwrap();
                let range4 = ranges[4].parse::<u32>().unwrap();

                if (range3..range4+1).contains(&range1) && (range3..range4+1).contains(&range2) {
                    fully_contained += 1;
                }
                else if (range1..range2+1).contains(&range3) && (range1..range2+1).contains(&range4) {
                    fully_contained += 1;
                }

                if (range3..range4+1).contains(&range1) || (range3..range4+1).contains(&range2) {
                    overlapping += 1;
                }
                else if (range1..range2+1).contains(&range3) || (range1..range2+1).contains(&range4) {
                    overlapping += 1;
                }
            }
        }
    }

    println!("{}", fully_contained);
    println!("{}", overlapping);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
