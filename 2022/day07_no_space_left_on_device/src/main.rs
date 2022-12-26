use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::env;
use id_tree::*;
use id_tree::InsertBehavior::*;
use regex::Regex;
use std::rc::Rc;

#[derive(Debug)]
enum FileSystemEntryType {
    File,
    Directory,
}

#[derive(Debug)]
struct FileSystemEntry {
    fs_type: FileSystemEntryType,
    name: String,
    size: u32,
}

fn traverse (tree: &Tree<FileSystemEntry>, node: &NodeId, sum_of_sizes: &mut u32) -> u32 {
    let mut node_size:u32 = 0;
    for child in tree.get(&node).unwrap().children() {
        let node_data = tree.get(child).unwrap().data();
        match node_data.fs_type {
            FileSystemEntryType::Directory => { node_size += traverse(tree, child, sum_of_sizes) },
            FileSystemEntryType::File => { node_size += node_data.size },
        }
    }

    if node_size < 100000 {
        *sum_of_sizes += node_size;
    }

    return node_size;
}

fn traverse2 (tree: &Tree<FileSystemEntry>, node: &NodeId, needed_space:u32, min_size: &mut u32) -> u32 {
    let mut node_size:u32 = 0;
    for child in tree.get(&node).unwrap().children() {
        let node_data = tree.get(child).unwrap().data();
        match node_data.fs_type {
            FileSystemEntryType::Directory => { node_size += traverse2(tree, child, needed_space, min_size) },
            FileSystemEntryType::File => { node_size += node_data.size },
        }
    }

    if node_size > needed_space {
        if node_size < *min_size {
            *min_size = node_size;
        }
    }

    return node_size;
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut tree: Tree<FileSystemEntry> = TreeBuilder::new().build();

    let cmd_cd_dir = Regex::new(r"\$ cd ([a-z]+)").unwrap();
    let cmd_cd_up = Regex::new(r"\$ cd \.\.").unwrap();
    let cmd_cd_root = Regex::new(r"\$ cd /").unwrap();
    let cmd_ls = Regex::new(r"\$ ls").unwrap();
    let result_dir = Regex::new(r"dir ([a-z]+)").unwrap();
    let result_file = Regex::new(r"(\d+) ([a-z\.]+)").unwrap();

    let root_id = Rc::new(tree.insert(Node::new(FileSystemEntry { fs_type: FileSystemEntryType::Directory, name: "/".to_string(), size: 0 }), AsRoot).unwrap());
    let mut current_node = root_id.clone();

    if let Ok(lines) = read_lines(Path::new(&args[1])) {
        for line in lines {
            if let Ok(terminal_line) = line {
                if cmd_cd_dir.is_match(&terminal_line) {
                    let cmd = cmd_cd_dir.captures_iter(&terminal_line).nth(0).unwrap();
                    for child in tree.get(&current_node).unwrap().children() {
                        if cmd[1].eq(tree.get(child).unwrap().data().name.as_str()) {
                            current_node = Rc::new(child.clone());
                            break;
                        }
                    };
                } else if cmd_cd_up.is_match(&terminal_line) {
                    current_node = Rc::new(tree.get(&current_node).unwrap().parent().unwrap().clone());
                } else if cmd_cd_root.is_match(&terminal_line) {
                } else if cmd_ls.is_match(&terminal_line) {
                } else if result_dir.is_match(&terminal_line) {
                    let result = result_dir.captures_iter(&terminal_line).nth(0).unwrap();
                    tree.insert(Node::new(FileSystemEntry { fs_type: FileSystemEntryType::Directory, name: result[1].to_string(), size: 0 }), UnderNode(&current_node)).unwrap();
                } else if result_file.is_match(&terminal_line) {
                    let result = result_file.captures_iter(&terminal_line).nth(0).unwrap();
                    let size = result[1].parse::<u32>().unwrap();
                    tree.insert(Node::new(FileSystemEntry { fs_type: FileSystemEntryType::File, name: result[2].to_string(), size: size }), UnderNode(&current_node)).unwrap();
                } else {
                }
            }
        }
    }

    let mut sum_of_sizes:u32 = 0;
    let root_dir_size = traverse(&tree, &root_id, &mut sum_of_sizes);

    println!("{}", sum_of_sizes);
    // println!("{}", root_dir_size);

    let currently_unused_space = 70000000 - root_dir_size;

    // println!("{}", currently_unused_space);

    let mut min_size:u32 = 70000000;
    traverse2(&tree, &root_id, 30000000 - currently_unused_space, &mut min_size);

    println!("{}", min_size);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
