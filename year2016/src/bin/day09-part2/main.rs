use std::io::stdin;

fn decompress(s: &str) -> usize {
    let mut i = 0;
    let mut sol = 0;
    while i < s.len() {
        let c = &s[i..i + 1];
        if c == "(" {
            i += 1; // skip opening parenthesis
            let marker_start = i;
            while &s[i..i + 1] != ")" {
                i += 1;
            }

            let marker_end = i; // not inclusive
            let raw_marker = &s[marker_start..marker_end];

            let mut parts = raw_marker.split("x");

            let sequence_length: usize = parts.next().unwrap().parse().unwrap();
            let times: usize = parts.next().unwrap().parse().unwrap();

            i += 1; // skip closing bracket

            sol += decompress(&s[i..i + sequence_length]) * times;
            i += sequence_length;
        } else {
            sol += 1;
            i += 1;
        }
    }

    return sol;
}

fn main() {
    let mut line: String = String::new();
    stdin().read_line(&mut line).expect("Could not read line");
    let sol = decompress(line.trim());
    println!("{}", sol);
}
