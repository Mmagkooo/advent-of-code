use std::io::{stdin, BufRead};

use itertools::Itertools;

/**
 * Get next rock
 * Position it (2 from the left edge, 3 above the highest point)
 * while (can move down):
 *   - jet-move left/right
 *   - move down
 */

/**
 * Denoting common x and y Cartesian coordinates
 */
#[derive(Clone, Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

const DOWN: Point = Point { x: 0, y: -1 };
const LEFT: Point = Point { x: -1, y: 0 };
const RIGHT: Point = Point { x: 1, y: 0 };

#[derive(Clone, Copy, Debug)]
enum RockShape {
    Horizontal,
    Cross,
    ReverseL,
    Vertical,
    Square,
}

const CHAMBER_WIDTH: usize = 7;

#[derive(Clone, Debug)]
struct Rock {
    occupied: Vec<Point>,
}

impl Rock {
    fn translate(&self, direction: Point) -> Self {
        let mut copy = self.clone();
        copy.occupied = copy
            .occupied
            .iter()
            .map(|p| Point {
                x: p.x + direction.x,
                y: p.y + direction.y,
            })
            .collect();
        return copy;
    }
}

fn is_legal(rock: &Rock, chamber: &Rock) -> bool {
    for point in rock.occupied.iter() {
        if point.y < 0 {
            return false;
        }

        if point.x < 0 || point.x >= CHAMBER_WIDTH as i32 {
            return false;
        }

        for chamber_point in chamber.occupied.iter() {
            if point == chamber_point {
                return false;
            }
        }
    }

    return true;
}

fn create_rock(chamber_top: i32, rock_shape: RockShape) -> Rock {
    let occupied: Vec<Point> = match rock_shape {
        RockShape::Horizontal => (0..4)
            .map(|x| Point {
                x: x + 2,
                y: chamber_top + 3,
            })
            .collect(),
        RockShape::Cross => [(0, 0), (0, -1), (0, 1), (-1, 0), (1, 0)]
            // (0, 0) is the centre of the cross
            .into_iter()
            .map(|(x, y)| Point {
                x: x + 2 + 1,
                y: y + chamber_top + 3 + 1,
            })
            .collect(),
        RockShape::ReverseL => [(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)]
            .into_iter()
            .map(|(x, y)| Point {
                x: x + 2,
                y: y + chamber_top + 3,
            })
            .collect(),
        RockShape::Vertical => (0..4)
            .map(|y| Point {
                x: 2,
                y: y + chamber_top + 3,
            })
            .collect(),
        RockShape::Square => (0..2)
            .cartesian_product(0..2)
            .map(|(x, y)| Point {
                x: x + 2,
                y: y + chamber_top + 3,
            })
            .collect(),
    };

    return Rock { occupied };
}

fn get_height(rock: &Rock) -> i32 {
    return rock.occupied.iter().map(|p| p.y).max().unwrap_or(-1) + 1;
}

fn main() {
    let mut rock_shapes = [
        RockShape::Horizontal,
        RockShape::Cross,
        RockShape::ReverseL,
        RockShape::Vertical,
        RockShape::Square,
    ]
    .iter()
    .cycle();

    let line = stdin().lock().lines().nth(0).unwrap().unwrap();
    let mut jet = line.chars().cycle();

    let mut chamber = Rock { occupied: vec![] };
    for _ in 0..2022 {
        let rock_shape = rock_shapes.next().unwrap();
        let chamber_height = get_height(&chamber);
        let mut rock = create_rock(chamber_height, *rock_shape);

        loop {
            let jet_char = jet.next().unwrap();
            let jet_direction = match jet_char {
                '>' => RIGHT,
                '<' => LEFT,
                invalid => panic!("Invalid char in jet stream: {invalid}"),
            };

            let after_jet = rock.translate(jet_direction);
            if is_legal(&after_jet, &chamber) {
                rock = after_jet;
            }

            let after_down = rock.translate(DOWN);
            if !is_legal(&after_down, &chamber) {
                break;
            }
            rock = after_down;
        }

        chamber.occupied.extend(rock.occupied);
    }

    println!("{}", get_height(&chamber));
}
