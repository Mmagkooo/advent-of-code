use std::{
    collections::HashSet,
    io::{self, BufRead},
};

#[derive(Eq, Clone, Copy, Debug, Hash, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}
type Movement = Point;

fn direction_to_tuple(direction: &str) -> Movement {
    match direction {
        "R" => Movement { x: 1, y: 0 },
        "L" => Movement { x: -1, y: 0 },
        "U" => Movement { x: 0, y: 1 },
        "D" => Movement { x: 0, y: -1 },
        _ => panic!("Invalid direction: {direction}"),
    }
}

fn generate_new_tail(head: &Point, tail: &Point) -> Point {
    let dx = (head.x - tail.x).abs();
    let dy = (head.y - tail.y).abs();

    // if adjacent, don't move
    if dx <= 1 && dy <= 1 {
        return *tail;
    }

    return Point {
        x: tail.x + (head.x - tail.x).signum(),
        y: tail.y + (head.y - tail.y).signum(),
    };
}

fn print_matrix(matrix: &HashSet<Point>, dim: i32) {
    for y in (0..dim).rev() {
        for x in 0..dim {
            if x == 0 && y == 0 {
                print!("s"); // starting point
                continue;
            }

            let p = Point { x, y };
            let c = if matrix.contains(&p) { "#" } else { "." };
            print!("{c}");
        }
        println!();
    }
}

fn main() {
    let lines = io::stdin().lock().lines();
    let lines: Vec<String> = lines.map(|l| l.unwrap()).collect();

    let mut visited: HashSet<Point> = HashSet::new();
    let mut head: Point = Point { x: 0, y: 0 };
    let mut tail: Point = Point { x: 0, y: 0 };
    visited.insert(head);

    for line in lines {
        let command: Vec<&str> = line.split(" ").collect();
        let movement = direction_to_tuple(command[0]);
        let amount: i32 = command[1].parse().unwrap();

        for _ in 0..amount {
            head = Point {
                x: head.x + movement.x,
                y: head.y + movement.y,
            };
            tail = generate_new_tail(&head, &tail);
            visited.insert(tail);
            print_matrix(&visited, 6);
            println!();
        }
    }

    println!("{}", visited.len());
}
