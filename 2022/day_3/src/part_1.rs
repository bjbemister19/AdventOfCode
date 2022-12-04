use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug)]
struct Rugsack {
    compartment_1: String,
    compartment_2: String,
}

fn find_common_char(rugsack: Rugsack) -> char {
    for c in rugsack.compartment_1.chars() {
        for h in rugsack.compartment_2.chars() {
            if c == h {
                return c;
            }
        }
    }
    panic!()
}

fn get_char_priority(c: char) -> u32 {
    if c.is_lowercase() {
        c as u32 - 'a' as u32 + 1
    } else {
        c as u32 - 'A' as u32 + 27
    }
}

pub fn run() {
    let file = File::open("data/input.txt").unwrap();
    let reader = BufReader::new(file);

    let rugsacks: Vec<Rugsack> = reader
        .lines()
        .map(|line| {
            let rugsack = line.unwrap();
            let char_count = rugsack.chars().count();
            assert!(char_count % 2 == 0);

            Rugsack {
                compartment_1: rugsack[0..char_count / 2].to_string(),
                compartment_2: rugsack[char_count / 2..char_count].to_string(),
            }
        })
        .collect();

    let total: Option<u32> = rugsacks
        .into_iter()
        .map(|rugsack| get_char_priority(find_common_char(rugsack)))
        .reduce(|p1, p2| p1 + p2);
    println!("Result P1: {}", total.unwrap());
}
