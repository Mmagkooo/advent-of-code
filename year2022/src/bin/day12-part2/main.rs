use std::{
    collections::{HashMap, VecDeque},
    io::{self, BufRead},
};

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
struct Point {
    i: i32,
    j: i32,
}

type Matrix<T> = Vec<Vec<T>>;

const START_SYMBOL: &str = "S";
const END_SYMBOL: &str = "E";
const INF: i32 = std::i32::MAX;
const DIRS: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

fn get_neighbors(point: &Point, matrix: &Matrix<i32>) -> Vec<Point> {
    let Point { i, j } = *point;
    let altitude = matrix[i as usize][j as usize];

    let height = matrix.len() as i32;
    let width = matrix[0].len() as i32;
    DIRS.iter()
        .map(|(di, dj)| Point {
            i: i + di,
            j: j + dj,
        })
        .filter(|p| p.i >= 0 && p.i < height && p.j >= 0 && p.j < width)
        .filter(|p| matrix[p.i as usize][p.j as usize] - altitude <= 1)
        .collect()
}

fn main() {
    let lines = io::stdin().lock().lines();

    let mut start_point: Option<Point> = None;
    let mut end_point: Option<Point> = None;
    let mut matrix: Matrix<i32> = vec![];
    for (i, line) in lines.enumerate() {
        let line = line.expect("Could not read line");
        let line = line.trim();
        matrix.push(line.chars().map(|c| c as i32).collect());

        let start_search = line.find(START_SYMBOL);
        if start_search.is_some() {
            start_point = Some(Point {
                i: i as i32,
                j: start_search.unwrap() as i32,
            });
        }

        let end_search = line.find(END_SYMBOL);
        if end_search.is_some() {
            end_point = Some(Point {
                i: i as i32,
                j: end_search.unwrap() as i32,
            });
        }
    }

    let start_point = start_point.expect("No start_point specified");
    let end_point = end_point.expect("No end_point specified");

    matrix[start_point.i as usize][start_point.j as usize] = 'a' as i32;
    matrix[end_point.i as usize][end_point.j as usize] = 'z' as i32;

    let mut start_points: Vec<Point> = vec![];
    for (i, row) in matrix.iter().enumerate() {
        for (j, altitude) in row.iter().enumerate() {
            if *altitude == 'a' as i32 {
                start_points.push(Point {
                    i: i as i32,
                    j: j as i32,
                });
            }
        }
    }

    let mut candidates: Vec<i32> = vec![];
    for start_point in start_points.iter() {
        let mut distances: HashMap<Point, i32> = HashMap::new();
        let mut deck: VecDeque<(Point, i32)> = VecDeque::new();
        deck.push_front((start_point.clone(), 0));
        while !deck.is_empty() {
            let (current_point, current_dist) = deck.pop_back().unwrap();
            let old_dist = match distances.get(&current_point) {
                Some(dist) => *dist,
                None => INF,
            };
    
            if current_dist >= old_dist {
                continue;
            }
    
            distances.insert(current_point.clone(), current_dist);
            if current_point == end_point {
                continue;
            }
    
            for neighbor in get_neighbors(&current_point, &matrix) {
                deck.push_front((neighbor, current_dist + 1));
            }
        }

        // from some starting points it might not be possible to reach the end
        match distances.get(&end_point) {
            Some(dist) => candidates.push(*dist),
            None => ()
        }
    }

    println!("{}", candidates.iter().min().unwrap());
}
