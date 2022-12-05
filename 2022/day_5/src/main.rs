use std::{
    fs::File,
    io::{BufRead, BufReader},
};

mod stacks;

fn move_boxes_p1(stacks: &mut Vec<Vec<char>>, qty: usize, from: usize, to: usize) {
    // println!("move {} from {} to {}", qty, from, to);
    for _i in 0..qty {
        let bx = stacks[from - 1].pop();
        if !bx.is_none() {
            stacks[to - 1].push(bx.unwrap());
        }
    }
}

fn move_boxes_p2(stacks: &mut Vec<Vec<char>>, qty: usize, from: usize, to: usize) {
    // println!("move {} from {} to {}", qty, from, to);

    let mut temp_stack = Vec::new();
    for _i in 0..qty {
        let bx = stacks[from - 1].pop();
        if !bx.is_none() {
            temp_stack.push(bx.unwrap());
        }
    }

    for _i in 0..temp_stack.len() {
        let bx = temp_stack.pop();
        stacks[to - 1].push(bx.unwrap());
    }
}

fn main() {
    let mut stacks_p1 = stacks::read("data/Cargo.txt");
    let mut stacks_p2 = stacks::read("data/Cargo.txt");

    let file = File::open("data/Cargo.txt").unwrap();
    let reader = BufReader::new(&file);

    for maybe_line in reader.lines() {
        let line = maybe_line.unwrap();
        if line.starts_with("move") {
            let command: Vec<&str> = line.split(" ").collect();
            assert!(command.len() == 6);
            move_boxes_p1(
                &mut stacks_p1,
                command[1].parse().unwrap(),
                command[3].parse().unwrap(),
                command[5].parse().unwrap(),
            );
            move_boxes_p2(
                &mut stacks_p2,
                command[1].parse().unwrap(),
                command[3].parse().unwrap(),
                command[5].parse().unwrap(),
            );
        }
    }

    println!("Part 1 Results");
    for s in stacks_p1 {
        println!("{:?}", s);
    }

    println!("Part 2 Results");
    for s in stacks_p2 {
        println!("{:?}", s);
    }
}
