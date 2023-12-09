use std::collections::HashMap;

use regex::Regex;

type Label = String;

struct NodeNeighbors {
    left: Label,
    right: Label,
}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: u64, b: u64) -> u64 {
    a * b / gcd(a, b)
}

fn main() {
    let mut lines = std::io::stdin().lines().map(|l| l.unwrap());
    let instructions_line = lines.next().unwrap();

    assert!(lines.next().unwrap().is_empty()); // empty line

    // load mapping
    let re = Regex::new(r"^(.*) = \((.*), (.*)\)$").unwrap();
    let neighbors: HashMap<Label, NodeNeighbors> = lines
        .map(|l| {
            let (_, [label, left, right]) = re.captures(&l).unwrap().extract::<3>();
            let neighbors = NodeNeighbors {
                left: left.into(),
                right: right.into(),
            };
            (label.into(), neighbors)
        })
        .collect();

    // iterate over labels ending with A, count steps until Z for each, find lcm of counts
    let sol = neighbors
        .keys()
        .filter(|label| label.ends_with("A"))
        .map(|mut current_label| {
            let mut instructions = instructions_line.chars().cycle();
            let mut step_count: u64 = 0;
            while !current_label.ends_with("Z") {
                let instruction = instructions.next().unwrap();
                let current_neighbors = neighbors.get(current_label).unwrap();
                current_label = match instruction {
                    'L' => &current_neighbors.left,
                    'R' => &current_neighbors.right,
                    invalid => panic!("Not a valid instruction: {invalid}"),
                };
                step_count += 1;
            }
            step_count
        })
        .reduce(lcm)
        .unwrap();
    println!("{sol}");
}
