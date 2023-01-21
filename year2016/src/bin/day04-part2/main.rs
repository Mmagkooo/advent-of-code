use std::io::{stdin, BufRead};

fn shift(encrypted_data: String, by: u32) -> String {
    encrypted_data
        .chars()
        .map(|c| match c {
            '-' => ' ',
            letter @ 'a'..='z' => char::from_u32((letter as u32 - 'a' as u32 + by) % 26 + 'a' as u32).unwrap(),
            invalid_char => panic!("Invalid char: {invalid_char}"),
        })
        .collect()
}

fn main() {
    stdin()
        .lock()
        .lines()
        .map(|l| l.expect("Cannot read line"))
        .for_each(|l| {
            let mut parts = l.strip_suffix("]").unwrap().split("[");
            let encrypted_data = parts.next().unwrap();

            let split_by_dash = encrypted_data.split("-");
            let mut encrypted_data: Vec<&str> = split_by_dash.collect();
            let by: u32 = encrypted_data.last().unwrap().parse().unwrap();
            encrypted_data.pop();
            let encrypted_data = encrypted_data.join("-");

            let shifted = shift(encrypted_data, by);
            if shifted == "northpole object storage" {
                println!("{by}");
            }
        });
}
