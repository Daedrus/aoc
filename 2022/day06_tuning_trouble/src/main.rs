use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::env;
use std::collections::HashSet;

//const CAPACITY: usize = 4;
const CAPACITY: usize = 14;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut diffchars: Vec<char> = Vec::with_capacity(CAPACITY);
    let mut index = 0;

    if let Ok(lines) = read_lines(Path::new(&args[1])) {
        for line in lines {
            if let Ok(datastream) = line {
                for c in datastream.chars() {
                    index += 1;
                    if diffchars.len() >= CAPACITY {
                        diffchars.remove(0);
                        diffchars.push(c);

                        if diffchars.clone().into_iter().collect::<HashSet<_>>().len() == CAPACITY {
                             break;
                        }
                    } else {
                        diffchars.push(c);
                    }
                }
            }
        }
    }

    println!("{}", index);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
