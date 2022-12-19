use std::io::{stdin, BufRead};

fn main() {
    let cubes: Vec<Vec<i32>> = stdin()
        .lock()
        .lines()
        .map(|l| l.expect("Could not read line"))
        .map(|s| s.split(",").map(|d| d.parse().unwrap()).collect())
        .collect();

    let cubes_num = cubes.len();
    let mut sol = cubes_num * 6;
    for ci in 0..cubes_num {
        for cj in ci + 1..cubes_num {
            let mut diff = 0;
            for i in 0..3 {
                diff += (cubes[ci][i] - cubes[cj][i]).abs();
            }
            if diff == 1 {
                sol -= 2;
            }
        }
    }

    println!("{sol}");
}
