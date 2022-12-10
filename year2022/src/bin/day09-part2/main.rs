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

fn main() {
    let lines = io::stdin().lock().lines();
    let lines: Vec<String> = lines.map(|l| l.unwrap()).collect();

    const NUMBER_OF_KNOTS: usize = 10;
    let mut knots = [Point { x: 0, y: 0 }; NUMBER_OF_KNOTS];

    let mut visited: HashSet<Point> = HashSet::new();
    visited.insert(knots[0]);

    for line in lines {
        let command: Vec<&str> = line.split(" ").collect();
        let movement = direction_to_tuple(command[0]);
        let number_of_steps: i32 = command[1].parse().unwrap();

        for _ in 0..number_of_steps {
            knots[0] = Point {
                x: knots[0].x + movement.x,
                y: knots[0].y + movement.y,
            };
            for knot_i in 1..NUMBER_OF_KNOTS {
                knots[knot_i] = generate_new_tail(&knots[knot_i - 1], &knots[knot_i]);
            }
            visited.insert(knots[knots.len() - 1]);
        }
    }

    println!("{}", visited.len());
}
