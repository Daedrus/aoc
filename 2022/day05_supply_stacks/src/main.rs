use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::env;
use regex::Regex;

fn main() {
    let args: Vec<String> = env::args().collect();

    let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();

    // hardcoded Vecs since ain't nobody got time fo' that :)
    let mut stacks: [Vec<char>; 9] = [
        vec!['B','Q','C'],
        vec!['R','Q','W','Z'],
        vec!['B','M','R','L','V'],
        vec!['C','Z','H','V','T','W'],
        vec!['D','Z','H','B','N','V','G'],
        vec!['H','N','P','C','J','F','V','Q'],
        vec!['D','G','T','R','W','Z','S'],
        vec!['C','G','M','N','B','W','Z','P'],
        vec!['N','J','B','M','W','Q','F','P']];

    if let Ok(lines) = read_lines(Path::new(&args[1])) {
        for line in lines.skip(10) {
            if let Ok(move_procedure) = line {
                let moves = re.captures_iter(&move_procedure).nth(0).unwrap();
                let move_amount = moves[1].parse::<usize>().unwrap();
                let from_stack  = moves[2].parse::<usize>().unwrap();
                let to_stack    = moves[3].parse::<usize>().unwrap();

                // Part 1
                // for _ in 0..move_amount {
                //     let c = stacks[from_stack-1].pop().unwrap();
                //     stacks[to_stack-1].push(c);
                // }

                for i in (0..move_amount).rev() {
                    stacks[to_stack-1].push(*stacks[from_stack-1].get(stacks[from_stack-1].len()-i-1).unwrap());
                }

                for _ in 0..move_amount {
                    stacks[from_stack-1].pop();
                }
            }
        }
    }

    for stack in stacks {
        println!("{}", stack.last().unwrap());
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
