use std::io::{stdin, BufRead};
use itertools::Itertools;

struct Node {
    used: u32,
    avail: u32,
}

fn parse_size(s: &str) -> u32 {
    s.strip_suffix("T").unwrap().parse().unwrap()
}

fn main() {
    let nodes: Vec<Node> = stdin()
        .lock()
        .lines()
        .skip(2)
        .map(|l| l.expect("Cannot read"))
        .map(|l| {
            let split: Vec<&str> = l.split_whitespace().collect();
            Node {
                used: parse_size(split[2]),
                avail: parse_size(split[3]),
            }
        }).collect();
    
    let sol = nodes.iter().permutations(2).filter(|pair| {
        pair[0].used != 0 && pair[0].used <= pair[1].avail
    }).count();

    println!("{sol}");
}
