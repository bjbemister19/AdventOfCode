use anyhow::{Context, Result};
use std::collections::HashMap;
use std::fs;

#[derive(Debug)]
struct Game {
    game_number: u32,
    rounds: Vec<HashMap<String, u32>>,
}

fn parse_games(input: &str) -> Vec<Game> {
    let parsed_input = input
        .lines()
        .filter_map(|unparsed_game| {
            let split = unparsed_game.split(':').collect::<Vec<&str>>();
            let game_num = split[0].split(" ").collect::<Vec<&str>>()[1]
                .parse::<u32>()
                .ok()?;

            // let rounds: Vec<HashMap<String, u32>> = Vec::new();
            let unparsed_rounds = split[1].split(";").collect::<Vec<&str>>();
            let rounds = unparsed_rounds
                .into_iter()
                .filter_map(|unparsed_round| {
                    let mut hash_map: HashMap<String, u32> = HashMap::new();

                    let split = unparsed_round.trim().split(",").collect::<Vec<&str>>();
                    for draw in split {
                        let split = draw.trim().split(' ').collect::<Vec<&str>>();
                        let num_cubes = split[0].trim().parse::<u32>().ok()?;
                        let cube_color = split[1].trim();
                        hash_map.insert(String::from(cube_color), num_cubes);
                    }
                    Some(hash_map)
                })
                .collect::<Vec<HashMap<String, u32>>>();

            Some(Game {
                game_number: game_num,
                rounds: rounds,
            })
        })
        .collect::<Vec<Game>>();
    parsed_input
}

fn part_1(games: &Vec<Game>) -> Result<()> {
    // Given only 12 red cubes, 13 green cubes, and 14 blue cubes
    let possible_games = games
        .into_iter()
        .filter(|game| {
            for round in &game.rounds {
                let num_red = round.get("red").unwrap_or(&0);
                let num_green = round.get("green").unwrap_or(&0);
                let num_blue = round.get("blue").unwrap_or(&0);

                if num_red > &12 || num_green > &13 || num_blue > &14 {
                    return false;
                }
            }
            true
        })
        .collect::<Vec<&Game>>();

    let mut sum_of_game_numbers: u32 = 0;
    for game in &possible_games {
        sum_of_game_numbers = sum_of_game_numbers + game.game_number;
    }

    println!("{:?}", sum_of_game_numbers);

    Ok(())
}

fn part_2(games: &Vec<Game>) -> Result<()> {
    let power = games
        .into_iter()
        .map(|game| {
            let mut min_red: u32 = 0;
            let mut min_green: u32 = 0;
            let mut min_blue: u32 = 0;

            for round in &game.rounds {
                let num_red = *round.get("red").unwrap_or(&0);
                let num_green = *round.get("green").unwrap_or(&0);
                let num_blue = *round.get("blue").unwrap_or(&0);

                if num_red > min_red {
                    min_red = num_red;
                }
                if num_green > min_green {
                    min_green = num_green;
                }
                if num_blue > min_blue {
                    min_blue = num_blue;
                }
            }

            min_red * min_green * min_blue
        })
        .collect::<Vec<u32>>();

    let total_power = power
        .into_iter()
        .reduce(|a, b| a + b)
        .context("Failed to sum")?;
    println!("{:?}", total_power);

    Ok(())
}

fn main() -> Result<()> {
    let input = fs::read_to_string("input.txt").context("Failed to read input")?;

    let games = parse_games(&input);

    println!("Part 1: ");
    part_1(&games)?;

    println!("Part 2: ");
    part_2(&games)?;

    Ok(())
}
