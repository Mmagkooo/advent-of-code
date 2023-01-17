use std::collections::HashSet;
use std::io::{stdin, BufRead};

type Point = (i32, i32);

fn main() {
    let mut line = String::new();
    stdin()
        .lock()
        .read_line(&mut line)
        .expect("Could not read line");

    let mut visited = HashSet::<Point>::new();

    // Point coordinates: (x, y)
    let mut position = (0, 0); // initially at origin
    visited.insert(position);
    let mut direction = (0, 1); // initially facing north
    'main_loop: for command in line.split(", ") {
        let command = command.trim();
        direction = match &command[..1] {
            "R" => (direction.1, -direction.0),
            "L" => (-direction.1, direction.0),
            invalid => panic!("Invalid symbol: {invalid}"),
        };

        let amount = &command[1..].parse::<i32>().unwrap();

        for _ in 0..*amount {
            position = (position.0 + direction.0, position.1 + direction.1);

            if visited.contains(&position) {
                break 'main_loop;
            } else {
                visited.insert(position);
            }
        }
    }

    println!("{}", position.0.abs() + position.1.abs());
}
