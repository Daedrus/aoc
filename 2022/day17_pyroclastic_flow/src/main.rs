use std::collections::HashMap;

// const INPUT: &str = include_str!("../input.example");
const INPUT: &str = include_str!("../input");

const MAX_HEIGHT: usize = 10000 * 4;

const WIDTH: usize = 7;

// Empirically determined, probably only works with the example input and my input
// Degenerate inputs (e.g. all vertical lines + all left pushes) will break this.
// I assume that a robust solution will check for collisions against the
// entire "contour" somehow.
const SAVED_ROWS: usize = 50;

// Part 1
// const TESTED_UNITS: u64 = 2022;
const TESTED_UNITS: u64 = 1_000_000_000_000;

#[derive(PartialEq, Debug, Eq, Hash, Copy, Clone)]
enum ShapeType {
    HorizontalLine,
    Cross,
    MirroredL,
    VerticalLine,
    Square
}

#[derive(Debug, Copy, Clone, Hash)]
enum JetDirection {
    Left,
    Right
}

#[derive(Debug)]
struct Shape {
    shape_type: ShapeType,
    position: (usize, usize)
}

// Use this as a key to find repeating patterns
#[derive(PartialEq, Eq, Hash, Debug)]
struct Pattern {
    shape_type: ShapeType,
    jet_index: usize,
    height_map: [usize; WIDTH]
}

fn spawn(current_spawn_index: &mut u32, position: (usize, usize)) -> Shape {
    if *current_spawn_index == 0 {
        /*
         * #@@@
         */
        *current_spawn_index = 1;
        Shape {
            shape_type: ShapeType::HorizontalLine,
            position: position
        }
    } else if *current_spawn_index == 1 {
        /*
         *  @
         * @#@
         *  @
         */
        *current_spawn_index = 2;
        Shape {
            shape_type: ShapeType::Cross,
            position: (position.0 + 1, position.1 + 1)
        }
    } else if *current_spawn_index == 2 {
        /*
         *   @
         *   @
         * #@@
         */
        *current_spawn_index = 3;
        Shape {
            shape_type: ShapeType::MirroredL,
            position: position
        }
    } else if *current_spawn_index == 3 {
        /*
         * @
         * @
         * @
         * #
         */
        *current_spawn_index = 4;
        Shape {
            shape_type: ShapeType::VerticalLine,
            position: position
        }
    } else if *current_spawn_index == 4 {
        /*
         * @@
         * #@
         */
        *current_spawn_index = 0;
        Shape {
            shape_type: ShapeType::Square,
            position: position
        }
    } else {
        unreachable!()
    }
}

fn show_board(highest_rock: usize, play_area: &[[char; WIDTH]; MAX_HEIGHT]) {
    for row in (1..=highest_rock).rev() {
        print!("{:<2} ", row);
        for column in 0..play_area[row].len() {
            print!("{}", play_area[row][column]);
        }
    }
    println!("   0123456");
}

// Move SAVED_ROWS from the top to the bottom of the board and clear the rest
// Do this so that we don't stack overflow
fn reset_board(highest_rock: &mut usize, play_area: &mut [[char; WIDTH]; MAX_HEIGHT]) {
    for i in 1..=SAVED_ROWS {
        for j in 0..play_area[i].len() {
            play_area[i][j] = play_area[*highest_rock-SAVED_ROWS+i][j];
        }
    }
    for i in SAVED_ROWS+1..play_area.len() {
        for j in 0..play_area[i].len() {
            play_area[i][j] = '.';
        }
    }

    *highest_rock = SAVED_ROWS;
}

