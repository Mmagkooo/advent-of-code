use std::io::{stdin, BufRead};

fn is_abba(s: &str) -> bool {
    let chars: Vec<char> = s.chars().collect();
    return chars.len() == 4
        && chars[0] == chars[3]
        && chars[1] == chars[2]
        && chars[0] != chars[1];
}

fn contains_abba(line: &str) -> bool {
    let line_length = line.len();
    if line_length < 4 {
        return false;
    }

    for i in 0..line_length - 4 + 1 {
        if is_abba(&line[i..i + 4]) {
            return true;
        }
    }
    return false;
}

fn supports_tls(line: &String) -> bool {
    let slices: Vec<&str> = line.split(|c| "[]".contains(c)).collect();
    assert!(&line[0..1] != "["); // assert starting with a non-bracketed substring

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

    let outsides_ok = outsides.iter().map(|s| *s).any(contains_abba);
    let insides_ok = !insides.iter().map(|s| *s).any(contains_abba);
    return outsides_ok && insides_ok;
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
