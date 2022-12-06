use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("Failed reading file");

    let mut max_sack_sum = 0;
    for raw_sack in contents.trim().split("\n\n") {
        let sack = raw_sack.split("\n");

        let mut sack_sum = 0;
        for food in sack {
            sack_sum += food.parse::<i32>().unwrap();
        }

        if sack_sum > max_sack_sum {
            max_sack_sum = sack_sum;
        }
    }

    println!("{}", max_sack_sum);
}