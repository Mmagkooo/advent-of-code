use std::io::{stdin, BufRead};

fn main() {
    let lines: Vec<String> = stdin()
        .lock()
        .lines()
        .map(|l| l.expect("Cannot read line"))
        .collect();
    let expected_length: usize = lines[0].parse().unwrap();
    let mut sequence = lines[1].clone();

    while sequence.len() < expected_length {
        let sequence_copy: String = sequence
            .chars()
            .rev()
            .map(|c| match c {
                '1' => '0',
                '0' => '1',
                inv => panic!("Invalid char: {inv}"),
            })
            .collect();
        sequence.push('0');
        sequence.push_str(&sequence_copy);
    }

    let mut checksum = sequence[..expected_length].to_string();
    while checksum.len() % 2 == 0 {
        let chars: Vec<char> = checksum.chars().collect();
        let mut next_checksum = String::new();
        for (i, c) in chars.iter().enumerate().step_by(2) {
            let next_c = chars[i + 1];
            next_checksum.push(match *c == next_c {
                true => '1',
                false => '0',
            });
        }
        checksum = next_checksum;
    }

    println!("{checksum}");
}
