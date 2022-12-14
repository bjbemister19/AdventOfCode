use std::fs;

#[derive(Debug)]
enum Operation {
    OldTimesConst(i64),
    OldPlusConst(i64),
    OldSquared,
}

impl Operation {
    fn parse(operation: &str) -> Option<Operation> {
        if operation.contains("+") {
            let parts = operation.split("+ ").collect::<Vec<&str>>();
            Some(Operation::OldPlusConst(parts[1].parse().ok()?))
        } else if operation.contains("*") {
            let parts = operation.split(" * ").collect::<Vec<&str>>();
            if parts[1] == "old" {
                Some(Operation::OldSquared)
            } else {
                Some(Operation::OldTimesConst(parts[1].parse().ok()?))
            }
        } else {
            None
        }
    }

    fn compute(&self, old: i64) -> i64 {
        match self {
            Operation::OldTimesConst(c) => old * c,
            Operation::OldPlusConst(c) => old + c,
            Operation::OldSquared => old * old,
        }
    }
}

#[derive(Debug)]
struct Monkey {
    monkey_number: i64,
    items: Vec<i64>,
    operation: Operation,
    divisible_by: i64,
    if_true_throw_to: usize,
    if_false_throw_to: usize,
    inspected: i64,
}

impl Monkey {
    fn from_file(path: &str) -> Option<Vec<Monkey>> {
        let contents = fs::read_to_string(path).ok()?;
        let monkeys = contents
            .split("Monkey")
            .filter(|s| s != &"")
            .map(|s| {
                let parts = s
                    .split("\n")
                    .filter(|s| s != &"")
                    .map(|s| s.trim())
                    .collect::<Vec<&str>>();

                Some(Monkey {
                    monkey_number: parts[0].trim_end_matches(":").parse().ok()?,
                    items: parts[1]
                        .trim_start_matches("Starting items: ")
                        .split(", ")
                        .map(|s| s.parse().ok())
                        .filter_map(|i| i)
                        .collect(),
                    operation: Operation::parse(parts[2].trim_start_matches("Operation: "))?,
                    divisible_by: parts[3]
                        .trim_start_matches("Test: divisible by ")
                        .parse()
                        .ok()?,
                    if_true_throw_to: parts[4]
                        .trim_start_matches("If true: throw to monkey ")
                        .parse()
                        .ok()?,
                    if_false_throw_to: parts[5]
                        .trim_start_matches("If false: throw to monkey ")
                        .parse()
                        .ok()?,
                    inspected: 0,
                })
            })
            .filter_map(|maybe_monkey| maybe_monkey)
            .collect::<Vec<Monkey>>();
        Some(monkeys)
    }
}

fn round_p1(monkeys: &mut Vec<Monkey>) {
    for i in 0..monkeys.len() {
        for j in 0..monkeys[i].items.len() {
            monkeys[i].inspected += 1;
            let new = monkeys[i].operation.compute(monkeys[i].items[j]) / 3;
            if new % monkeys[i].divisible_by == 0 {
                let throw_to = monkeys[i].if_true_throw_to;
                monkeys[throw_to].items.push(new);
            } else {
                let throw_to = monkeys[i].if_false_throw_to;
                monkeys[throw_to].items.push(new);
            }
        }
        monkeys[i].items = Vec::new();
    }
}

fn round_p2(monkeys: &mut Vec<Monkey>) {
    for i in 0..monkeys.len() {
        for j in 0..monkeys[i].items.len() {
            monkeys[i].inspected += 1;
            let new = monkeys[i].operation.compute(monkeys[i].items[j])
                % monkeys
                    .iter()
                    .map(|a| a.divisible_by)
                    .reduce(|a, b| a * b)
                    .unwrap();
            if new % monkeys[i].divisible_by == 0 {
                let throw_to = monkeys[i].if_true_throw_to;
                // let x = monkeys[i].items[j];
                monkeys[throw_to].items.push(new);
            } else {
                let throw_to = monkeys[i].if_false_throw_to;
                // let x = monkeys[i].items[j];
                monkeys[throw_to].items.push(new);
            }
        }
        monkeys[i].items = Vec::new();
    }
}

fn main() {
    let mut monkeys = Monkey::from_file("data/input.txt").unwrap();
    for _ in 0..10000 {
        round_p2(&mut monkeys);
    }

    monkeys.sort_by(|a, b| b.inspected.cmp(&a.inspected));

    for monkey in monkeys.iter() {
        println!(
            "Monkey {} inspected {}",
            monkey.monkey_number, monkey.inspected
        );
    }

    println!(
        "Monkey Business: {}",
        monkeys[0].inspected * monkeys[1].inspected
    );
}
