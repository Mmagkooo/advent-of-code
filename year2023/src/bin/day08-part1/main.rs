use std::collections::HashMap;

use regex::Regex;

type Label = String;

struct NodeNeighbors {
    left: Label,
    right: Label,
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

    let mut instructions = instructions_line.chars().cycle();
    let final_label = String::from("ZZZ");
    let mut current_label = String::from("AAA");
    let mut step_count = 0;
    while current_label != final_label {
        let instruction = instructions.next().unwrap();
        let current_neighbors = neighbors.get(&current_label).unwrap();
        current_label = match instruction {
            'L' => current_neighbors.left.clone(),
            'R' => current_neighbors.right.clone(),
            invalid => panic!("Not a valid instruction: {invalid}"),
        };
        step_count += 1;
    }

    println!("{step_count}");
}
