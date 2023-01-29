use std::{
    collections::{HashMap, HashSet},
    io::{stdin, BufRead},
};

type Field = Vec<Vec<char>>;

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
struct Point {
    i: i32,
    j: i32,
}

// map from point to (point with distance)
type Graph = HashMap<Point, Vec<(Point, i32)>>;

const DIRECTIONS: [Point; 4] = [
    Point { i: 1, j: 0 },
    Point { i: -1, j: 0 },
    Point { i: 0, j: 1 },
    Point { i: 0, j: -1 },
];

const WALL: char = '#';

fn find_next_node(
    field: &Field,
    mut current_point: Point,
    visited: &mut HashSet<Point>,
) -> Option<(Point, i32)> {
    let mut distance = 1;
    loop {
        visited.insert(current_point);
        let c = field[current_point.i as usize][current_point.j as usize];
        if c.is_digit(10) {
            return Some((current_point, distance));
        }

        let mut free: Vec<Point> = vec![];
        for direction in DIRECTIONS {
            let next_point = Point {
                i: current_point.i + direction.i,
                j: current_point.j + direction.j,
            };

            let next_char = field[next_point.i as usize][next_point.j as usize];
            if next_char == WALL || visited.contains(&next_point) {
                continue;
            }

            free.push(next_point);
        }

        match free.len() {
            0 => {
                // dead end
                return None;
            }
            1 => {
                current_point = free[0];
                distance += 1;
            }
            _multiple => {
                // found the next intersection
                return Some((current_point, distance));
            }
        }
    }
}

fn chart(field: &Field, current_node: Point, graph: &mut Graph) {
    if graph.contains_key(&current_node) {
        return;
    }

    for direction in DIRECTIONS {
        let next_point = Point {
            i: current_node.i + direction.i,
            j: current_node.j + direction.j,
        };

        let next_char = field[next_point.i as usize][next_point.j as usize];
        if next_char == WALL {
            continue;
        }

        let mut visited = HashSet::new();
        visited.insert(current_node);
        match find_next_node(field, next_point, &mut visited) {
            None => continue,
            Some((next_node, distance)) => {
                // intentionally not putting both edges
                graph
                    .entry(current_node)
                    .or_insert(vec![])
                    .push((next_node, distance));
                chart(field, next_node, graph);
            }
        }
    }
}

fn find_digits(field: &Field, digit_to_point: &mut HashMap<Point, char>) {
    for i in 0..field.len() {
        for j in 0..field[i].len() {
            let c = field[i][j];
            if c.is_digit(10) {
                let p = Point {
                    i: i as i32,
                    j: j as i32,
                };
                digit_to_point.insert(p, c);
            }
        }
    }
}

// point, visited_vector
type MemoConfig = (Point, u32);

fn find_min_path(
    current_node: Point,
    graph: &Graph,
    visited_vector: u32,
    point_to_digit: &HashMap<Point, char>,
    path: i32,
    min_path: &mut i32,
    memo: &mut HashMap<MemoConfig, i32>,
) {
    // check end
    if visited_vector.count_ones() == point_to_digit.len() as u32 {
        // update min
        if path < *min_path {
            *min_path = path;
        }
        return;
    }

    // check if seen in a better state
    let config = (current_node, visited_vector);
    match memo.get(&config) {
        None => (),
        Some(old_value) if *old_value <= path => {
            return;
        }
        _ => (),
    }

    // update to the new better result
    memo.insert(config, path);

    for (neighbor, distance) in graph.get(&current_node).unwrap() {
        let new_visited_vector = match point_to_digit.get(neighbor) {
            Some(d) => visited_vector | (1u32 << d.to_digit(10).unwrap()),
            None => visited_vector,
        };

        find_min_path(
            *neighbor,
            graph,
            new_visited_vector,
            point_to_digit,
            path + distance,
            min_path,
            memo,
        );
    }
}

fn main() {
    let lines = stdin().lock().lines().map(|l| l.expect("Cannot read line"));
    let mut field: Field = vec![];

    for line in lines {
        field.push(line.chars().collect());
    }

    let mut point_to_digit = HashMap::new();
    find_digits(&field, &mut point_to_digit);

    let mut zero_point = Point { i: 0, j: 0 };
    for (point, digit) in point_to_digit.iter() {
        if *digit == '0' {
            zero_point = *point;
            break;
        }
    }

    let mut graph: Graph = HashMap::new();
    chart(&field, zero_point, &mut graph);

    let mut memo = HashMap::new();
    let mut min_path = i32::MAX;
    find_min_path(
        zero_point,
        &graph,
        0,
        &point_to_digit,
        0,
        &mut min_path,
        &mut memo,
    );
    println!("{min_path}");
}
