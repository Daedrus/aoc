use regex::Regex;
use std::collections::HashMap;

// const INPUT: &str = include_str!("../input.example");
const INPUT: &str = include_str!("../input");
// Example sizes
// const MAP_HEIGHT: usize = 12;
// const MAP_LENGTH: usize = 16;
// const CUBE_SIDE: usize = 4;
// Real sizes
const MAP_HEIGHT: usize = 200;
const MAP_LENGTH: usize = 150;
const CUBE_SIDE: usize = 50;

#[derive(Debug)]
enum TurnDirection {
    Left, // counterclockwise
    Right, // clockwise
    None, // don't turn
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Ord, PartialOrd)]
enum FacingDirection {
    Up = 3,
    Down = 1,
    Left = 2,
    Right = 0,
}

#[derive(Debug)]
struct MoveAndTurn {
    steps: usize,
    turn: TurnDirection 
}

#[derive(Debug)]
struct PositionAndFacingDirection {
    x: usize,
    y: usize,
    dir: FacingDirection
}

fn step_to_next_position(current_pos: &mut PositionAndFacingDirection, map: &[[char; MAP_LENGTH]; MAP_HEIGHT], wraps: &HashMap<(usize, usize, FacingDirection),(usize, usize, FacingDirection)>) {
    match current_pos.dir {
        FacingDirection::Up => {
            if current_pos.x == 0 || map[current_pos.x-1][current_pos.y] == ' ' {
                let next_pos = wraps.get(&(current_pos.x, current_pos.y, FacingDirection::Up)).unwrap();
                if map[next_pos.0][next_pos.1] != '#' {
                    current_pos.x = next_pos.0;
                    current_pos.y = next_pos.1;
                    current_pos.dir = next_pos.2;
                }
            } else if map[current_pos.x-1][current_pos.y] != '#' {
                current_pos.x -= 1;
            }
        },
        FacingDirection::Down => {
            if current_pos.x == MAP_HEIGHT - 1 || map[current_pos.x+1][current_pos.y] == ' ' {
                let next_pos = wraps.get(&(current_pos.x, current_pos.y, FacingDirection::Down)).unwrap();
                if map[next_pos.0][next_pos.1] != '#' {
                    current_pos.x = next_pos.0;
                    current_pos.y = next_pos.1;
                    current_pos.dir = next_pos.2;
                }
            } else if map[current_pos.x+1][current_pos.y] != '#' {
                current_pos.x += 1;
            }
        },
        FacingDirection::Left => {
            if current_pos.y == 0 || map[current_pos.x][current_pos.y-1] == ' ' {
                let next_pos = wraps.get(&(current_pos.x, current_pos.y, FacingDirection::Left)).unwrap();
                if map[next_pos.0][next_pos.1] != '#' {
                    current_pos.x = next_pos.0;
                    current_pos.y = next_pos.1;
                    current_pos.dir = next_pos.2;
                }
            } else if map[current_pos.x][current_pos.y-1] != '#' {
                current_pos.y -= 1;
            }
        },
        FacingDirection::Right => {
            if current_pos.y == MAP_LENGTH - 1 || map[current_pos.x][current_pos.y+1] == ' ' {
                let next_pos = wraps.get(&(current_pos.x, current_pos.y, FacingDirection::Right)).unwrap();
                if map[next_pos.0][next_pos.1] != '#' {
                    current_pos.x = next_pos.0;
                    current_pos.y = next_pos.1;
                    current_pos.dir = next_pos.2;
                }
            } else if map[current_pos.x][current_pos.y+1] != '#' {
                current_pos.y += 1;
            }
        },
    }

    // println!("{:?} {} {}", current_pos.dir as usize, current_pos.x, current_pos.y);
}

