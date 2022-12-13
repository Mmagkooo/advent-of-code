use std::{
    io::{self, BufRead},
};

fn main() {
    let lines = io::stdin().lock().lines();
    let lines: Vec<String> = lines
        .map(|line| line.expect("Could not read line"))
        .collect();

    for line_i in (0..lines.len()).step_by(3) {
        let left: Vec<char> = lines[line_i].chars().collect();
        let right: Vec<char> = lines[line_i + 1].chars().collect();

        let left_level = 0;
        let right_level = 0;

        let mut char_i: usize = 0;
        loop {
            // TODO check indices
            let left_char = left[char_i];
            let right_char = right[char_i];

        }
    }
}
