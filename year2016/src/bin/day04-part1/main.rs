use std::cmp::Ordering;
use std::collections::HashMap;
use std::io::{stdin, BufRead};

fn calculate_checksum(encrypted_data: String) -> String {
    let encrypted_data = encrypted_data.replace("-", "");
    let mut counter = HashMap::<char, i32>::new();
    encrypted_data.chars().for_each(|c| {
        *counter.entry(c).or_insert(0) += 1;
    });

    let mut pairs: Vec<_> = counter.iter().collect();
    pairs.sort_by(|pair_a, pair_b| {
        let value_comparison = pair_a.1.cmp(pair_b.1);
        if value_comparison == Ordering::Equal {
            pair_b.0.cmp(pair_a.0)
        } else {
            value_comparison
        }
    });

    return pairs.iter().map(|pair| pair.0).rev().take(5).collect();
}

fn main() {
    let sol: i32 = stdin()
        .lock()
        .lines()
        .map(|l| l.expect("Cannot read line"))
        .filter_map(|l| {
            let mut parts = l.strip_suffix("]").unwrap().split("[");
            let encrypted_data = parts.next().unwrap();
            let expected_checksum = parts.next().unwrap();

            let split_by_dash = encrypted_data.split("-");
            let mut encrypted_data: Vec<&str> = split_by_dash.collect();
            let sector_id: i32 = encrypted_data.last().unwrap().parse().unwrap();
            encrypted_data.pop();
            let encrypted_data = encrypted_data.join("");

            match calculate_checksum(encrypted_data) {
                checksum if checksum == expected_checksum => Some(sector_id),
                _ => None,
            }
        })
        .sum();

    println!("{}", sol);
}
