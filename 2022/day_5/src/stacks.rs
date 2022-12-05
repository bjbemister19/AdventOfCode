use std::{
    fs::File,
    io::{BufRead, BufReader},
};

// Is there a better way to do this in rust?
fn split_string_by(input_string: &str, num_chars: i32) -> Vec<String> {
    let mut s = String::new();
    let mut v: Vec<String> = Vec::new();
    let mut i = 0;
    for c in input_string.chars() {
        s.push(c);
        if i == (num_chars - 1) {
            i = 0;
            v.push(s);
            s = String::new();
        } else {
            i = i + 1;
        }
    }
    if !s.is_empty() {
        v.push(s);
    }
    v
}

fn num_stacks(path: &str) -> usize {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(&file);

    let line = reader.lines().next().unwrap().unwrap();
    let v = split_string_by(&line, 4);
    v.len()
}

fn extract_char(s: &str) -> Option<char> {
    if s.starts_with("[") {
        let vec_chars = s.strip_prefix("[").unwrap().chars().collect::<Vec<char>>();
        Some(vec_chars[0])
    } else {
        None
    }
}

pub fn read(path: &str) -> Vec<Vec<char>> {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(&file);

    let num_stacks = num_stacks(path);
    let mut stacks: Vec<Vec<char>> = Vec::with_capacity(num_stacks);
    for _i in 0..num_stacks {
        stacks.push(Vec::new());
    }

    for maybe_line in reader.lines() {
        let line = maybe_line.unwrap();
        if line == "" {
            break;
        }
        let v = split_string_by(&line, 4);
        let mut count = 0;
        for el in v {
            let maybe_char = extract_char(&el);
            if !maybe_char.is_none() {
                stacks[count].insert(0, maybe_char.unwrap());
            }
            count = count + 1;
        }
    }

    stacks
}
