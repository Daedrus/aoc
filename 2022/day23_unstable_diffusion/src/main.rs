use std::collections::HashSet;
use std::collections::HashMap;

// const INPUT: &str = include_str!("../input.minimal");
// const INPUT: &str = include_str!("../input.example");
const INPUT: &str = include_str!("../input");

const ROUNDS: u32 = 10;

#[derive(Debug)]
enum Direction {
    North,
    South,
    East,
    West
}

fn print_map(elves: &HashSet<(i64, i64)>) {
    println!("");
    for i in -5..15 {
        for j in -5..15 {
            if elves.contains(&(i, j)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!("");
    }
}

fn main() {
    let mut lines = INPUT.lines();
    let mut elves: HashSet<(i64, i64)> = HashSet::new();
    let mut proposals: HashMap<(i64, i64), Vec<(i64, i64)>> = HashMap::new();

    let mut directions: Vec<Direction> = vec![
        Direction::North,
        Direction::South,
        Direction::West,
        Direction::East];
    let mut round = 1;

    let mut line_index = 0;
    while let Some(line) = lines.next() {
        line.chars()
            .zip(0..line.len())
            .for_each(|(c, i)| { if c == '#' { elves.insert((line_index, i as i64)); } });

        line_index += 1;
    }

    // println!("{:?}", elves);
    // print_map(&elves);

    // Part 1
    // while round <= ROUNDS {
    loop {
        // println!("Round {:?} is starting", round);
        // println!("Direction order is {:?}", directions);
        for elf in &elves {
            // println!("Elf {:?} is considering", elf);
            if !elves.contains(&(elf.0 - 1, elf.1    )) &&
               !elves.contains(&(elf.0 - 1, elf.1 - 1)) &&
               !elves.contains(&(elf.0 - 1, elf.1 + 1)) &&
               !elves.contains(&(elf.0 + 1, elf.1    )) &&
               !elves.contains(&(elf.0 + 1, elf.1 - 1)) &&
               !elves.contains(&(elf.0 + 1, elf.1 + 1)) &&
               !elves.contains(&(elf.0    , elf.1 - 1)) &&
               !elves.contains(&(elf.0    , elf.1 + 1)) {
                // println!("Elf {:?} stays put", elf);
            } else {
                for direction in &directions {
                    match direction {
                        Direction::North => {
                            if !elves.contains(&(elf.0 - 1, elf.1    )) &&
                               !elves.contains(&(elf.0 - 1, elf.1 - 1)) &&
                               !elves.contains(&(elf.0 - 1, elf.1 + 1)) {
                                if proposals.contains_key(&(elf.0 - 1, elf.1)) {
                                    proposals.get_mut(&(elf.0 - 1, elf.1)).unwrap().push((elf.0, elf.1));
                                } else {
                                    proposals.insert((elf.0 - 1, elf.1), vec![(elf.0, elf.1)]);
                                }
                                // println!("Elf {:?} proposes N {:?}", elf, (elf.0 - 1, elf.1));
                                break;
                            }
                        },
                        Direction::South => {
                            if !elves.contains(&(elf.0 + 1, elf.1    )) &&
                               !elves.contains(&(elf.0 + 1, elf.1 - 1)) &&
                               !elves.contains(&(elf.0 + 1, elf.1 + 1)) {
                                if proposals.contains_key(&(elf.0 + 1, elf.1)) {
                                    proposals.get_mut(&(elf.0 + 1, elf.1)).unwrap().push((elf.0, elf.1));
                                } else {
                                    proposals.insert((elf.0 + 1, elf.1), vec![(elf.0, elf.1)]);
                                }
                                // println!("Elf {:?} proposes S {:?}", elf, (elf.0 + 1, elf.1));
                                break;
                            }
                        },
                        Direction::East => {
                            if !elves.contains(&(elf.0    , elf.1 + 1)) &&
                               !elves.contains(&(elf.0 - 1, elf.1 + 1)) &&
                               !elves.contains(&(elf.0 + 1, elf.1 + 1)) {
                                if proposals.contains_key(&(elf.0, elf.1 + 1)) {
                                    proposals.get_mut(&(elf.0, elf.1 + 1)).unwrap().push((elf.0, elf.1));
                                } else {
                                    proposals.insert((elf.0, elf.1 + 1), vec![(elf.0, elf.1)]);
                                }
                                // println!("Elf {:?} proposes E {:?}", elf, (elf.0, elf.1 + 1));
                                break;
                            }
                        },
                        Direction::West => {
                            if !elves.contains(&(elf.0    , elf.1 - 1)) &&
                               !elves.contains(&(elf.0 - 1, elf.1 - 1)) &&
                               !elves.contains(&(elf.0 + 1, elf.1 - 1)) {
                                if proposals.contains_key(&(elf.0, elf.1 - 1)) {
                                    proposals.get_mut(&(elf.0, elf.1 - 1)).unwrap().push((elf.0, elf.1));
                                } else {
                                    proposals.insert((elf.0, elf.1 - 1), vec![(elf.0, elf.1)]);
                                }
                                // println!("Elf {:?} proposes W {:?}", elf, (elf.0, elf.1 - 1));
                                break;
                            }
                        }
                    }
                }
            }
        }

        // println!("{:?}", proposals);

        for (proposal, proposal_elves) in &proposals {
            if proposal_elves.len() == 1 {
                elves.insert(*proposal);
                elves.remove(&proposal_elves[0]);
            }
        }

        if proposals.is_empty() {
            break;
        }

        proposals.clear();

        directions.rotate_left(1);
        round += 1;

        // print_map(&elves);
    }

    let mut sorted_elves = elves.into_iter().collect::<Vec<_>>();
    sorted_elves.sort();
    let min_x = sorted_elves.first().unwrap().0;
    let max_x = sorted_elves.last().unwrap().0;
    sorted_elves.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
    let min_y = sorted_elves.first().unwrap().1;
    let max_y = sorted_elves.last().unwrap().1;
    println!("{} {} {} {}", min_x, max_x, min_y, max_y);
    println!("{}", (max_x - min_x + 1).abs() * (max_y - min_y + 1).abs() - sorted_elves.len() as i64);
    println!("{}", round);
}
