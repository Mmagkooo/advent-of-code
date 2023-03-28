use std::{
    borrow::Borrow,
    collections::HashMap,
    io::{stdin, BufRead},
};

type Registers = HashMap<String, i32>;

/**
 * Tries to interpret raw_val as a number or as the value of a register.
 */
fn get_value(raw_val: &String, registers: &Registers) -> i32 {
    let val = match raw_val.parse() {
        Ok(val) => val,
        Err(_) => *registers.get(raw_val).unwrap(),
    };
    return val;
}

const REGISTER_NAMES: [&str; 4] = ["a", "b", "c", "d"];

// fn get_register_name(raw_val: &String) -> Option<&String> {
//     match REGISTER_NAMES.contains(&raw_val.as_str()) {
//         true => Some(raw_val),
//         false => None,
//     }
// }

fn main() {
    let mut registers = Registers::new();
    for reg_name in REGISTER_NAMES {
        registers.insert(reg_name.to_string(), 0);
    }

    let argv: Vec<String> = std::env::args().collect();
    let input: i32 = argv[1].parse().unwrap();
    registers.insert("a".to_string(), input);

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
        let instruction = instructions[pc as usize].clone();
        match instruction[0].as_str() {
            // one-argument instructions
            "inc" => {
                let reg = instruction[1].clone();
                let reg_val = registers.get(&reg).unwrap();
                registers.insert(reg, reg_val + 1);
            }
            "dec" => {
                let reg = instruction[1].clone();
                let reg_val = registers.get(&reg).unwrap();
                registers.insert(reg, reg_val - 1);
            }
            // two-argument instructions
            "cpy" => {
                let val = get_value(instruction[1].borrow(), &registers);
                let reg = instruction[2].clone();
                registers.insert(reg, val);
            }
            "jnz" => {
                let condition_value = get_value(&instruction[1], &registers);
                if condition_value != 0 {
                    let jump_amount: i32 = get_value(&instruction[2], &registers);
                    pc += jump_amount - 1; // -1 because regular pc++ happens
                }
            }
            "tgl" => {
                let modification_index = get_value(&instruction[1], &registers) + pc;
                // only allow modifying on legal indices, otherwise nothing happens
                if modification_index >= 0 && modification_index < total_instructions {
                    let modified_instruction = &mut instructions[modification_index as usize];
                    println!("Toggling at index {modification_index}: {modified_instruction:?}");
                    let new_mnemonic = match modified_instruction.len() - 1 {
                        // match by number of args
                        1 => match modified_instruction[0].as_str() {
                            "inc" => "dec",
                            _ => "inc",
                        },
                        2 => match modified_instruction[0].as_str() {
                            "jnz" => "cpy",
                            _ => "jnz",
                        },
                        inv => {
                            panic!("Invalid number of args ({inv}) of instruction {instruction:?}")
                        }
                    };
                    modified_instruction[0] = new_mnemonic.to_string();
                }
            }
            _ => panic!("Invalid instruction: {instruction:?}"),
        }

        pc += 1;
    }

    println!("{}", registers.get("a").unwrap());
}
