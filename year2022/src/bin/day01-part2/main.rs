use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("Failed reading file");

    let mut sack_sums: Vec<i32> = vec![];
    for raw_sack in contents.trim().split("\n\n") {
        let sack = raw_sack.split("\n");

        let mut sack_sum = 0;
        for food in sack {
            sack_sum += food.parse::<i32>().unwrap();
        }

        sack_sums.push(sack_sum);
    }

    sack_sums.sort();
    sack_sums.reverse();
    println!("{}", sack_sums[0] + sack_sums[1] + sack_sums[2]);
}