// Once we have the same shape type, the same jet index and the same
// heights on each column then that is a cycle
fn check_for_cycle(
    highest_rock: usize,
    shape_type: ShapeType,
    shape_count: u64,
    jet_index: usize,
    patterns: &mut HashMap<Pattern, (u64, u64)>,
    play_area: &[[char; WIDTH]; MAX_HEIGHT],
    total_height: u64) -> (bool, (u64, u64))
{
    let mut height_map = [0; WIDTH];
    for j in 0..WIDTH {
        for i in (0..=highest_rock).rev() {
            if play_area[i][j] == '#' {
                height_map[j] = i;
                break;
            }
        }
    }

    let pattern = Pattern {
        shape_type: shape_type,
        jet_index: jet_index,
        height_map: height_map
    };

    if patterns.contains_key(&pattern) {
        let value = patterns.get(&pattern).unwrap();
        return (true, *value);
    } else {
        patterns.insert(pattern, (shape_count, total_height));
        return (false, (0,0));
    }
}

fn jet_push(shape: &mut Shape, direction: JetDirection, play_area: &[[char; WIDTH]; MAX_HEIGHT]) {
    match shape.shape_type {
        ShapeType::HorizontalLine => {
            match direction {
                JetDirection::Left => {
                    if shape.position.1 > 0 && play_area[shape.position.0][shape.position.1 - 1] != '#' {
                        shape.position.1 -= 1;
                    }
                },
                JetDirection::Right => {
                    if shape.position.1 < WIDTH - 4 && play_area[shape.position.0][shape.position.1 + 4] != '#' {
                        shape.position.1 += 1;
                    }
                }
            }
        },
        ShapeType::Cross => {
            match direction {
                JetDirection::Left => {
                    if shape.position.1 > 1 &&
                       play_area[shape.position.0][shape.position.1 - 2] != '#' &&
                       play_area[shape.position.0-1][shape.position.1 - 1] != '#' &&
                       play_area[shape.position.0+1][shape.position.1 - 1] != '#' {
                        shape.position.1 -= 1;
                    }
                },
                JetDirection::Right => {
                    if shape.position.1 < WIDTH - 2 &&
                       play_area[shape.position.0][shape.position.1 + 2] != '#' &&
                       play_area[shape.position.0-1][shape.position.1 + 1] != '#' &&
                       play_area[shape.position.0+1][shape.position.1 + 1] != '#' {
                        shape.position.1 += 1;
                    }
                }
            }
        },
        ShapeType::MirroredL => {
            match direction {
                JetDirection::Left => {
                    if shape.position.1 > 0 &&
                       play_area[shape.position.0][shape.position.1 - 1] != '#' &&
                       play_area[shape.position.0+1][shape.position.1 + 1] != '#' &&
                       play_area[shape.position.0+2][shape.position.1 + 1] != '#' {
                        shape.position.1 -= 1;
                    }
                },
                JetDirection::Right => {
                    if shape.position.1 < WIDTH - 3 &&
                       play_area[shape.position.0][shape.position.1 + 3] != '#' &&
                       play_area[shape.position.0+1][shape.position.1 + 3] != '#' &&
                       play_area[shape.position.0+2][shape.position.1 + 3] != '#' {
                        shape.position.1 += 1;
                    }
                }
            }
        },
        ShapeType::VerticalLine => {
            match direction {
                JetDirection::Left => {
                    if shape.position.1 > 0 &&
                        play_area[shape.position.0][shape.position.1 - 1] != '#' &&
                        play_area[shape.position.0 + 1][shape.position.1 - 1] != '#' &&
                        play_area[shape.position.0 + 2][shape.position.1 - 1] != '#' &&
                        play_area[shape.position.0 + 3][shape.position.1 - 1] != '#' {
                        shape.position.1 -= 1;
                    }
                },
                JetDirection::Right => {
                    if shape.position.1 < WIDTH - 1 &&
                        play_area[shape.position.0][shape.position.1 + 1] != '#' &&
                        play_area[shape.position.0 + 1][shape.position.1 + 1] != '#' &&
                        play_area[shape.position.0 + 2][shape.position.1 + 1] != '#' &&
                        play_area[shape.position.0 + 3][shape.position.1 + 1] != '#' {
                        shape.position.1 += 1;
                    }
                }
            }
        },
        ShapeType::Square => {
            match direction {
                JetDirection::Left => {
                    if shape.position.1 > 0 &&
                        play_area[shape.position.0][shape.position.1 - 1] != '#' &&
                        play_area[shape.position.0+1][shape.position.1 - 1] != '#' {
                        shape.position.1 -= 1;
                    }
                },
                JetDirection::Right => {
                    if shape.position.1 < WIDTH - 2 &&
                        play_area[shape.position.0][shape.position.1 + 2] != '#' &&
                        play_area[shape.position.0+1][shape.position.1 + 2] != '#' {
                        shape.position.1 += 1;
                    }
                }
            }
        }
    }
}


