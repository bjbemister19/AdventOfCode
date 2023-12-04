use anyhow::{anyhow, Context, Result, Ok};
use std::collections::HashSet;
use std::fs;

#[derive(Debug)]
struct Card {
    winning_numbers: HashSet<usize>,
    your_numbers: HashSet<usize>,
}

fn parse_cards(input: &str) -> Result<Vec<Card>> {
    let cards = input
        .lines()
        .map(|unparsed_card| {
            let colon_split = unparsed_card.split(":").skip(1).collect::<Vec<&str>>();
            if colon_split.len() != 1 {
                return Err(anyhow!(
                    "Unexpected number of elements splitting on colon - Expected: 1 Actual: {}",
                    colon_split.len()
                ));
            }

            let pipe_split = colon_split[0].split("|").collect::<Vec<&str>>();
            if pipe_split.len() != 2 {
                return Err(anyhow!(
                    "Unexpected number of elements splitting on pipe - Expected: 1 Actual: {}",
                    pipe_split.len()
                ));
            }

            let unparsed_winning_numbers = pipe_split[0].trim();
            let unparsed_your_numbers = pipe_split[1].trim();

            let winning_numbers = unparsed_winning_numbers
                .split(" ")
                .filter_map(|number| {
                    if number == "" {
                        None
                    } else {
                        Some(number.parse::<usize>().ok())
                    }
                })
                .collect::<Option<HashSet<usize>>>()
                .context("Failed to parse winning numbers")?;
            let your_numbers = unparsed_your_numbers
                .split(" ")
                .filter_map(|number| {
                    if number == "" {
                        None
                    } else {
                        Some(number.parse::<usize>().ok())
                    }
                })
                .collect::<Option<HashSet<usize>>>()
                .context("Failed to parse your numbers")?;

            Ok(Card {
                winning_numbers: winning_numbers,
                your_numbers: your_numbers,
            })
        })
        .collect::<Result<Vec<Card>>>();
    cards
}

fn part_1(cards: &Vec<Card>) -> Result<()> {
    let common_numbers = cards
        .iter()
        .map(|card| {
            card.winning_numbers
                .intersection(&card.your_numbers)
                .collect()
        })
        .collect::<Vec<HashSet<&usize>>>();

    let points = common_numbers
        .iter()
        .map(|common_numbers| {
            let base: u32 = 2;
            let exp: u32 = common_numbers
                .len()
                .try_into()
                .map_err(|_| anyhow!("Failed to covert len to u32"))?;
            if exp == 0 {
                return Ok(0);
            }
            Ok(base.pow(exp - 1))
        })
        .collect::<Result<Vec<u32>>>()?;

    let total_points = points.into_iter().reduce(|a, b| a + b).context("Failed to total points")?;

    println!("{:?}", total_points);
    
    Ok(())
}

fn part_2(_cards: &Vec<Card>) -> Result<()>{
    Ok(())
}

fn main() -> Result<()> {
    let input = fs::read_to_string("input.txt").context("Failed to load input")?;
    let cards = parse_cards(&input).context("Failed to parse cards")?;

    println!("Part 1: ");
    part_1(&cards)?;

    println!("Part 2: ");
    part_2(&cards)?;

    Ok(())
}
