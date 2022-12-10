use std::{io::{self, BufRead}, cmp::max};

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

    let mut max_scenic_score = 0;
    for i in 1..height - 1 {
        for j in 1..width - 1 {
            let curr_height = forest[i][j];

            // go left
            let mut score_left = 0;
            for j in (0..j).rev() {
                score_left += 1;
                if curr_height <= forest[i][j] {
                    break;
                }
            }

            let mut score_right = 0;
            for j in j+1..width {
                score_right += 1;
                if curr_height <= forest[i][j] {
                    break;
                }
            }

            let mut score_up = 0;
            for i in (0..i).rev() {
                score_up += 1;
                if curr_height <= forest[i][j] {
                    break;
                }
            }

            let mut score_down = 0;
            for i in i+1..height {
                score_down += 1;
                if curr_height <= forest[i][j] {
                    break;
                }
            }

            let scenic_score = score_left * score_right * score_up * score_down;
            max_scenic_score = max(scenic_score, max_scenic_score);
        }
    }
    println!("{max_scenic_score}");
}
