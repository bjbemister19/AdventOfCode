use std::{io::{BufRead, BufReader}, fs::File};

#[derive(Debug, Copy, Clone)]
struct Pair {
    begin: i32,
    end: i32
}

fn is_fully_contained(range1: Pair, range2: Pair) -> bool{
    if (range1.begin <= range2.begin && range1.end >= range2.end) || (range2.begin <= range1.begin && range2.end >= range1.end) {
        true
    } else {
        false
    }
}

fn has_any_overlap(range1: Pair, range2: Pair) -> bool {
    if (range1.begin < range2.begin && range1.end < range2.begin) || (range1.begin > range2.end && range1.end > range2.end) {
        false
    } else {
        true
    }
}

fn part_1() {
    let file = File::open("data/input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut count = 0;

    for line in reader.lines() {
        let raw = line.unwrap();
        let pairs: Vec<Pair> = raw.split(",").map(|s| {
            let range: Vec<&str> = s.split("-").collect();
            Pair{begin: range[0].parse().unwrap(), end: range[1].parse().unwrap()}
        }).collect();
        if is_fully_contained(pairs[0], pairs[1]) {
            count = count + 1;
        }
    }

    println!("Result P1: {}", count);
}

fn part_2() {
    let file = File::open("data/input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut count = 0;

    for line in reader.lines() {
        let raw = line.unwrap();
        let pairs: Vec<Pair> = raw.split(",").map(|s| {
            let range: Vec<&str> = s.split("-").collect();
            Pair{begin: range[0].parse().unwrap(), end: range[1].parse().unwrap()}
        }).collect();
        if has_any_overlap(pairs[0], pairs[1]) {
            count = count + 1;
        }
    }

    println!("Result P2: {}", count);
}

fn main() {
    part_1();
    part_2();
}
