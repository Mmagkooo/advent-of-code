use std::io::{stdin, BufRead};

const SAFE: char = '.';
const TRAP: char = '^';

fn get_position_kind(chars: &Vec<char>, i: i32) -> char {
    if i < 0{
        return SAFE;
    }

    *chars.get(i as usize).or(Some(&SAFE)).unwrap()
}

fn main() {
    let lines: Vec<String> = stdin()
        .lock()
        .lines()
        .map(|l| l.expect("Cannot read"))
        .collect();

    let number_of_lines: usize = lines[0].parse().unwrap();
    let mut line = lines[1].clone();
    let mut safe_counter = 0;
    for _ in 0..number_of_lines {
        let chars: Vec<char> = line.chars().collect();
        let mut next_line = String::new();
        for i in 0..line.len() {
            let left = get_position_kind(&chars, i as i32 - 1);
            let center = get_position_kind(&chars, i as i32);
            let right = get_position_kind(&chars, i as i32 + 1);

            if center == SAFE {
                safe_counter += 1;
            }

            let next_char = match (left, center, right) {
                (TRAP, TRAP, SAFE) => TRAP,
                (SAFE, TRAP, TRAP) => TRAP,
                (TRAP, SAFE, SAFE) => TRAP,
                (SAFE, SAFE, TRAP) => TRAP,
                _ => SAFE,
            };
            next_line.push(next_char);
        }

        line = next_line;
    }

    println!("{safe_counter}");
}
