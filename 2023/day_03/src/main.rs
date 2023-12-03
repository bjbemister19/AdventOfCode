use anyhow::{Context, Ok, Result};
use std::collections::{HashMap, HashSet};
use std::fs;

mod helpers;
use helpers::Array2d;

fn get_parsed_input(input: &str) -> Array2d<char> {
    let v = &input.lines().collect::<Vec<&str>>();

    let x_cap = v[0].len();
    let y_cap = v.len();

    let mut parsed_input: Array2d<char> = Array2d::new(x_cap, y_cap, '\0');
    for (i, line) in input.lines().enumerate() {
        for (j, char) in line.chars().enumerate() {
            parsed_input.insert(char, i, j);
        }
    }
    parsed_input
}

fn is_symbol(c: char) -> bool {
    !(c.is_digit(10) || c == '.')
}

fn adjcent_to_symbol(parsed_input: &Array2d<char>, row_u: usize, col_u: usize) -> bool {
    let row: i32 = row_u.try_into().unwrap();
    let col: i32 = col_u.try_into().unwrap();

    let vec = vec![
        parsed_input.get_s(row - 1, col - 1), // Top Left
        parsed_input.get_s(row - 1, col),     // Top
        parsed_input.get_s(row - 1, col + 1), // Top Right
        parsed_input.get_s(row, col - 1),     // Left
        parsed_input.get_s(row, col + 1),     // Right
        parsed_input.get_s(row + 1, col - 1), // Bottom Left
        parsed_input.get_s(row + 1, col),     // Bottom
        parsed_input.get_s(row + 1, col + 1), // Bottom Right
    ];

    for maybe_symbol in vec {
        if let Some(symbol) = maybe_symbol {
            if is_symbol(symbol) {
                return true;
            }
        }
    }
    false
}

fn part_1(parsed_input: &Array2d<char>) -> Result<()> {
    let mut buffer = String::new();
    let mut adjcent = false;
    let mut part_numbers: Vec<u32> = Vec::new();

    for row in 0..parsed_input.width {
        for col in 0..parsed_input.height {
            let maybe_c = parsed_input.get(row, col);
            if let Some(c) = maybe_c {
                if c.is_digit(10) && c != '.' {
                    buffer.push(c);
                    if !adjcent {
                        adjcent = adjcent_to_symbol(&parsed_input, row, col);
                    }
                } else {
                    if adjcent {
                        part_numbers.push(
                            buffer
                                .parse::<u32>()
                                .context("Failed to parse part number")?,
                        );
                    }
                    buffer = String::new();
                    adjcent = false;
                }
            } else {
                anyhow::bail!("Index out of bounds, must be bug");
            }
        }
    }

    let total = part_numbers
        .into_iter()
        .reduce(|a, b| a + b)
        .context("Failed to reduce")?;
    println!("{:?}", total);

    Ok(())
}

fn adjcent_stars(parsed_input: &Array2d<char>, row_u: usize, col_u: usize) -> Vec<String> {
    let row: i32 = row_u.try_into().unwrap();
    let col: i32 = col_u.try_into().unwrap();

    let vec: Vec<((i32, i32), Option<char>)> = vec![
        ((row - 1, col - 1), parsed_input.get_s(row - 1, col - 1)), // Top Left
        ((row - 1, col), parsed_input.get_s(row - 1, col)),         // Top
        ((row - 1, col + 1), parsed_input.get_s(row - 1, col + 1)), // Top Right
        ((row, col - 1), parsed_input.get_s(row, col - 1)),         // Left
        ((row, col + 1), parsed_input.get_s(row, col + 1)),         // Right
        ((row + 1, col - 1), parsed_input.get_s(row + 1, col - 1)), // Bottom Left
        ((row + 1, col), parsed_input.get_s(row + 1, col)),         // Bottom
        ((row + 1, col + 1), parsed_input.get_s(row + 1, col + 1)), // Bottom Right
    ];

    let mut v: Vec<String> = Vec::new();
    for maybe_symbol in vec {
        if let Some(symbol) = maybe_symbol.1 {
            if symbol == '*' {
                v.push(format!("{},{}", maybe_symbol.0 .0, maybe_symbol.0 .1));
            }
        }
    }
    v
}

fn part_2(parsed_input: &Array2d<char>) -> Result<()> {
    let mut buffer = String::new();
    let mut set: HashSet<String> = HashSet::new();
    let mut map: HashMap<String, Vec<u32>> = HashMap::new();

    for row in 0..parsed_input.width {
        for col in 0..parsed_input.height {
            let maybe_c = parsed_input.get(row, col);
            if let Some(c) = maybe_c {
                if c.is_digit(10) && c != '.' {
                    buffer.push(c);
                    let adjcent_stars = adjcent_stars(&parsed_input, row, col);
                    for star in adjcent_stars {
                        set.insert(star);
                    }
                } else {
                    if !set.is_empty() {
                        let part_number = buffer
                            .parse::<u32>()
                            .context("Failed to parse part number")?;
                        for star in set {
                            if map.contains_key(&star) {
                                let v = map
                                    .get(&star)
                                    .context("Cant get from map, should be impossible")?;
                                let mut new_v = v.clone();
                                new_v.push(part_number);
                                map.insert(star, new_v);
                            } else {
                                let mut v = Vec::new();
                                v.push(part_number);
                                map.insert(star, v);
                            }
                        }
                    }
                    buffer = String::new();
                    set = HashSet::new()
                }
            } else {
                anyhow::bail!("Index out of bounds, must be bug");
            }
        }
    }

    let mut sum_of_gear_ratios: u32 = 0;
    for maybe_gear in map {
        if maybe_gear.1.len() == 2 {
            let gear_ratio = maybe_gear.1[0] * maybe_gear.1[1];
            sum_of_gear_ratios = sum_of_gear_ratios + gear_ratio;
        }
    }
    println!("{:?}", sum_of_gear_ratios);

    Ok(())
}

fn main() -> Result<()> {
    let input = fs::read_to_string("input.txt").context("Failed to read from file")?;
    let parsed_input = get_parsed_input(&input);

    println!("Part 1: ");
    part_1(&parsed_input)?;

    println!("Part 2: ");
    part_2(&parsed_input)?;

    Ok(())
}
