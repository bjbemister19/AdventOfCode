mod rps;

fn part_1() {
    let matches = rps::read_matches_part_1("data/input.txt");

    let mut total_score = 0;
    for m in matches {
        total_score = total_score + rps::calculate_match_score(m);
    }
    println!("Score: {}", total_score);
}

fn part_2() {
    let matches = rps::read_matches_part_2("data/input.txt");

    let mut total_score = 0;
    for m in matches {
        total_score = total_score + rps::calculate_match_score(m);
    }
    println!("Score: {}", total_score);
}

fn main() {
    part_1();
    part_2();
}
