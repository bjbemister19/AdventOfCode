use anyhow::{Context, Result};
use std::convert::TryFrom;
use std::fs;

fn part1(input: &str) -> Result<()> {
    let calibration_values = input
        .lines()
        .filter_map(|line| {
            let mut unparsed_number = String::new();
            unparsed_number.push(line.chars().find(|c| c.is_digit(10))?);
            unparsed_number.push(line.chars().rev().find(|c| c.is_digit(10))?);
            unparsed_number.parse::<u32>().ok()
        })
        .collect::<Vec<u32>>();

    let sum_of_calibration_values = calibration_values
        .into_iter()
        .reduce(|a, b| a + b)
        .context("Failed to sum")?;
    println!("{:?}", sum_of_calibration_values);

    Ok(())
}

fn find_first<I>(chars: I, num_map: &Vec<&str>) -> Option<char>
where
    I: Iterator<Item = char>,
{
    let mut buffer = String::new();
    for c in chars {
        if c.is_digit(10) {
            return Some(c);
        } else {
            buffer.push(c);
            for word in num_map {
                if buffer.contains(word) {
                    let index = num_map.iter().position(|&r| &r == word)?;
                    return Some(char::from_digit(u32::try_from(index).ok()?, 10)?);
                }
            }
        }
    }
    return None;
}

fn part2(input: &str) -> Result<()> {
    let num_map = vec![
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let rev_num_map = vec![
        "orez", "eno", "owt", "eerht", "ruof", "evif", "xis", "neves", "thgie", "enin",
    ];

    let calibration_values = input
        .lines()
        .filter_map(|line| {
            let mut unparsed_number = String::new();
            unparsed_number.push(find_first(line.chars(), &num_map)?);
            unparsed_number.push(find_first(line.chars().rev(), &rev_num_map)?);
            unparsed_number.parse::<u32>().ok()
        })
        .collect::<Vec<u32>>();

    let sum_of_calibration_values = calibration_values
        .into_iter()
        .reduce(|a, b| a + b)
        .context("Failed to sum")?;
    println!("{:?}", sum_of_calibration_values);

    Ok(())
}

fn main() -> Result<()> {
    let input = fs::read_to_string("input.txt").context("Error reading input file")?;

    println!("Part 1: ");
    part1(&input)?;

    println!("Part2: ");
    part2(&input)?;
    Ok(())
}
