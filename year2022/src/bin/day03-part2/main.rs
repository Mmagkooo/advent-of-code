use std::io::{self, BufRead};

const GROUP_SIZE: i32 = 3;
const MEMO_SIZE: usize = 'z' as usize + 1; // last potentially indexable value

fn main() {
    let mut sol: i32 = 0;
    let mut i: i32 = 0;
    let mut memo: [[bool; GROUP_SIZE as usize]; MEMO_SIZE] = [[false; GROUP_SIZE as usize]; MEMO_SIZE];
    for line in io::stdin().lock().lines().map(|line| line.unwrap()) {
        let line_str = line.as_str();

        for c in line_str.chars() {
            memo[c as usize][i as usize] = true;
        }

        i += 1;
        if i == GROUP_SIZE {
            // check which char has three occurrences
            for c in line_str.chars() {
                if memo[c as usize] == [true, true, true] {
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

            // reset memo
            for triplet in memo.iter_mut() {
                *triplet = [false, false, false];
            }

            // reset counter
            i = 0;
        }
    }

    println!("{sol}");
}