fn create_part1_wraps(map: &[[char; MAP_LENGTH]; MAP_HEIGHT], wraps: &mut HashMap<(usize, usize, FacingDirection),(usize, usize, FacingDirection)>) {
    // Scan by rows and check where they wrap
    for i in 0..MAP_HEIGHT {
        let mut first_index: usize = MAP_LENGTH + 1;
        let mut last_index: usize = MAP_LENGTH + 1;
        for j in 0..MAP_LENGTH {
            if first_index == MAP_LENGTH + 1 {
                if map[i][j] != ' ' {
                    first_index = j;
                }
            } else if last_index == MAP_LENGTH + 1 {
                if map[i][j] == ' ' {
                    last_index = j;
                    wraps.insert((i, first_index, FacingDirection::Left), (i,last_index-1,FacingDirection::Left));
                    wraps.insert((i, last_index-1, FacingDirection::Right), (i,first_index,FacingDirection::Right));
                    break;
                } else if j == MAP_LENGTH - 1 {
                    last_index = j;
                    wraps.insert((i, first_index, FacingDirection::Left), (i,last_index,FacingDirection::Left));
                    wraps.insert((i, last_index, FacingDirection::Right), (i,first_index,FacingDirection::Right));
                    break;
                }
            }
        }
    }

    // for (key,value) in wraps.iter() {
    //     println!("{:?}", (key, value));
    // }

    // Scan by columns and check where they wrap
    for j in 0..MAP_LENGTH {
        let mut first_index: usize = MAP_HEIGHT + 1;
        let mut last_index: usize = MAP_HEIGHT + 1;
        for i in 0..MAP_HEIGHT {
            if first_index == MAP_HEIGHT + 1 {
                if map[i][j] != ' ' {
                    first_index = i;
                }
            } else if last_index == MAP_HEIGHT + 1 {
                if map[i][j] == ' ' {
                    last_index = i;
                    wraps.insert((first_index, j, FacingDirection::Up), (last_index-1, j, FacingDirection::Up));
                    wraps.insert((last_index-1, j, FacingDirection::Down), (first_index, j, FacingDirection::Down));
                } else if i == MAP_HEIGHT - 1 {
                    last_index = i;
                    wraps.insert((first_index, j, FacingDirection::Up), (last_index, j, FacingDirection::Up));
                    wraps.insert((last_index, j, FacingDirection::Down), (first_index, j, FacingDirection::Down));
                }
            }
        }
    }
}

fn create_part2_wraps(wraps: &mut HashMap<(usize, usize, FacingDirection),(usize, usize, FacingDirection)>) {
    // Note that these wraps only work on my cube, not on the example one
    // Not sure how to generalize for all cube layouts
    for i in 0..CUBE_SIDE {
        // Quadrant 1 wraps
        wraps.insert(
            (              0,   CUBE_SIDE+i, FacingDirection::Up),
            (  3*CUBE_SIDE+i,             0, FacingDirection::Right)); // Connects to quadrant 6
        wraps.insert(
            (              i,     CUBE_SIDE, FacingDirection::Left),
            (3*CUBE_SIDE-1-i,             0, FacingDirection::Right)); // Connects to quadrant 5

        // Quadrant 2 wraps
        wraps.insert(
            (              0, 2*CUBE_SIDE+i, FacingDirection::Up),
            (  4*CUBE_SIDE-1,             i, FacingDirection::Up)); // Connects to quadrant 6
        wraps.insert(
            (              i, 3*CUBE_SIDE-1, FacingDirection::Right),
            (3*CUBE_SIDE-1-i, 2*CUBE_SIDE-1, FacingDirection::Left)); // Connects to quadrant 4
        wraps.insert(
            (    CUBE_SIDE-1, 2*CUBE_SIDE+i, FacingDirection::Down),
            (    CUBE_SIDE+i, 2*CUBE_SIDE-1, FacingDirection::Left)); // Connects to quadrant 3

        // Quadrant 3 wraps
        wraps.insert(
            (    CUBE_SIDE+i,     CUBE_SIDE, FacingDirection::Left),
            (    2*CUBE_SIDE,             i, FacingDirection::Down)); // Connects to quadrant 5
        wraps.insert(
            (    CUBE_SIDE+i, 2*CUBE_SIDE-1, FacingDirection::Right),
            (    CUBE_SIDE-1, 2*CUBE_SIDE+i, FacingDirection::Up)); // Connects to quadrant 2

        // Quadrant 4 wraps
        wraps.insert(
            (3*CUBE_SIDE-1-i, 2*CUBE_SIDE-1, FacingDirection::Right),
            (              i, 3*CUBE_SIDE-1, FacingDirection::Left)); // Connects to quadrant 2
        wraps.insert(
            (  3*CUBE_SIDE-1,   CUBE_SIDE+i, FacingDirection::Down),
            (  3*CUBE_SIDE+i,   CUBE_SIDE-1, FacingDirection::Left)); // Connects to quadrant 6

        // Quadrant 5 wraps
        wraps.insert(
            (    2*CUBE_SIDE,             i, FacingDirection::Up),
            (    CUBE_SIDE+i,     CUBE_SIDE, FacingDirection::Right)); // Connects to quadrant 3
        wraps.insert(
            (3*CUBE_SIDE-1-i,             0, FacingDirection::Left),
            (              i,     CUBE_SIDE, FacingDirection::Right)); // Connects to quadrant 1

        // Quadrant 6 wraps
        wraps.insert(
            (  3*CUBE_SIDE+i,             0, FacingDirection::Left),
            (              0,   CUBE_SIDE+i, FacingDirection::Down)); // Connects to quadrant 1
        wraps.insert(
            (  3*CUBE_SIDE+i,   CUBE_SIDE-1, FacingDirection::Right),
            (  3*CUBE_SIDE-1,   CUBE_SIDE+i, FacingDirection::Up)); // Connects to qudrant 4
        wraps.insert(
            (  4*CUBE_SIDE-1,             i, FacingDirection::Down),
            (              0, 2*CUBE_SIDE+i, FacingDirection::Down)); // Connects to quadrant 2
    }
}

