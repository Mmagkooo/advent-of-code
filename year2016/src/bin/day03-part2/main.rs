use std::io::{stdin, BufRead};

fn is_triangle(a: i32, b: i32, c: i32) -> bool {
    a + b > c && a + c > b && b + c > a
}

fn main() {
    let data = stdin()
        .lock()
        .lines()
        .map(|l| l.expect("Cannot read line"))
        .map(|l| {
            l.split_whitespace()
                .map(|str| str.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();

    let mut sol = 0;
    for col in 0..3 {
        for row in (0..data.len()).step_by(3) {
            if is_triangle(data[row][col], data[row + 1][col], data[row + 2][col]) {
                sol += 1;
            }
        }
    }

    println!("{}", sol);
}
