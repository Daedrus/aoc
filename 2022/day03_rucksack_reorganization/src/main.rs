use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::env;
use std::collections::HashSet;
use std::collections::HashMap;


fn main() {
    let args: Vec<String> = env::args().collect();

    let priorities1:[(char, usize);26] = core::array::from_fn(|i| ((b'a'+(i as u8)) as char, i+1));
    let priorities2:[(char, usize);26] = core::array::from_fn(|i| ((b'A'+(i as u8)) as char, i+27));
    let mut priorities = HashMap::new();

    for priority in priorities1 {
        priorities.insert(priority.0, priority.1);
    }
    for priority in priorities2 {
        priorities.insert(priority.0, priority.1);
    }

    let mut sum:usize = 0;
    let mut badge_sum:usize = 0;

    let mut elf_number = 0;
    let mut group_rucksacks:Vec<String> = Vec::new();

    if let Ok(lines) = read_lines(Path::new(&args[1])) {
        for line in lines {
            if let Ok(rucksack) = line {
                let rucksack_len = rucksack.len();

                let compartment1 = rucksack[0..(rucksack_len / 2)].chars().collect::<HashSet<_>>();
                let compartment2 = rucksack[(rucksack_len / 2)..rucksack_len].chars().collect::<HashSet<_>>();

                let shared_item = compartment1.intersection(&compartment2).nth(0).unwrap();

                sum += priorities.get(shared_item).unwrap();

                if elf_number % 3 == 0 {
                    group_rucksacks.push(rucksack);
                } else if elf_number % 3 == 1 {
                    group_rucksacks.push(rucksack);
                } else if elf_number % 3 == 2 {
                    group_rucksacks.push(rucksack);

                    let rucksack1 = group_rucksacks[0].chars().collect::<HashSet<_>>();
                    let rucksack2 = group_rucksacks[1].chars().collect::<HashSet<_>>();
                    let rucksack3 = group_rucksacks[2].chars().collect::<HashSet<_>>();

                    badge_sum += priorities.get(rucksack1.intersection(&rucksack2).copied().collect::<HashSet<_>>().intersection(&rucksack3).nth(0).unwrap()).unwrap();

                    group_rucksacks.clear();
                }
                elf_number += 1;
            }
        }
    }

    println!("{}", sum);
    println!("{}", badge_sum);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
