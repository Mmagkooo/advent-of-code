use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();

    let mut forest: Vec<Vec<i32>> = vec![];
    for line in lines {
        let row: Vec<i32> = line
            .unwrap()
            .chars()
            .map(|c| c as i32 - '0' as i32)
            .collect();
        forest.push(row);
    }

    let height = forest.len();
    let width = forest[0].len();
    let mut visible: Vec<Vec<bool>> = vec![vec![false; width]; height];

    for i in 1..height - 1 {
        let mut tmp_max = forest[i][0];
        for j in 1..width - 1 {
            if tmp_max < forest[i][j] {
                tmp_max = forest[i][j];
                visible[i][j] = true;
            }
        }

        let mut tmp_max = forest[i][width-1];
        for j in (1..width - 1).rev() {
            if tmp_max < forest[i][j] {
                tmp_max = forest[i][j];
                visible[i][j] = true;
            }
        }
    }

    for j in 1..width - 1 {
        let mut tmp_max = forest[0][j];
        for i in 1..height - 1 {
            if tmp_max < forest[i][j] {
                tmp_max = forest[i][j];
                visible[i][j] = true;
            }
        }

        let mut tmp_max = forest[height-1][j];
        for i in (1..height - 1).rev() {
            if tmp_max < forest[i][j] {
                tmp_max = forest[i][j];
                visible[i][j] = true;
            }
        }
    }

    let mut visible_count = (width as i32) * 2 + (height as i32) * 2 - 4;
    println!("visible by default: {visible_count}");
    for row in &visible {
        for value in row {
            visible_count += *value as i32;
        }
        let printable_row = row
            .clone()
            .iter()
            .map(|v| if *v { "1" } else { "0" })
            .collect::<Vec<&str>>()
            .join("");
        println!("{}", printable_row)
    }
    println!("total visible: {visible_count}");
}
