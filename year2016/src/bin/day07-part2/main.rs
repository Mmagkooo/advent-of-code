use std::io::{stdin, BufRead};

fn supports_tls(line: &String) -> bool {
    let slices: Vec<&str> = line.split(|c| "[]".contains(c)).collect();
    assert!(&line[0..1] != "["); // assert starting with a non-bracketed substring
    let mut insides: Vec<&str> = vec![];
    let mut outsides: Vec<&str> = vec![];
    for (i, slice) in slices.iter().enumerate() {
        if i % 2 == 0 {
            outsides.push(slice);
        } else {
            insides.push(slice);
        }
    }

    for c1 in 'a'..='z' {
        for c2 in 'a'..='z' {
            if c1 == c2 {
                continue;
            }
            let aba = format!("{c1}{c2}{c1}");
            let bab = format!("{c2}{c1}{c2}");

            let outside_ok = outsides.iter().any(|slice| slice.contains(&aba));
            let inside_ok = insides.iter().any(|slice| slice.contains(&bab));

            if outside_ok && inside_ok {
                return true;
            }
        }
    }

    return false;
}

fn main() {
    let sol = stdin()
        .lock()
        .lines()
        .map(|l| l.expect("Could not read line"))
        .filter(supports_tls)
        .map(|x| x)
        .count();
    println!("{sol}");
}
