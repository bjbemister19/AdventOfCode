use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn common_all_rugsacks(group: Vec<String>) -> char {
    for c0 in group[0].chars() {
        for c1 in group[1].chars() {
            if c0 == c1 {
                for c2 in group[2].chars() {
                    if c1 == c2 {
                        return c0;
                    }
                }
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

    let mut group = Vec::<String>::new();
    let mut all_groups = Vec::<Vec<String>>::new();

    let mut count = 0;
    for line in reader.lines() {
        group.push(line.unwrap());
        count = count + 1;

        if count >= 3 {
            count = 0;
            all_groups.push(group);
            group = Vec::<String>::new();
        }
    }

    let total = all_groups
        .into_iter()
        .map(|group| get_char_priority(common_all_rugsacks(group)))
        .reduce(|p1, p2| p1 + p2);
    println!("Result P2: {}", total.unwrap());
}
