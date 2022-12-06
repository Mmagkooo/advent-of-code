use std::io::{self, BufRead};

fn main() {
    let mut sol = 0;
    for line in io::stdin().lock().lines().map(|line| line.unwrap()) {
        let line_str = line.as_str();
        let half = line_str.len() / 2;
        let left = &line[..half];
        let right = &line[half..];

        const MEMO_SIZE: usize = 'z' as usize + 1; // last potentially indexable value
        let mut memo: [i32; MEMO_SIZE] = [0; MEMO_SIZE];
        for c in left.chars() {
            memo[c as usize] += 1;
        }

        for c in right.chars() {
            if memo[c as usize] != 0 {
                let value: i32;
                if c >= 'a' && c <= 'z' {
                    value = (c as i32) - ('a' as i32) + 1;
                } else {
                    value = (c as i32) - ('A' as i32) + 27;
                }
                sol += value;
                break;
            }
        }
    }

    println!("{sol}");
}
