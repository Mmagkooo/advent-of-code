use std::io;

type Range = (i32, i32);

fn get_range(raw: &str) -> Range {
    let pair: Vec<i32> = raw.split("-").map(|num_str| num_str.parse().unwrap()).collect();
    assert_eq!(pair.len(), 2);
    (pair[0], pair[1])
}

fn is_within(range1: &Range, range2: &Range) -> bool {
    range1.0 >= range2.0 && range1.1 <= range2.1
}

fn main() {
    let mut sol = 0;
    loop {
        let mut input = String::new();
        if io::stdin().read_line(&mut input).unwrap() != 0 {
            let pairs: Vec<&str> = input.trim().split(",").collect();
            let (range1, range2) = (get_range(pairs[0]), get_range(pairs[1]));
            if is_within(&range1, &range2) || is_within(&range2, &range1) {
                sol += 1;
            }
        } else {
            break;
        }
    }

    println!("{sol}");
}