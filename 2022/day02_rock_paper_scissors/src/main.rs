use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::env;

const WIN: u32 = 6;
const DRAW: u32 = 3;
const LOSS: u32 = 0;
const ROCK: u32 = 1;     // A|X
const PAPER: u32 = 2;    // B|Y
const SCISSORS: u32 = 3; // C|Z

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut sum:u32 = 0;

    if let Ok(lines) = read_lines(Path::new(&args[1])) {
        for line in lines {
            if let Ok(round) = line {
                match round.as_str() {
                    // Part 1
                    // "A X" => sum += DRAW + ROCK,
                    // "A Y" => sum += WIN + PAPER,
                    // "A Z" => sum += LOSS + SCISSORS,
                    // "B X" => sum += LOSS + ROCK,
                    // "B Y" => sum += DRAW + PAPER,
                    // "B Z" => sum += WIN + SCISSORS,
                    // "C X" => sum += WIN + ROCK,
                    // "C Y" => sum += LOSS + PAPER,
                    // "C Z" => sum += DRAW + SCISSORS,
                    "A X" => sum += SCISSORS + LOSS,
                    "A Y" => sum += ROCK + DRAW,
                    "A Z" => sum += PAPER + WIN,
                    "B X" => sum += ROCK + LOSS,
                    "B Y" => sum += PAPER + DRAW,
                    "B Z" => sum += SCISSORS + WIN,
                    "C X" => sum += PAPER + LOSS,
                    "C Y" => sum += SCISSORS + DRAW,
                    "C Z" => sum += ROCK + WIN,
                    _ => unreachable!(),
                }
            }
        }
    }

    // Part 1
    println!("{}", sum);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
