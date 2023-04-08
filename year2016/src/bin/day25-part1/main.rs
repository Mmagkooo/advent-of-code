use std::{
    collections::HashMap,
    io::{stdin, BufRead},
};

type Registers<'a> = HashMap<&'a str, i32>;

/**
 * Tries to interpret raw_val as a number or as the value of a register.
 */
fn get_value(raw_val: &String, registers: &Registers) -> i32 {
    let raw_val = raw_val.as_str();
    let val = match raw_val.parse() {
        Ok(val) => val,
        Err(_) => *registers.get(&raw_val).unwrap(),
    };
    return val;
}

fn is_clock(instructions: &Vec<Vec<String>>, init_a: i32, output_len_limit: usize) -> bool {
    let mut registers = HashMap::<&str, i32>::new();
    for reg_name in ["a", "b", "c", "d"] {
        registers.insert(reg_name, 0);
    }
    registers.insert("a", init_a);

    let mut pc: i32 = 0;
    let mut accumulated_output: usize = 0;
    let mut expected_output_value: i32 = 0;
    let total_instructions = instructions.len() as i32;
    while pc < total_instructions {
        let instruction = &instructions[pc as usize];
        match instruction[0].as_str() {
            // one-argument instructions
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
            // two-argument instructions
            "cpy" => {
                let val = get_value(&instruction[1], &registers);
                let reg = instruction[2].as_str();
                registers.insert(reg, val);
            }
            "jnz" => {
                let condition_value = get_value(&instruction[1], &registers);
                if condition_value != 0 {
                    let jump_amount: i32 = instruction[2].parse().unwrap();
                    pc += jump_amount - 1; // -1 because regular pc++ happens
                }
            }
            "out" => {
                let output_value = get_value(&instruction[1], &registers);
                if output_value == expected_output_value {
                    expected_output_value = 1 - expected_output_value;
                    accumulated_output += 1;
                    if accumulated_output == output_len_limit {
                        return true;
                    }
                } else {
                    return false;
                }
            }
            _ => panic!("Invalid instruction: {instruction:?}"),
        }

        pc += 1;
    }

    return false;
}

fn main() {
    let lines = stdin().lock().lines().map(|l| l.expect("No line"));

    let mut instructions = vec![];
    for line in lines {
        let instruction: Vec<String> = line
            .split_whitespace()
            .map(|token| token.to_string())
            .collect();
        instructions.push(instruction);
    }

    let mut init_a = 0;
    while !is_clock(&instructions, init_a, 10) {
        println!("{init_a}: not clock");
        init_a += 1;
    }
    println!("{init_a}: yes clock!");
}
