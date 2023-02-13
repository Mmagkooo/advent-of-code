use std::{
    collections::HashMap,
    io::{stdin, BufRead},
};

fn main() {
    let mut registers = HashMap::<&str, i32>::new();
    for reg_name in ["a", "b", "c", "d"] {
        registers.insert(reg_name, 0);
    }

    let lines = stdin().lock().lines().map(|l| l.expect("No line"));

    let mut instructions = vec![];
    for line in lines {
        let instruction: Vec<String> = line
            .split_whitespace()
            .map(|token| token.to_string())
            .collect();
        instructions.push(instruction);
    }

    let mut pc: i32 = 0;
    let total_instructions = instructions.len() as i32;
    while pc < total_instructions {
        let instruction = &instructions[pc as usize];
        match instruction[0].as_str() {
            "cpy" => {
                let raw_val = instruction[1].as_str();
                let val = match raw_val.parse() {
                    Ok(val) => val,
                    Err(_) => *registers.get(&raw_val).unwrap(),
                };
                let reg = instruction[2].as_str();
                registers.insert(reg, val);
            }
            "inc" => {
                let reg = instruction[1].as_str();
                let reg_val = registers.get(&reg).unwrap();
                registers.insert(reg, reg_val + 1);
            }
            "dec" => {
                let reg = instruction[1].as_str();
                let reg_val = registers.get(&reg).unwrap();
                registers.insert(reg, reg_val - 1);
            }
            "jnz" => {
                let reg = instruction[1].as_str();
                let reg_val = match reg.parse::<i32>() {
                    Ok(number) => number,
                    Err(_) => *registers.get(reg).unwrap(),
                };

                let jump_amount: i32 = instruction[2].parse().unwrap();
                if reg_val != 0 {
                    pc += jump_amount;
                } else {
                    pc += 1;
                }

                continue; // don't do regular pc increment
            }
            _ => panic!("Invalid instruction: {instruction:?}"),
        }

        pc += 1;
    }

    println!("{}", registers.get("a").unwrap());
}
