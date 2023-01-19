use std::io::{stdin, BufRead};

fn main() {
    let sol = stdin()
        .lock()
        .lines()
        .map(|l| l.expect("Cannot read line"))
        .map(|l| {
            l.split_whitespace()
                .map(|str| str.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .filter(|sides| {
            sides[0] < sides[1] + sides[2]
                && sides[1] < sides[0] + sides[2]
                && sides[2] < sides[0] + sides[1]
        })
        .count();

    println!("{}", sol);
}
