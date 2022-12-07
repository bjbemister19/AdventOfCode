/*
 * This code needs to be cleaned up so badly before I share it, but it works!
 */

use std::fs;
use std::io::BufRead;
use std::io::BufReader;

// mod filesystem;
// use filesystem::INode;

#[derive(Debug, Clone)]
struct Directory {
    name: String,
    size: usize,
}

fn is_numeric(s: &str) -> bool {
    for c in s.chars() {
        if !c.is_numeric() {
            return false;
        }
    }
    true
}

fn main() {
    let file = fs::File::open("data/input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut stack: Vec<Directory> = Vec::new();

    let mut dirs: Vec<Directory> = Vec::new();

    for maybeLine in reader.lines() {
        let line = maybeLine.unwrap();
        let input = line.split(" ").collect::<Vec<&str>>();

        if input.len() < 2 {
            println!("Invalid input");
        } else if input[0] == "$" {
            if input[1] == "cd" && input[2] == ".." {
                let mut end = stack.len() - 1;
                let size = stack[end].size;
                dirs.push(stack.pop().unwrap());
                end = stack.len() - 1;
                stack[end] = Directory {
                    name: stack[end].name.clone(),
                    size: stack[end].size + size,
                };
            } else if input[1] == "cd" {
                stack.push(Directory {
                    name: String::from(input[2]),
                    size: 0,
                });
            }
        } else if is_numeric(input[0]) {
            let end = stack.len() - 1;
            stack[end] = Directory {
                name: stack[end].name.clone(),
                size: stack[end].size + input[0].parse::<usize>().unwrap(),
            };
        }
        // else if input[0] == "dir" {
        //     todo!();
        // }
    }

    while stack.len() > 0 {
        let mut end = stack.len() - 1;
        let size = stack[end].size;
        dirs.push(stack.pop().unwrap());
        if end > 0{
            end = stack.len() - 1;
            stack[end] = Directory {
                name: stack[end].name.clone(),
                size: stack[end].size + size,
            };
        }
    }

    let mut total = 0;
    for dir in &dirs {
        if dir.size <= 100000 {
            println!("dirname: {:?} size: {:?}", dir.name, dir.size);
            total = total + dir.size;
        }
    }
    println!("P1 total: {}", total);
    let total_disk = 70000000;
    let required_unused = 30000000;
    let required = total_disk - required_unused;
    let root = &dirs.iter().find(|dir| dir.name == "/");
    let used = root.unwrap().size;
    let min_need_to_free = used - required;

    println!("total: {} required_unused: {} required:{} used: {} min: {} test {}", total_disk, required_unused, required, used, min_need_to_free, used - min_need_to_free);

    let mut min_dir = Directory { name: String::from(""), size: total_disk };
    for dir in &dirs {
        if dir.size >= min_need_to_free && dir.size < min_dir.size {
            min_dir = dir.clone();
        }
    }

    println!("{:?}", min_dir);
}
