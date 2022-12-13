use std::collections::HashSet;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

#[derive(Debug, Copy, Clone, Eq, Hash, PartialEq)]
struct Pos {
    x: i32,
    y: i32,
}

#[derive(Debug, Copy, Clone)]
struct Knot {
    pos: Pos,
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Copy, Clone)]
struct Vector {
    direction: Direction,
    magnitude: i32,
}

fn move_head(head_direction: &Direction, head: &mut Knot) {
    match head_direction {
        Direction::Up => head.pos.x -= 1,
        Direction::Down => head.pos.x += 1,
        Direction::Left => head.pos.y -= 1,
        Direction::Right => head.pos.y += 1,
    }
}

fn is_adjcent(head: &Knot, tail: &Knot) -> bool {
    head.pos.x == tail.pos.x || head.pos.y == tail.pos.y
}

fn add_component(x: i32, y: i32) -> i32 {
    (x - y) / (x - y).abs()
}

fn move_tail(knots: &mut Vec<Knot>, i: usize) {
    if (knots[i - 1].pos.x - knots[i].pos.x).abs() > 1 {
        knots[i].pos.x += add_component(knots[i - 1].pos.x, knots[i].pos.x);
        if !is_adjcent(&knots[i - 1], &knots[i]) {
            knots[i].pos.y += add_component(knots[i - 1].pos.y, knots[i].pos.y);
        }
    }
    if (knots[i - 1].pos.y - knots[i].pos.y).abs() > 1 {
        knots[i].pos.y += add_component(knots[i - 1].pos.y, knots[i].pos.y);
        if !is_adjcent(&knots[i - 1], &knots[i]) {
            knots[i].pos.x += add_component(knots[i - 1].pos.x, knots[i].pos.x);
        }
    }
}

fn move_rope(vector: &Vector, knots: &mut Vec<Knot>, visited: &mut HashSet<Pos>) {
    for _ in 0..vector.magnitude {
        for i in 0..knots.len() {
            if i == 0 {
                move_head(&vector.direction, &mut knots[i]);
            } else {
                move_tail(knots, i);
            }
            visited.insert(knots[knots.len() - 1].pos.clone());
        }
    }
}

fn parse_direction(dir: &str) -> Option<Direction> {
    if dir == "U" {
        Some(Direction::Up)
    } else if dir == "D" {
        Some(Direction::Down)
    } else if dir == "L" {
        Some(Direction::Left)
    } else if dir == "R" {
        Some(Direction::Right)
    } else {
        None
    }
}

fn parse_vector(command: &str) -> Option<Vector> {
    let components = command.split(" ").collect::<Vec<&str>>();
    Some(Vector {
        direction: parse_direction(&components[0])?,
        magnitude: components[1].parse().ok()?,
    })
}

fn p1() {
    let f = File::open("data/input.txt").unwrap();
    let reader = BufReader::new(f);

    let head = Knot {
        pos: Pos { x: 0, y: 4 },
    };
    let tail = Knot {
        pos: Pos { x: 0, y: 4 },
    };
    let mut visited: HashSet<Pos> = HashSet::new();

    let mut knots: Vec<Knot> = Vec::new();
    knots.push(head);
    knots.push(tail);

    for maybe_line in reader.lines() {
        let line = maybe_line.unwrap();

        let vector = parse_vector(&line).unwrap();
        move_rope(&vector, &mut knots, &mut visited);
    }
    println!("Part 1");
    println!(
        "Head {},{} Tail {},{}",
        knots[0].pos.x, knots[0].pos.y, knots[1].pos.x, knots[1].pos.y
    );
    println!("Visited {}", visited.len())
}

fn p2() {
    let f = File::open("data/input.txt").unwrap();
    let reader = BufReader::new(f);

    let mut visited: HashSet<Pos> = HashSet::new();

    let mut knots: Vec<Knot> = vec![
        Knot {
            pos: Pos { x: 0, y: 4 }
        };
        10
    ];

    for maybe_line in reader.lines() {
        let line = maybe_line.unwrap();

        let vector = parse_vector(&line).unwrap();
        move_rope(&vector, &mut knots, &mut visited);
    }

    println!("Part 2");
    println!(
        "Head {},{} Tail {},{}",
        knots[0].pos.x, knots[0].pos.y, knots[1].pos.x, knots[1].pos.y
    );
    println!("Visited {}", visited.len())
}

fn main() {
    p1();
    p2();
}
