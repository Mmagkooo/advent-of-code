use std::io::{stdin, BufRead};

struct Disc {
    positions: i32,
    current_position: i32,
}

fn main() {
    let lines: Vec<String> = stdin()
        .lock()
        .lines()
        .map(|l| l.expect("Cannot read line"))
        .collect();

    let mut discs = vec![];

    for line in lines.iter() {
        let words: Vec<&str> = line.split_whitespace().collect();
        discs.push(Disc {
            positions: words[3].parse().unwrap(),
            current_position: words
                .last()
                .unwrap()
                .strip_suffix(".")
                .unwrap()
                .parse()
                .unwrap(),
        });
    }

    let mut start_time = 0;
    loop {
        let mut found = true;
        for (i, disc) in discs.iter().enumerate() {
            let disc_i = i as i32 + 1;
            if (disc_i + disc.current_position + start_time) % disc.positions != 0 {
                found = false;
                break;
            }
        }

        if found {
            break;
        } else {
            start_time += 1;
        }
    }
    println!("{start_time}");
}
