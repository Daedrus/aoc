use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::env;
use regex::Regex;

fn draw_crt(crt: &mut [[char; 40]; 6], cycle: u32, sprite_pos: i32) {
    let current_row: usize = (cycle-1) as usize / 40;
    let current_column: usize = (cycle-1) as usize % 40;
    if (sprite_pos - current_column as i32).abs() <= 1 {
        crt[current_row][current_column] = '#';
    }
}

fn compute_signal_strength(signal_strength_sum: &mut i32, cycle: u32, x_value: i32) {
    if (cycle == 20) || (cycle == 60) || (cycle == 100) || (cycle == 140) || (cycle == 180) || (cycle == 220) {
        *signal_strength_sum += cycle as i32 * x_value;
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let cmd_noop = Regex::new(r"noop").unwrap();
    let cmd_addx = Regex::new(r"addx (-?\d+)").unwrap();

    let mut cycles: u32 = 0;
    let mut x_value: i32 = 1;
    let mut signal_strength_sum: i32 = 0;
    let mut crt: [[char; 40]; 6] =  [['.'; 40]; 6];

    if let Ok(lines) = read_lines(Path::new(&args[1])) {
        for line in lines {
            if let Ok(terminal_line) = line {
                if cmd_noop.is_match(&terminal_line) {
                    cycles += 1;
                    compute_signal_strength(&mut signal_strength_sum, cycles, x_value);
                    draw_crt(&mut crt, cycles, x_value);
                } else if cmd_addx.is_match(&terminal_line) {
                    let result = cmd_addx.captures_iter(&terminal_line).nth(0).unwrap();
                    let value = result[1].parse::<i32>().unwrap();

                    for addx_cycle in 0..2 {
                        cycles += 1;
                        compute_signal_strength(&mut signal_strength_sum, cycles, x_value);
                        draw_crt(&mut crt, cycles, x_value);
                        if addx_cycle == 1 {
                            x_value += value;
                        }
                    }
                } else {
                    println!("PROBLEM");
                }
            }
        }
    }

    println!("{:?}", signal_strength_sum);
    for row in 0..crt.len() {
        for line in 0..crt[row].len() {
            print!("{}", crt[row][line]);
        }
        println!();
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
