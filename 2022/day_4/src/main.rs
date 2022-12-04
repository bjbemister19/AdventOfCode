use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug, Copy, Clone)]
struct Pair {
    begin: i32,
    end: i32,
}

fn is_fully_contained(range1: Pair, range2: Pair) -> bool {
    if (range1.begin <= range2.begin && range1.end >= range2.end)
        || (range2.begin <= range1.begin && range2.end >= range1.end)
    {
        true
    } else {
        false
    }
}

fn has_any_overlap(range1: Pair, range2: Pair) -> bool {
    if (range1.begin < range2.begin && range1.end < range2.begin)
        || (range1.begin > range2.end && range1.end > range2.end)
    {
        false
    } else {
        true
    }
}

fn main() {
    let file = File::open("data/input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut p1_count = 0;
    let mut p2_count = 0;

    for line in reader.lines() {
        let raw = line.unwrap();
        let pairs: Vec<Pair> = raw
            .split(",")
            .map(|s| {
                let range: Vec<&str> = s.split("-").collect();
                Pair {
                    begin: range[0].parse().unwrap(),
                    end: range[1].parse().unwrap(),
                }
            })
            .collect();
        if is_fully_contained(pairs[0], pairs[1]) {
            p1_count = p1_count + 1;
        }
        if has_any_overlap(pairs[0], pairs[1]) {
            p2_count = p2_count + 1;
        }
    }

    println!("Result P1: {}", p1_count);
    println!("Result P2: {}", p2_count);
}
