use std::io::{self, BufRead};

#[derive(Debug)]
struct Cpu {
    cycle_counter: i32,
    register: i32,
    sum: i32
}

impl Cpu {
    fn new() -> Cpu {
        Cpu { cycle_counter: 0, register: 1, sum: 0 }
    }

    fn noop(&mut self) {
        self.run_cycle();
    }

    fn run_cycle(&mut self) {
        self.cycle_counter += 1;
        self.check();
    }

    fn check(&mut self) {
        if self.cycle_counter % 40 == 20 {
            self.sum += self.register * self.cycle_counter;
            println!("{self:?}");
        }
    }

    fn addx(&mut self, value: i32) {
        self.run_cycle();
        self.run_cycle();
        self.register += value;
    }
}


fn main() {
    let lines = io::stdin().lock().lines();

    let mut cpu = Cpu::new();

    for line in lines {
        let line = line.expect("Could not read line");

        if line == "noop" {
            cpu.noop();
        } else if line.starts_with("addx ") {
            let value: i32 = line.split(" ").nth(1).unwrap().parse().unwrap();
            cpu.addx(value);
        } else {
            panic!("Invalid line: '{line}'");
        }
    }

    println!("{}", cpu.sum);
}
