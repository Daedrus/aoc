use std::fs::File;
use std::io::{BufRead, BufReader};
use std::env;

#[derive(Debug)]
struct Tree {
    height: i8,
    visible: bool
}

fn calculate_scenic_score(arr: &mut Vec<Vec<Tree>>, tree_x: usize, tree_y: usize) -> u32 {
    let mut left_view:u32 = 0;
    let mut right_view:u32 = 0;
    let mut up_view:u32 = 0;
    let mut down_view:u32 = 0;

    if tree_x == 0 || tree_y == 0 || tree_x == (arr.len()-1) || (tree_y == arr.len()-1) {
        return 0;
    } else {
        for j in (0..tree_y).rev() {
            if arr[tree_x][j].height < arr[tree_x][tree_y].height {
                left_view += 1;
            } else {
                left_view += 1;
                break;
            }
        }

        for j in (tree_y+1)..arr.len() {
            if arr[tree_x][j].height < arr[tree_x][tree_y].height {
                right_view += 1;
            } else {
                right_view += 1;
                break;
            }
        }

        for i in (0..tree_x).rev() {
            if arr[i][tree_y].height < arr[tree_x][tree_y].height {
                up_view += 1;
            } else {
                up_view += 1;
                break;
            }
        }

        for i in (tree_x+1)..arr.len() {
            if arr[i][tree_y].height < arr[tree_x][tree_y].height {
                down_view += 1;
            } else {
                down_view += 1;
                break;
            }
        }

        // println!("left_view of {} {} is {}", tree_x, tree_y, left_view);
        // println!("right_view of {} {} is {}", tree_x, tree_y, right_view);
        // println!("up_view of {} {} is {}", tree_x, tree_y, up_view);
        // println!("down_view of {} {} is {}", tree_x, tree_y, down_view);
        return left_view * right_view * up_view * down_view;
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let f = BufReader::new(File::open(&args[1]).unwrap());

    let mut arr: Vec<Vec<Tree>> = f.lines()
        .map(|l| l.unwrap().chars()
             .map(|number| Tree { height: number.to_digit(10).unwrap() as i8, visible: false })
             .collect())
        .collect();

    // Go through the rows back and forth
    for i in 0..arr.len() {
        let mut max_visible: i8 = -1;
        for j in 0..arr[i].len() {
            if arr[i][j].height > max_visible {
                arr[i][j].visible = true;
                max_visible = arr[i][j].height;
            }
        }

        max_visible = -1;
        for j in (0..arr[i].len()).rev() {
            if arr[i][j].height > max_visible {
                arr[i][j].visible = true;
                max_visible = arr[i][j].height;
            }
        }
    }

    // Go through the columns back and forth (take advantage of the square matrix)
    for i in 0..arr.len() {
        let mut max_visible: i8 = -1;
        for j in 0..arr[i].len() {
            if arr[j][i].height > max_visible {
                arr[j][i].visible = true;
                max_visible = arr[j][i].height;
            }
        }

        max_visible = -1;
        for j in (0..arr[i].len()).rev() {
            if arr[j][i].height > max_visible {
                arr[j][i].visible = true;
                max_visible = arr[j][i].height;
            }
        }
    }

    let mut visible_trees = 0;
    let mut max_scenic_score: u32 = 0;
    for i in 0..arr.len() {
        for j in 0..arr[i].len() {
            if arr[i][j].visible {
                visible_trees += 1;
            }
            let scenic_score = calculate_scenic_score(&mut arr, i, j);
            // println!("scenic score of {} {} is {}", i, j, scenic_score);
            // println!("");
            if scenic_score > max_scenic_score {
                max_scenic_score = scenic_score;
            }
        }
    }

    println!("{}", visible_trees);
    println!("{}", max_scenic_score);
}
