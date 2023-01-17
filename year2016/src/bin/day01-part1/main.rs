use std::io::{stdin, BufRead};

fn main() {
    let mut line = String::new();
    stdin()
        .lock()
        .read_line(&mut line)
        .expect("Could not read line");

    // Point coordinates: (x, y)
    let mut position = (0, 0); // initially at origin
    let mut direction = (0, 1); // initially facing north
    for command in line.split(", ") {
        let command = command.trim();
        direction = match &command[..1] {
            "R" => (direction.1, -direction.0),
            "L" => (-direction.1, direction.0),
            invalid => panic!("Invalid symbol: {invalid}"),
        };

        let amount = &command[1..].parse::<i32>().unwrap();

        position = (
            position.0 + direction.0 * amount,
            position.1 + direction.1 * amount,
        );
    }

    println!("{}", position.0.abs() + position.1.abs());
}
