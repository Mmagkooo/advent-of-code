use std::{
    cmp::max,
    collections::HashMap,
    fmt::Display,
    io,
    ops::{Add, AddAssign},
};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

static LEFT_DOWN: Point = Point { x: -1, y: 1 };
static RIGHT_DOWN: Point = Point { x: 1, y: 1 };

impl AddAssign for Point {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl Add for Point {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

#[derive(Clone, Debug)]
enum Element {
    // Air,
    Rock,
    Sand,
}

impl Display for Element {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            // Element::Air => write!(f, "."),
            Element::Rock => write!(f, "#"),
            Element::Sand => write!(f, "o"),
        }
    }
}

fn raw_pair_to_point(raw_pair: &str) -> Point {
    let mut pair_iterator = raw_pair.split(",").map(|v| v.parse().unwrap());
    let x = pair_iterator.next().unwrap();
    let y = pair_iterator.next().unwrap();
    assert!(pair_iterator.next().is_none());
    return Point { x, y };
}

type Matrix = HashMap<Point, Element>;

fn put_sand(matrix: &mut Matrix, bottom: i32) -> Point {
    let mut current_point = Point { x: 500, y: 0 };

    while current_point.y < bottom {
        let down_point = Point {
            x: current_point.x,
            y: current_point.y + 1,
        };

        let down_element = matrix.get(&down_point);
        if down_element.is_none() {
            current_point = down_point;
            continue;
        }

        let left_down_point = current_point + LEFT_DOWN;
        let left_down_element = matrix.get(&left_down_point);
        if left_down_element.is_none() {
            current_point = left_down_point;
            continue;
        }

        let right_down_point = current_point + RIGHT_DOWN;
        let right_down_element = matrix.get(&right_down_point);
        if right_down_element.is_none() {
            current_point = right_down_point;
            continue;
        }

        // otherwise sand has reached end
        break;
    }

    matrix.insert(current_point, Element::Sand);
    return current_point;
}

// fn print_matrix(matrix: &Matrix, bottom: i32) {
//     for y in 0..bottom + 1 {
//         for x in 450..550 {
//             let el = matrix.get(&Point { x, y }).unwrap_or(&Element::Air);
//             print!("{el}");
//         }
//         println!();
//     }
// }

fn main() {
    let lines = io::stdin()
        .lines()
        .map(|line| line.expect("Could not read line"));

    let mut matrix: Matrix = HashMap::new();

    // load rocks
    for line in lines {
        let mut points = line.split(" -> ").map(raw_pair_to_point);
        let mut current_point = points.next().unwrap();

        for point in points {
            let step = Point {
                x: (point.x - current_point.x).signum(),
                y: (point.y - current_point.y).signum(),
            };
            while current_point != point {
                matrix.insert(current_point, Element::Rock);
                current_point += step;
            }
            matrix.insert(current_point, Element::Rock);
        }
    }

    // find bottom
    let mut bottom = 0;
    for p in matrix.keys() {
        bottom = max(bottom, p.y);
    }

    // put sand
    let mut steps = 0;
    loop {
        let sand_point = put_sand(&mut matrix, bottom);
        if sand_point.y >= bottom {
            break;
        }
        // print_matrix(&matrix, bottom);
        steps += 1;
    }
    println!("{steps}");
}
