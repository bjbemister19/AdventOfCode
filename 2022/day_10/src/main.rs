use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Write;
use std::{thread, time};

#[derive(Debug)]
struct CPU {
    cycle_count: i32,
    register: i32,
    signal_strength: i32,
}

impl CPU {
    fn new() -> CPU {
        CPU {
            cycle_count: 0,
            register: 1,
            signal_strength: 0,
        }
    }

    fn draw(&self) {
        let col = (self.cycle_count - 1) % 40;
        if self.register == col || self.register - 1 == col || self.register + 1 == col {
            // print!("{} ", col);
            print!("#");
        } else {
            // print!("{} ", col);
            print!(".");
        }
        if col == 39 {
            println!("");
        }
        io::stdout().flush().unwrap();
        thread::sleep(time::Duration::from_millis(25));
    }

    fn signal_is_interesting(&self) -> bool {
        // println!("{} <= 220 && {} && ({} || {})", self.cycle_count, self.cycle_count % 20 == 0, self.cycle_count < 40, self.cycle_count % 40 == 0);
        if self.cycle_count <= 220
            && self.cycle_count % 20 == 0
            && (self.cycle_count < 40 || (self.cycle_count - 20) % 40 == 0)
        {
            true
        } else {
            false
        }
    }

    fn update_signal_strength(&mut self) {
        if self.signal_is_interesting() {
            // println!("{} * {} = {}", self.cycle_count, self.register, self.cycle_count * self.register);
            self.signal_strength += self.cycle_count * self.register;
        }
    }

    fn execute(&mut self, instruction: &str) -> Option<()> {
        if instruction == "noop" {
            self.cycle_count += 1;
            self.update_signal_strength();
            self.draw();
        } else if instruction.starts_with("addx") {
            self.cycle_count += 1;
            self.update_signal_strength();
            self.draw();
            self.cycle_count += 1;
            self.update_signal_strength();
            self.draw();
            let rhs = instruction.split(" ").collect::<Vec<&str>>()[1]
                .parse::<i32>()
                .ok()?;
            // println!("{} += {} = {}", self.register, rhs, self.register + rhs);
            self.register += rhs;
        }
        Some(())
    }
}

fn main() {
    let file = File::open("data/input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut cpu = CPU::new();
    for maybe_line in reader.lines() {
        let line = maybe_line.unwrap();
        cpu.execute(&line).unwrap();
    }

    println!("CPU: {:?}", cpu);
}
