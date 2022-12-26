use regex::Regex;

// const INPUT: &str = include_str!("../input.example");
// const INPUT: &str = include_str!("../input.minimal");
const INPUT: &str = include_str!("../input");

const MAX_SIZE: usize = 22;

fn flood_fill((x, y, z): (usize, usize, usize), cube: &[[[u32; MAX_SIZE]; MAX_SIZE]; MAX_SIZE], visited: &mut[[[u32; MAX_SIZE]; MAX_SIZE]; MAX_SIZE]) -> u32 {
    let mut touched_sides = 0;

    if cube[x][y][z] == 1 || visited[x][y][z] == 1 {
        return 0
    }

    if x > 0 && cube[x-1][y][z] == 1 {
        touched_sides += 1;
    }
    if x < MAX_SIZE - 1 && cube[x+1][y][z] == 1 {
        touched_sides += 1;
    }

    if y > 0 && cube[x][y-1][z] == 1 {
        touched_sides += 1;
    }
    if y < MAX_SIZE - 1 && cube[x][y+1][z] == 1 {
        touched_sides += 1;
    }

    if z > 0 && cube[x][y][z-1] == 1 {
        touched_sides += 1;
    }
    if z < MAX_SIZE - 1 && cube[x][y][z+1] == 1 {
        touched_sides += 1;
    }

    visited[x][y][z] = 1;

    if x > 0 {
        touched_sides += flood_fill((x-1,y,z), &cube, visited);
    }
    if x < MAX_SIZE - 1 {
        touched_sides += flood_fill((x+1,y,z), &cube, visited);
    }
    if y > 0 {
        touched_sides += flood_fill((x,y-1,z), &cube, visited);
    }
    if y < MAX_SIZE - 1 {
        touched_sides += flood_fill((x,y+1,z), &cube, visited);
    }
    if z > 0 {
        touched_sides += flood_fill((x,y,z-1), &cube, visited);
    }
    if z < MAX_SIZE - 1 {
        touched_sides += flood_fill((x,y,z+1), &cube, visited);
    }

    touched_sides
}


fn surface_area(cube: &[[[u32; MAX_SIZE]; MAX_SIZE]; MAX_SIZE]) -> u32 {
    let mut unconnected_sides = 0;

    for x in 0..MAX_SIZE {
        for y in 0..MAX_SIZE {
            for z in 0..MAX_SIZE {
                if cube[x][y][z] == 1 {
                    if x == 0 {
                        unconnected_sides += 1;
                    }
                    if x > 0 && cube[x-1][y][z] == 0 {
                        unconnected_sides += 1;
                    }
                    if x == MAX_SIZE - 1 {
                        unconnected_sides += 1;
                    }
                    if x < MAX_SIZE - 1 && cube[x+1][y][z] == 0 {
                        unconnected_sides += 1;
                    }

                    if y == 0 {
                        unconnected_sides += 1;
                    }
                    if y > 0 && cube[x][y-1][z] == 0 {
                        unconnected_sides += 1;
                    }
                    if y == MAX_SIZE - 1 {
                        unconnected_sides += 1;
                    }
                    if y < MAX_SIZE - 1 && cube[x][y+1][z] == 0 {
                        unconnected_sides += 1;
                    }

                    if z == 0 {
                        unconnected_sides += 1;
                    }
                    if z > 0 && cube[x][y][z-1] == 0 {
                        unconnected_sides += 1;
                    }
                    if z == MAX_SIZE - 1 {
                        unconnected_sides += 1;
                    }
                    if z < MAX_SIZE - 1 && cube[x][y][z+1] == 0 {
                        unconnected_sides += 1;
                    }
                }
            }
        }
    }

    unconnected_sides
}

fn main() {
    let mut cube = [[[0; MAX_SIZE]; MAX_SIZE]; MAX_SIZE];
    let mut visited = [[[0; MAX_SIZE]; MAX_SIZE]; MAX_SIZE];
    let coord_regex = Regex::new(r"(\d+),(\d+),(\d+)").unwrap();
    let mut lines = INPUT.lines();

    while let Some(line) = lines.next() {
        let result = coord_regex.captures_iter(&line).nth(0).unwrap();
        let x = result[1].parse::<usize>().unwrap();
        let y = result[2].parse::<usize>().unwrap();
        let z = result[3].parse::<usize>().unwrap();
        // Min coordinate is 0 and max is 19 (in my input)
        // In order for flood_fill to work, leave a layer of unoccupied
        // spaces before 0 and after 19 (on all axes). This is why MAX_SIZE
        // is 22 and why we shift all coordinates by 1.
        cube[x+1][y+1][z+1] = 1;
    }

    println!("{}", surface_area(&cube));
    println!("{}", flood_fill((0,0,0), &cube, &mut visited));
}
