use std::{
    collections::HashMap,
    io::{stdin, BufRead},
};

type Point = (i32, i32);
type Direction = Point;
const DIRECTIONS: [Direction; 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

const HEIGHT: i32 = 50;
const WIDTH: i32 = 50;

/**
 * Made a real-life 3d cube out of paper with faces labeled 1-6
 * according to my input data:
 * x12
 * x3x
 * 45x
 * 6xx
 *
 * Returns (new face, new point, new direction index)
 */
fn progress_point(face: i32, point: Point, direction_index: i32) -> (i32, Point, i32) {
    let direction = DIRECTIONS[direction_index as usize];
    let new_point = (point.0 + direction.0, point.1 + direction.1);

    let (new_face, new_point, new_direction_index) = match face {
        1 => {
            if new_point.0 < 0 {
                (6, (new_point.1, 0), 0)
            } else if new_point.0 >= HEIGHT {
                (3, (0, new_point.1), direction_index)
            } else if new_point.1 < 0 {
                (4, (HEIGHT - 1 - new_point.0, 0), 0)
            } else if new_point.1 >= WIDTH {
                (2, (new_point.0, 0), direction_index)
            } else {
                (face, new_point, direction_index)
            }
        }
        2 => {
            if new_point.0 < 0 {
                (6, (HEIGHT - 1, new_point.1), direction_index)
            } else if new_point.0 >= HEIGHT {
                (3, (new_point.1, WIDTH - 1), 2)
            } else if new_point.1 < 0 {
                (1, (new_point.0, WIDTH - 1), direction_index)
            } else if new_point.1 >= WIDTH {
                (5, (HEIGHT - 1 - new_point.0, WIDTH - 1), 2)
            } else {
                (face, new_point, direction_index)
            }
        }
        3 => {
            if new_point.0 < 0 {
                (1, (HEIGHT - 1, new_point.1), direction_index)
            } else if new_point.0 >= HEIGHT {
                (5, (0, new_point.1), direction_index)
            } else if new_point.1 < 0 {
                (4, (0, new_point.0), 1)
            } else if new_point.1 >= WIDTH {
                (2, (HEIGHT - 1, new_point.0), 3)
            } else {
                (face, new_point, direction_index)
            }
        }
        4 => {
            if new_point.0 < 0 {
                (3, (new_point.1, 0), 0)
            } else if new_point.0 >= HEIGHT {
                (6, (0, new_point.1), direction_index)
            } else if new_point.1 < 0 {
                (1, (HEIGHT - 1 - new_point.0, 0), 0)
            } else if new_point.1 >= WIDTH {
                (5, (new_point.0, 0), direction_index)
            } else {
                (face, new_point, direction_index)
            }
        }
        5 => {
            if new_point.0 < 0 {
                (3, (HEIGHT - 1, new_point.1), direction_index)
            } else if new_point.0 >= HEIGHT {
                (6, (new_point.1, WIDTH - 1), 2)
            } else if new_point.1 < 0 {
                (4, (new_point.0, WIDTH - 1), direction_index)
            } else if new_point.1 >= WIDTH {
                (2, (HEIGHT - 1 - new_point.0, WIDTH - 1), 2)
            } else {
                (face, new_point, direction_index)
            }
        }
        6 => {
            if new_point.0 < 0 {
                (4, (HEIGHT - 1, new_point.1), direction_index)
            } else if new_point.0 >= HEIGHT {
                (2, (0, new_point.1), direction_index)
            } else if new_point.1 < 0 {
                (1, (0, new_point.0), 1)
            } else if new_point.1 >= WIDTH {
                (5, (HEIGHT - 1, new_point.0), 3)
            } else {
                (face, new_point, direction_index)
            }
        }
        _ => panic!("Invalid face: {face}"),
    };

    return (new_face, new_point, new_direction_index);
}

fn main() {
    let lines: Vec<String> = stdin()
        .lock()
        .lines()
        .map(|line| line.expect("Could not read line"))
        .collect();

    let mut cube: Vec<HashMap<Point, char>> = vec![HashMap::new(); 6 + 1]; // index 0 won't be used

    for (i, line) in lines.iter().enumerate() {
        let i = i as i32;
        for (j, char) in line.chars().enumerate() {
            let j = j as i32;
            if "#.".contains(char) {
                let face = if i < HEIGHT {
                    if j < 2 * WIDTH {
                        1
                    } else {
                        2
                    }
                } else if i < 2 * HEIGHT {
                    3
                } else if i < 3 * HEIGHT {
                    if j < WIDTH {
                        4
                    } else {
                        5
                    }
                } else {
                    6
                };

                let face_field = &mut cube[face];
                let point = (i % HEIGHT, j % WIDTH);
                face_field.insert(point, char);
            }
        }
    }

    let commands_line = lines.last().expect("No last line");
    let chars: Vec<char> = commands_line.chars().collect();
    let chars_len = chars.len();
    let mut char_i = 0;

    let mut direction_index: i32 = 0;
    let mut curr_face = 1;
    let mut curr_point: Point = (0, 0);
    while char_i < chars_len {
        if chars[char_i].is_digit(10) {
            let mut steps = 0;
            while char_i < chars_len && chars[char_i].is_digit(10) {
                steps = steps * 10 + chars[char_i] as i32 - '0' as i32;
                char_i += 1;
            }

            for _ in 0..steps {
                let (next_face, next_point, next_direction_index) =
                    progress_point(curr_face, curr_point, direction_index);
                let next_point_kind = cube[next_face as usize].get(&next_point);
                (curr_face, curr_point, direction_index) = match next_point_kind {
                    Some('.') => (next_face, next_point, next_direction_index),
                    Some('#') => break,
                    Some(_) => panic!("Invalid field"),
                    None => panic!("Non-existent field"),
                }
            }
        } else if "RL".contains(chars[char_i]) {
            let index_change = match chars[char_i] {
                'R' => 1,
                'L' => -1,
                _ => panic!(),
            };
            let dirs = DIRECTIONS.len() as i32;
            direction_index = (direction_index + index_change + dirs) % dirs;
            char_i += 1;
        } else {
            panic!("Invalid char: {}", chars[char_i]);
        }
    }

    println!("Face: {curr_face}, point: {curr_point:?}, direction_index: {direction_index}");
    println!("Consider the face number and convert to row * 1000 + col * 4 + dir");
    // decided it makes more sense not to implement the final conversion logic
}
