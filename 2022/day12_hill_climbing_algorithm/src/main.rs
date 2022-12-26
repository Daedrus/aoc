use std::fs::File;
use std::io::{BufRead, BufReader};
use std::env;
use std::collections::VecDeque;

#[derive(Debug)]
struct Node {
    id: char,
    visited: bool,
    parent: (usize, usize)
}

fn shortest_path_length(start_node: (usize, usize), end_node: (usize, usize), arr: &mut Vec<Vec<Node>>) -> u32 {
    let mut q: VecDeque<(usize, usize)> = VecDeque::new();

    arr[start_node.0][start_node.1].visited = true;
    q.push_back(start_node);
    while !q.is_empty() {
        // println!{"Queue is {:?}", q};

        let v = q.pop_front().unwrap();

        // println!{"Visiting {:?}", v};

        if v == end_node {
            break;
        }

        let mut neighbors: Vec<(usize, usize)> = Vec::new();
        // top neighbor
        if v.0 > 0 && arr[v.0][v.1].id as u32 + 1 >= arr[v.0-1][v.1].id as u32 {
            neighbors.push((v.0-1, v.1));
        }
        // left neighbor
        if v.1 > 0 && arr[v.0][v.1].id as u32 + 1 >= arr[v.0][v.1-1].id as u32 {
            neighbors.push((v.0, v.1-1));
        }
        // right neighbor
        if v.1 < arr[v.0].len()-1 && arr[v.0][v.1].id as u32 + 1 >= arr[v.0][v.1+1].id as u32 {
            neighbors.push((v.0, v.1+1));
        }
        // bottom neighbor
        if v.0 < arr.len()-1 && arr[v.0][v.1].id as u32 + 1 >= arr[v.0+1][v.1].id as u32 {
            neighbors.push((v.0+1, v.1));
        }

        // println!{"Neighbors of {:?} are {:?}", v, neighbors};

        for neighbor in neighbors {
            if !arr[neighbor.0][neighbor.1].visited {
                // println!{"Mark {:?} as visited, it has elevation {}", neighbor, arr[neighbor.0][neighbor.1].id};
                arr[neighbor.0][neighbor.1].visited = true;
                arr[neighbor.0][neighbor.1].parent = v;
                q.push_back((neighbor.0, neighbor.1));
            }
        }

        // for i in 0..arr.len() {
        //     for j in 0..arr[i].len() {
        //         if arr[i][j].visited {
        //             print!("V");
        //         } else {
        //             print!(".");
        //         }
        //     }
        //     println!();
        // }
    }

    let mut curr_node = end_node;
    let mut path_length: u32 = 0;
    // Too lazy to detect the case where there is no path between start and end so do this hack
    while curr_node != start_node && path_length < (arr.len() as u32 * arr[0].len() as u32 + 1) {
        curr_node = arr[curr_node.0][curr_node.1].parent;
        path_length += 1;
    }

    for i in 0..arr.len() {
        for j in 0..arr[i].len() {
            arr[i][j].visited = false;
            arr[i][j].parent = (0,0);
        }
    }

    // println!{"Ran BFS from {:?}, path length is {:?}", start_node, path_length};

    path_length
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let f = BufReader::new(File::open(&args[1]).unwrap());

    let mut arr: Vec<Vec<Node>> = f.lines()
        .map(|l| l.unwrap().chars()
             .map(|id| Node { id: id, visited: false, parent: (0,0) })
             .collect())
        .collect();

    let mut start_node: (usize, usize) = (0,0); 
    let mut end_node: (usize, usize) = (0,0); 

    // It should be possible to do this in the .map(s) above somehow
    for i in 0..arr.len() {
        for j in 0..arr[i].len() {
            if arr[i][j].id == 'S' {
                start_node = (i,j);
                arr[i][j].id = 'a';
            }
            if arr[i][j].id == 'E' {
                end_node = (i,j);
                arr[i][j].id = 'z';
            }
        }
    }

    // Part 1
    // let path_length = shortest_path_length(start_node, end_node, &mut arr);
    // println!{"{:?}", path_length};

    let mut min_path_length: u32 = arr.len() as u32 * arr[0].len() as u32 + 1;

    // Again, too lazy, just re-apply the same solution on all 'a' nodes
    for i in 0..arr.len() {
        for j in 0..arr[i].len() {
            if arr[i][j].id == 'a' {
                let path_length = shortest_path_length((i,j), end_node, &mut arr);
                if path_length < min_path_length {
                    min_path_length = path_length;
                }
            }
        }
    }

    println!{"{:?}", min_path_length};

}
