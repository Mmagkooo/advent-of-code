use std::collections::{HashSet, VecDeque};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

fn is_wall(x: i32, y: i32, seed: i32) -> bool {
    let polynomial = x * x + 3 * x + 2 * x * y + y + y * y;
    let val = polynomial + seed;
    let ones = val.count_ones();
    return ones % 2 == 1;
}

const DIRS: [Point; 4] = [
    Point { x: -1, y: 0 },
    Point { x: 0, y: -1 },
    Point { x: 0, y: 1 },
    Point { x: 1, y: 0 },
];

fn main() {
    let argv: Vec<String> = std::env::args().collect();
    let seed: i32 = argv[1].parse().unwrap(); // office designer's favorite number

    let mut memo = HashSet::<Point>::new();
    let mut queue = VecDeque::<(Point, i32)>::new();
    let start = Point { x: 1, y: 1 };
    queue.push_back((start, 0));
    while !queue.is_empty() {
        let (current_point, steps_so_far) = queue.pop_front().unwrap();

        if current_point.x < 0 || current_point.y < 0 {
            continue;
        }

        if is_wall(current_point.x, current_point.y, seed) {
            continue;
        }

        if steps_so_far > 50 {
            continue;
        }

        if memo.contains(&current_point) {
            continue;
        }
        memo.insert(current_point);

        for direction in DIRS {
            let next_point = Point {
                x: current_point.x + direction.x,
                y: current_point.y + direction.y,
            };
            queue.push_back((next_point, steps_so_far + 1));
        }
    }

    println!("{}", memo.len());
}
