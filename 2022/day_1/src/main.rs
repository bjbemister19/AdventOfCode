use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn part_1() {
    let file = File::open("data/input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut max = 0;
    let mut total = 0;
    for line in reader.lines() {
        let s = line.unwrap();
        let maybe_i = s.parse::<i32>();
        if maybe_i.is_ok() {
            let i = maybe_i.ok().unwrap();
            total = total + i;
        } else {
            if total > max {
                max = total;
            }
            total = 0;
        }
    }
    println!("Part 1: {}", max);
}

fn part_2() {
    let file = File::open("data/input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut totals = Vec::new();
    let mut per_elf_total = 0;
    for line in reader.lines() {
        let maybe_food_calories = line.unwrap().parse::<i32>();
        if maybe_food_calories.is_ok() {
            let food_calories = maybe_food_calories.ok().unwrap();
            per_elf_total = per_elf_total + food_calories;
        } else {
            totals.push(per_elf_total);
            per_elf_total = 0;
        }
    }

    totals.sort_by(|a, b| b.cmp(a));
    println!("{}", totals[0] + totals[1] + totals[2]);
}

fn main() {
    part_1();
    part_2();
}