fn fall(shape: &mut Shape, highest_rock: &mut usize, play_area: &mut [[char; WIDTH]; MAX_HEIGHT]) -> bool {
    match shape.shape_type {
        ShapeType::HorizontalLine => {
            if shape.position.0 > 1 &&
               play_area[shape.position.0 - 1][shape.position.1] != '#' &&
               play_area[shape.position.0 - 1][shape.position.1 + 1] != '#' &&
               play_area[shape.position.0 - 1][shape.position.1 + 2] != '#' &&
               play_area[shape.position.0 - 1][shape.position.1 + 3] != '#' {
                shape.position.0 = shape.position.0 - 1;
                return true;
            } else {
                if *highest_rock <= shape.position.0 {
                    *highest_rock = shape.position.0;
                }
                play_area[shape.position.0][shape.position.1] = '#';
                play_area[shape.position.0][shape.position.1 + 1] = '#';
                play_area[shape.position.0][shape.position.1 + 2] = '#';
                play_area[shape.position.0][shape.position.1 + 3] = '#';
                return false;
            }
        },
        ShapeType::Cross => {
            if shape.position.0 > 2 &&
               play_area[shape.position.0 - 2][shape.position.1] != '#' &&
               play_area[shape.position.0 - 1][shape.position.1 - 1] != '#' &&
               play_area[shape.position.0 - 1][shape.position.1 + 1] != '#' {
                shape.position.0 = shape.position.0 - 1;
                return true;
            } else {
                if *highest_rock <= shape.position.0 + 1 {
                    *highest_rock = shape.position.0 + 1;
                }
                play_area[shape.position.0][shape.position.1] = '#';
                play_area[shape.position.0][shape.position.1 - 1] = '#';
                play_area[shape.position.0][shape.position.1 + 1] = '#';
                play_area[shape.position.0 - 1][shape.position.1] = '#';
                play_area[shape.position.0 + 1][shape.position.1] = '#';
                return false;
            }
        },
        ShapeType::MirroredL => {
            if shape.position.0 > 1 &&
               play_area[shape.position.0 - 1][shape.position.1] != '#' &&
               play_area[shape.position.0 - 1][shape.position.1 + 1] != '#' &&
               play_area[shape.position.0 - 1][shape.position.1 + 2] != '#' {
                shape.position.0 = shape.position.0 - 1;
                return true;
            } else {
                if *highest_rock <= shape.position.0 + 2 {
                    *highest_rock = shape.position.0 + 2;
                }
                play_area[shape.position.0][shape.position.1] = '#';
                play_area[shape.position.0][shape.position.1 + 1] = '#';
                play_area[shape.position.0][shape.position.1 + 2] = '#';
                play_area[shape.position.0+1][shape.position.1 + 2] = '#';
                play_area[shape.position.0+2][shape.position.1 + 2] = '#';
                return false;
            }
        },
        ShapeType::VerticalLine => {
            if shape.position.0 > 1 &&
               play_area[shape.position.0 - 1][shape.position.1] != '#' {
                shape.position.0 = shape.position.0 - 1;
                return true;
            } else {
                if *highest_rock <= shape.position.0 + 3 {
                    *highest_rock = shape.position.0 + 3;
                }
                play_area[shape.position.0][shape.position.1] = '#';
                play_area[shape.position.0+1][shape.position.1] = '#';
                play_area[shape.position.0+2][shape.position.1] = '#';
                play_area[shape.position.0+3][shape.position.1] = '#';
                return false;
            }
        },
        ShapeType::Square => {
            if shape.position.0 > 1 &&
               play_area[shape.position.0 - 1][shape.position.1] != '#' &&
               play_area[shape.position.0 - 1][shape.position.1 + 1] != '#' {
                shape.position.0 = shape.position.0 - 1;
                return true;
            } else {
                if *highest_rock <= shape.position.0 + 1 {
                    *highest_rock = shape.position.0 + 1;
                }
                play_area[shape.position.0][shape.position.1] = '#';
                play_area[shape.position.0][shape.position.1+1] = '#';
                play_area[shape.position.0+1][shape.position.1] = '#';
                play_area[shape.position.0+1][shape.position.1+1] = '#';
                return false;
            }
        }
    }
}