fn main() {
    let mut lines = INPUT.lines();
    let mut map: [[char; MAP_LENGTH]; MAP_HEIGHT] = [[' '; MAP_LENGTH]; MAP_HEIGHT];
    let movement_regex = Regex::new(r"(\d+)([L|R])?").unwrap();
    let mut movements: Vec<MoveAndTurn> = Vec::new();
    let mut wraps: HashMap<(usize, usize, FacingDirection),(usize, usize, FacingDirection)> = HashMap::new();

    let mut line_index = 0;
    while let Some(line) = lines.next() {
        if line_index < MAP_HEIGHT {
            line.chars()
                .zip(map[line_index].iter_mut())
                .for_each(|(c, m)| *m = c);
        } else if line_index == MAP_HEIGHT + 1 {
            let movement_capture = movement_regex.captures_iter(&line);

            movements = movement_capture.map(|mv| MoveAndTurn {
                steps: mv[1].parse::<usize>().unwrap(),
                turn: {
                    if let Some(dir) = mv.get(2) {
                        if dir.as_str().eq(&"L".to_string()) {
                            TurnDirection::Left
                        } else {
                            TurnDirection::Right
                        }
                    } else {
                        TurnDirection::None
                    }
                }
            }).collect::<Vec<MoveAndTurn>>();

            // println!("{:?}", movements);
        }

        line_index += 1;
    }

    // create_part1_wraps(&map, &mut wraps);
    create_part2_wraps(&mut wraps);

    // Starting position (hardcoded, too lazy to detect)
    // Example starting position
    // let mut you: PositionAndFacingDirection = PositionAndFacingDirection {
    //     x: 0,
    //     y: 8,
    //     dir: FacingDirection::Right
    // };
    let mut you: PositionAndFacingDirection = PositionAndFacingDirection {
        x: 0,
        y: 50,
        dir: FacingDirection::Right
    };

    // println!("{:?}", movements);

    for movement in movements {
        // println!("Moving {:?}", movement.steps);

        for _ in 0..movement.steps {
            step_to_next_position(&mut you, &map, &wraps);
        }

        match you.dir {
            FacingDirection::Up => {
                match movement.turn {
                    TurnDirection::Left => { you.dir = FacingDirection::Left; },
                    TurnDirection::Right => { you.dir = FacingDirection::Right; },
                    TurnDirection::None => { }
                }
            },
            FacingDirection::Down => {
                match movement.turn {
                    TurnDirection::Left => { you.dir = FacingDirection::Right; },
                    TurnDirection::Right => { you.dir = FacingDirection::Left; },
                    TurnDirection::None => { }
                }
            },
            FacingDirection::Left => {
                match movement.turn {
                    TurnDirection::Left => { you.dir = FacingDirection::Down; },
                    TurnDirection::Right => { you.dir = FacingDirection::Up; },
                    TurnDirection::None => { }
                }
            },
            FacingDirection::Right => {
                match movement.turn {
                    TurnDirection::Left => { you.dir = FacingDirection::Up; },
                    TurnDirection::Right => { you.dir = FacingDirection::Down; },
                    TurnDirection::None => { }
                }
            }
        }

        // println!("Turning {:?}, became {:?}", movement.turn, you.dir);

        // for i in 0..MAP_HEIGHT {
        //     for j in 0..MAP_LENGTH {
        //         if you.x == i && you.y == j {
        //             print!("*");
        //         } else {
        //             print!("{}", map[i][j]);
        //         }
        //     }
        //     println!("");
        // }
    }

    println!("Final password is {}", (you.x + 1) * 1000 + (you.y + 1) * 4 + you.dir as usize);
}
