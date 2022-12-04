use std::{io::{BufReader, BufRead}, fs::File};

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Throw {
    Rock = 1,
    Paper = 2,
    Sissors = 3
}

#[derive(Debug)]
pub struct Match {
    opponent: Throw,
    player: Throw
}

#[derive(Debug, Copy, Clone)]
pub enum Outcome {
    Lose = 0,
    Draw = 3,
    Win = 6
} 

fn outcome_of_match(m: Match) -> Outcome {
    if m.player == m.opponent {
        Outcome::Draw
    } else if m.player == Throw::Rock && m.opponent == Throw::Sissors {
        Outcome::Win
    } else if m.player == Throw::Paper && m.opponent == Throw::Rock {
        Outcome::Win
    } else if m.player == Throw::Sissors && m.opponent == Throw::Paper {
        Outcome::Win
    } else {
        Outcome::Lose
    }
}

pub fn calculate_match_score(m: Match) -> i32 {
    m.player as i32 + outcome_of_match(m) as i32
}

pub fn read_matches_part_1(path: &str) -> Vec<Match>{
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    let matches:Vec<Match> = reader.lines().map(|result| {
        let line = result.unwrap();
        let m:Vec<&str> = line.split(" ").collect();
        
        let opponent = match m[0] {
            "A" => Throw::Rock,
            "B" => Throw::Paper,
            "C" => Throw::Sissors,
            &_ => todo!(),
        };

        let player = match m[1] {
            "X" => Throw::Rock,
            "Y" => Throw::Paper,
            "Z" => Throw::Sissors,
            &_ => todo!(),
        };
        Match{opponent: opponent, player: player}
    }).collect();
    matches
}

/*
 * X means you need to lose
 * Y means you need to end the round in a draw
 * Z means you need to win
 */
fn choose_what_to_throw(opponent: Throw, instruction: &str) -> Throw {
    if instruction == "Y"{
        opponent
    } else if instruction == "Z" {
        match opponent {
            Throw::Rock => Throw::Paper,
            Throw::Paper => Throw::Sissors,
            Throw::Sissors => Throw::Rock
        }
    } else if instruction == "X" {
        match opponent {
            Throw::Rock => Throw::Sissors,
            Throw::Paper => Throw::Rock,
            Throw::Sissors => Throw::Paper
        }
    } else {
        panic!();
    }
}

pub fn read_matches_part_2(path: &str) -> Vec<Match>{
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    let matches:Vec<Match> = reader.lines().map(|result| {
        let line = result.unwrap();
        let m:Vec<&str> = line.split(" ").collect();
        
        let opponent = match m[0] {
            "A" => Throw::Rock,
            "B" => Throw::Paper,
            "C" => Throw::Sissors,
            &_ => todo!(),
        };

        let player = choose_what_to_throw(opponent, m[1]);
        Match{opponent: opponent, player: player}
    }).collect();
    matches
}