fn main() {
    let mut lines = INPUT.lines();

    let mut play_area = [['.'; WIDTH]; MAX_HEIGHT];
    let mut highest_rock = 0;
    let mut total_height: u64 = 0;
    let mut shape_falling;
    let mut shape_count: u64 = 0;
    let mut current_spawn_index = 0;
    let mut patterns: HashMap<Pattern, (u64, u64)> = HashMap::new();
    let mut reset_once = false;
    let mut found_cycle = false;
    let jet: Vec<JetDirection> = lines.nth(0).unwrap().chars().map(|direction| if direction == '<' { JetDirection::Left } else { JetDirection::Right }).collect();
    let mut current_jet_index = 0;

    let mut shape = spawn(&mut current_spawn_index, ((highest_rock + 4).try_into().unwrap(), 2));
    shape_falling = true;

    loop {
        if !shape_falling {
            shape = spawn(&mut current_spawn_index, ((highest_rock + 4).try_into().unwrap(), 2));
            shape_falling = true;
        }

        jet_push(&mut shape, jet[current_jet_index], &play_area);
        current_jet_index += 1;
        if current_jet_index == jet.len() {
            current_jet_index = 0;
        }

        if !fall(&mut shape, &mut highest_rock, &mut play_area) {
            shape_falling = false;
            shape_count += 1;

            let current_total: u64 = if !reset_once {
                highest_rock as u64
            } else {
                total_height + highest_rock as u64 - SAVED_ROWS as u64
            };

            // This found_cycle and cycle_found thing, KEKW
            if !found_cycle {
                let (cycle_found, cycle_start) = check_for_cycle(highest_rock, shape.shape_type, shape_count, current_jet_index, &mut patterns, &play_area, current_total);

                // Once we found a cycle start "eating away" at the shapes without actually
                // processing them. Stop once there aren't enough shapes to fill a cycle and
                // resume normal processing for them.
                if cycle_found {
                    found_cycle = true;
                    let cycle_length = current_total - cycle_start.1;
                    let cycle_shapes = shape_count - cycle_start.0;
                    loop {
                        if TESTED_UNITS > shape_count + cycle_shapes {
                            shape_count += cycle_shapes;
                            total_height += cycle_length;
                        } else {
                            break;
                        }
                    }
                }
            }

            // Keep the board from growing too much, we'll stack overflow otherwise
            if MAX_HEIGHT - highest_rock <= 20 {
                // This total_height + reset logic is present in multiple places and it is
                // super ugly. When resetting the board we keep SAVED_ROWS rows and we have
                // to subtract those from the total height. Too lazy to fix.
                if !reset_once {
                    total_height = highest_rock as u64;
                } else {
                    total_height += highest_rock as u64 - SAVED_ROWS as u64;
                }
                reset_once = true;
                reset_board(&mut highest_rock, &mut play_area);
            }

            if shape_count == TESTED_UNITS {
                if !reset_once {
                    total_height += highest_rock as u64;
                } else {
                    total_height += highest_rock as u64 - SAVED_ROWS as u64;
                }
                break;
            }
        }
    }

    println!("{}", total_height);

    //show_board(highest_rock, &play_area);
}
