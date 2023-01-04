use std::{
    cmp::max,
    collections::HashMap,
    io::{stdin, BufRead},
};

type Point = (i32, i32);
type Direction = Point;
const DIRECTIONS: [Direction; 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

fn progress_point(point: Point, direction: Direction, height: i32, width: i32) -> Point {
    return (
        (point.0 + direction.0 + height) % height,
        (point.1 + direction.1 + width) % width,
    );
}

fn main() {
    let lines: Vec<String> = stdin()
        .lock()
        .lines()
        .map(|line| line.expect("Could not read line"))
        .collect();

    let mut field: HashMap<Point, char> = HashMap::new();
    let mut start_point_j: Option<i32> = None;
    let height = lines.len() as i32 - 2;
    let mut width = -1; // find

    for (i, line) in lines.iter().enumerate() {
        width = max(width, line.len() as i32);
        for (j, char) in line.chars().enumerate() {
            if "#.".contains(char) {
                let point = (i as i32, j as i32);
                field.insert(point, char);
            }

            if i == 0 && start_point_j.is_none() && char == '.' {
                start_point_j = Some(j as i32);
            }
        }
    }

    let commands_line = lines.last().expect("No last line");
    let chars: Vec<char> = commands_line.chars().collect();
    let chars_len = chars.len();
    let mut char_i = 0;

    let mut direction_index: i32 = 0;
    let mut curr_point: Point = (0, start_point_j.unwrap());
    while char_i < chars_len {
        if chars[char_i].is_digit(10) {
            let mut steps = 0;
            while char_i < chars_len && chars[char_i].is_digit(10) {
                steps = steps * 10 + chars[char_i] as i32 - '0' as i32;
                char_i += 1;
            }

            let direction = DIRECTIONS[direction_index as usize];
            'movement_loop: for _ in 0..steps {
                // find next walkable '.'
                let mut next_point_kind: Option<char> = None;
                let last_walkable_point = curr_point.clone();
                'find_next_loop: while next_point_kind.is_none() {
                    let next_point = progress_point(curr_point, direction, height, width);
                    next_point_kind = field.get(&next_point).copied();
                    match next_point_kind {
                        Some('#') => {
                            curr_point = last_walkable_point;
                            break 'movement_loop; // to speed up
                        }
                        Some('.') => {
                            curr_point = next_point;
                            break 'find_next_loop;
                        }
                        None => (), // found ' '
                        Some(invalid) => {
                            panic!("Invalid char: {invalid}");
                        }
                    };
                    curr_point = next_point;
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

    let sol = (curr_point.0 + 1) * 1000 + (curr_point.1 + 1) * 4 + direction_index;
    println!("{sol}");
}
