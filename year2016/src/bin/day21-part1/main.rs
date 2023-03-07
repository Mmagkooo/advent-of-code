use std::io::{stdin, BufRead};

fn rotate_right(password: &mut Vec<char>) {
    let last = password.remove(password.len() - 1);
    password.insert(0, last);
}

fn rotate_left(password: &mut Vec<char>) {
    let first = password.remove(0);
    password.push(first);
}

fn main() {
    let lines = stdin().lock().lines().map(|l| l.expect("Cannot read line"));
    let argv: Vec<String> = std::env::args().collect();
    let mut password: Vec<char> = argv[1].chars().collect();
    let password_length = password.len();

    for command in lines {
        let split: Vec<&str> = command.split_whitespace().collect();
        if command.starts_with("swap position") {
            let pos1 = split[2].parse().unwrap();
            let pos2 = split.last().unwrap().parse().unwrap();
            password.swap(pos1, pos2);
        } else if command.starts_with("swap letter") {
            let letter1 = split[2].chars().next().unwrap();
            let letter2 = split.last().unwrap().chars().next().unwrap();
            for i in 0..password_length {
                if password[i] == letter1 {
                    password[i] = letter2;
                } else if password[i] == letter2 {
                    password[i] = letter1;
                }
            }
        } else if command.starts_with("reverse positions") {
            let pos1: usize = split[2].parse().unwrap();
            let pos2: usize = split.last().unwrap().parse().unwrap();
            let reversing_length = pos2 - pos1 + 1;
            for i in 0..reversing_length / 2 {
                password.swap(pos1 + i, pos2 - i);
            }
        } else if command.starts_with("rotate left") {
            let steps = split[2].parse().unwrap();
            for _ in 0..steps {
                rotate_left(&mut password);
            }
        } else if command.starts_with("rotate right") {
            let steps = split[2].parse().unwrap();
            for _ in 0..steps {
                rotate_right(&mut password);
            }
        } else if command.starts_with("rotate based on") {
            let letter = split.last().unwrap().chars().next().unwrap();
            let index = password.iter().position(|c| *c == letter).unwrap();
            let steps = index + 1 + if index >= 4 { 1 } else { 0 };
            for _ in 0..steps {
                rotate_right(&mut password);
            }
        } else if command.starts_with("move") {
            let from_index = split[2].parse().unwrap();
            let to_index = split.last().unwrap().parse().unwrap();
            let removed_char = password.remove(from_index);
            password.insert(to_index, removed_char);
        } else {
            panic!("Invalid command: {command}");
        }
    }

    let final_password: String = password.iter().collect();
    println!("{}", final_password);
}
