use std::{
    collections::HashMap,
    io::{stdin, BufRead},
};

fn main() {
    let lines: Vec<String> = stdin()
        .lock()
        .lines()
        .map(|l| l.expect("Could not read"))
        .collect();

    let line_width = lines[0].len();
    let mut counter_per_index: Vec<HashMap<char, i32>> = vec![HashMap::new(); line_width];
    for line in lines {
        for (i, c) in line.char_indices() {
            *counter_per_index[i].entry(c).or_insert(0) += 1;
        }
    }

    for counter in counter_per_index.iter() {
        let max_letter = counter.iter().max_by_key(|(_, count)| *count);
        print!("{}", max_letter.unwrap().0);
    }
    println!();
}
