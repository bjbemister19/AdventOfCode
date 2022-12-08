use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

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

fn is_file(input: &Vec<&str>) -> Option<bool> {
    if input.len() < 1 {
        return None;
    }
    Some(is_numeric(input[0]))
}

fn is_command(input: &Vec<&str>) -> Option<bool> {
    if input.len() < 1 {
        return None;
    }
    if input[0] == "$" {
        Some(true)
    } else {
        Some(false)
    }
}

fn is_exiting_directory(input: &Vec<&str>) -> bool {
    if input.len() > 2 && input[1] == "cd" && input[2] == ".." {
        true
    } else {
        false
    }
}

fn is_entering_directory(input: &Vec<&str>) -> bool {
    if input.len() > 1 && input[1] == "cd" {
        true
    } else {
        false
    }
}

fn pop_and_accumulate(stack: &mut Vec<Directory>, directory_sizes: &mut Vec<Directory>)  -> Option<()> {
    let top = stack.len() - 1;
    let size = stack[top].size;
    directory_sizes.push(stack.pop()?);
    stack[top-1].size = stack[top-1].size + size;
    Some(())
}

fn accumulate_size(stack: &mut Vec<Directory>, input: &Vec<&str>) -> Option<()> {
    let end = stack.len() - 1;
    stack[end].size = stack[end].size + input[0].parse::<usize>().ok()?;
    Some(())
}

fn accumulate_directory_sizes(reader: BufReader<File>) -> Result<Vec<Directory>, Box<dyn std::error::Error>> {
    let mut stack: Vec<Directory> = Vec::new();
    let mut directory_sizes: Vec<Directory> = Vec::new();

    for line_result in reader.lines() {
        let line = line_result?;
        let input = line.split(" ").collect::<Vec<&str>>();

        if is_command(&input).ok_or("Invalid input")? {
            if is_exiting_directory(&input) {
                pop_and_accumulate(&mut stack, &mut directory_sizes).ok_or("Invalid command, nothing to pop")?;
            } else if is_entering_directory(&input) {
                stack.push(Directory {
                    name: String::from(input[2]),
                    size: 0,
                });
            }
        } else if is_file(&input).ok_or("Invalid input")? {
            accumulate_size(&mut stack, &input).ok_or("Failed to parse size")?;
        }
    }

    while stack.len() > 1 {
        pop_and_accumulate(&mut stack, &mut directory_sizes).ok_or("Nothing to pop")?;
    }
    directory_sizes.push(stack.pop().ok_or("Nothing to pop")?);

    Ok(directory_sizes)
}

fn p1(directory_sizes: &Vec<Directory>) -> usize {
    let mut total = 0;
    for dir in directory_sizes {
        if dir.size <= 100000 {
            total = total + dir.size;
        }
    }
    total
}

fn p2(directory_sizes: &Vec<Directory>) -> Option<usize> {
    let total_disk = 70000000;
    let required_unused = 30000000;
    let required = total_disk - required_unused;
    let root = directory_sizes.iter().find(|dir| dir.name == "/");
    let used = root?.size;
    let min_need_to_free = used - required;

    let mut min_dir = Directory { name: String::from(""), size: total_disk };
    for dir in directory_sizes {
        if dir.size >= min_need_to_free && dir.size < min_dir.size {
            min_dir = dir.clone();
        }
    }
    //println!("{:?}", min_dir);
    Some(min_dir.size)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("data/input.txt")?;
    let reader = BufReader::new(file);

    let directory_sizes = accumulate_directory_sizes(reader)?;

    println!("P1 total: {}", p1(&directory_sizes));
    println!("P2 size of dir to be deleted {}", p2(&directory_sizes).ok_or("Failed to find root directory")?);

    Ok(())
}
