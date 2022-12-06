use std::io::{stdin, BufRead};
use std::env;
use std::vec;

const OFFSET: usize = 1;
const PERIOD: usize = 4;

fn main() {
    let argv: Vec<String> = env::args().collect();
    let width: i32 = argv[1].parse().expect("Could not parse");

    let mut stacks: Vec<Vec<char>> = vec::from_elem(vec![], width as usize);

    loop { // parse initial stacks
        let mut input = String::new();
        stdin().read_line(&mut input).expect("Failed reading");
        if !input.contains("[") {
            stdin().read_line(&mut input).expect("Could not read");
            break;
        } else {
            for i in 0..width {
                // TODO this iterates from the beginning each time???
                let input_index = i as usize * PERIOD + OFFSET;
                let letter = input.chars().nth(input_index).unwrap();
                if !letter.is_whitespace() {
                    stacks[i as usize].push(letter);
                }
            }
        }
    }

    stacks.iter_mut().for_each(|stack| stack.reverse());

    for line in stdin().lock().lines() {
        let line = line.unwrap();
        let words: Vec<&str> = line.split(" ").collect();
        let amount = words[1].parse::<usize>().unwrap();
        let from = words[3].parse::<usize>().unwrap() - 1;
        let to = words[5].parse::<usize>().unwrap() - 1;
        
        let mut buffer: Vec<char> = vec![];
        for _ in 0..amount {
            buffer.push(stacks[from].pop().unwrap());
        }
        buffer.reverse();
        
        for value in buffer {
            stacks[to].push(value);
        }
    }

    stacks.iter().for_each(|stack| print!("{}", stack.last().unwrap()));
    println!();
}