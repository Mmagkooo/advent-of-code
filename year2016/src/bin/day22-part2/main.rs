use std::{
    collections::HashMap,
    io::{stdin, BufRead},
};

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
struct Node {
    used: u32,
    total: u32,
}

fn parse_size(s: &str) -> u32 {
    s.strip_suffix("T").unwrap().parse().unwrap()
}

fn parse_coordinate(s: &str) -> i32 {
    s[1..].parse().unwrap()
}

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
struct Point {
    x: i32,
    y: i32,
}

// impl Point {
//     // manhattan distance: |point.x - 0| + |point.y - 0|
//     fn heuristic(&self) -> i32 {
//         self.x + self.y
//     }
// }

// const NEIGHBOR_DELTAS: [Point; 4] = [
//     Point { x: -1, y: 0 },
//     Point { x: 1, y: 0 },
//     Point { x: 0, y: -1 },
//     Point { x: 0, y: 1 },
// ];

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
struct Config {
    goal: Point,
    empty: Point, // doesn't have to be hashed - is a function of nodes; this is a property only to prevent traversing nodes
    nodes: Vec<Vec<Node>>,
}

#[derive(PartialEq, Eq)]
struct HeapConfig {
    config: Config,
    steps: i32,
    sorting_metric: i32,
}

impl PartialOrd for HeapConfig {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for HeapConfig {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // reverse because smallest first
        self.sorting_metric.cmp(&other.sorting_metric).reverse()
    }
}

fn map_to_vec_of_vecs<T>(map: &HashMap<Point, T>) -> Vec<Vec<T>>
where
    T: Copy,
{
    let max_x = *map.keys().map(|Point { x, y: _ }| x).max().unwrap();
    let max_y = *map.keys().map(|Point { x: _, y }| y).max().unwrap();
    let mut vec_of_vecs = vec![];
    for y in 0..=max_y {
        let mut row = vec![];
        for x in 0..=max_x {
            let el = map.get(&Point { x, y }).unwrap();
            row.push(*el);
        }
        vec_of_vecs.push(row);
    }
    return vec_of_vecs;
}

fn get_lines() -> Vec<String> {
    stdin()
        .lock()
        .lines()
        .skip(2)
        .map(|l| l.expect("Cannot read"))
        .collect()
}

fn get_nodes(lines: &Vec<String>) -> Vec<Vec<Node>> {
    let mut nodes = HashMap::<Point, Node>::new();
    let mut unused_nodes: Vec<Point> = vec![];
    for line in lines {
        let split: Vec<&str> = line.split_whitespace().collect();
        let node = Node {
            total: parse_size(split[1]),
            used: parse_size(split[2]),
        };

        let path = split[0];
        let path_split: Vec<&str> = path.split("-").collect();
        let x = parse_coordinate(path_split[1]);
        let y = parse_coordinate(path_split[2]);
        let point = Point { x, y };
        nodes.insert(point, node);

        if node.used == 0 {
            unused_nodes.push(point);
        }
    }

    return map_to_vec_of_vecs(&nodes);
}

fn main() {
    let lines = get_lines();
    let initial_nodes = get_nodes(&lines);

    for row in initial_nodes {
        for node in row {
            print!("[{:3}/{:3}] ", node.used, node.total);
        }
        println!();
    }

    // Ignore the derived and implemented traits, they were needed before when I thought I would solve this using a search algorithm (e.g. BFS, A*)
    println!("After inspecting the printed nodes, we can see that they are mostly swappable, except for the wall which needs to be bypassed.
Bring the empty node just left of the top-right node (0 is empty, G is goal):
    Z 0 G
    W X Y
To move 0 and G left by one, five steps are required:
    Z G 0
    W X Y
    ------
    Z G Y
    W X 0
    ------
    Z G Y
    W 0 X
    ------
    Z G Y
    0 W X
    ------
    0 G Y
    Z W X
This needs to be repeated (W - 2) times, where W is the width of the memory field. Then after the top left node becomes the empty node, i.e. when the field looks like this:
    0 G A B C ...
    J K L M N ...
Just one more step is required to swap 0 and G, which means the total number of steps required was:
    (initial moving of empty) + 5 * (W - 2) + 1
");
}
