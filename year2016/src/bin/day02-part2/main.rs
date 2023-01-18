use std::io::{stdin, BufRead};

type Point = (i32, i32);

fn make_move(position: &Point, change: &Point) -> Point {
    let new_position = (position.0 + change.0, position.1 + change.1);
    if new_position.0 < 0 || new_position.0 >= 5 || new_position.1 < 0 || new_position.1 >= 5 {
        return *position;
    }
    if KEYPAD[new_position.0 as usize][new_position.1 as usize] == 0 {
        return *position;
    }
    return new_position;
}

const KEYPAD: [[i32; 5]; 5] = [
    [0, 0, 1, 0, 0],
    [0, 2, 3, 4, 0],
    [5, 6, 7, 8, 9],
    [0, 10, 11, 12, 0],
    [0, 0, 13, 0, 0],
];

fn main() {
    let mut position = (2, 0);
    for line in stdin().lock().lines().map(|l| l.expect("Could not read")) {
        for command in line.chars() {
            let change = match command {
                'U' => (-1, 0),
                'D' => (1, 0),
                'R' => (0, 1),
                'L' => (0, -1),
                invalid => panic!("Invalid char: {invalid}"),
            };
            position = make_move(&position, &change);
        }
        print!("{:X}", KEYPAD[position.0 as usize][position.1 as usize]);
    }
    println!();
}
