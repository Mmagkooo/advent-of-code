use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

const USAGE: &str = "<MARKER_LEN> <INPUT_FILE>";

fn main() {
    // to practice reading input from a file specified in argv
    let args: Vec<String> = env::args().collect();
    let marker_len = args[1].parse::<usize>().expect(USAGE);

    let filename = &args[2];
    let file = File::open(filename).expect(USAGE);

    let mut reader = BufReader::new(file);
    let mut line = String::new();
    reader.read_line(&mut line).expect("Could not read file");

    for i in 0..line.len() {
        let slice = &line[i..i + marker_len];
        let mut set: HashSet<char> = HashSet::new();
        set.extend(slice.chars());
        if set.len() == marker_len {
            let sol = i + marker_len;
            println!("{sol}");
            break;
        }
    }
}
