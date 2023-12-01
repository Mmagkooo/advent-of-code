use std::io::stdin;

fn extract_digit_from_start(s: &str) -> Option<i32> {
    let first_char = s.chars().nth(0).unwrap();
    if first_char.is_numeric() {
        Some(first_char as i32 - '0' as i32)
    } else if s.starts_with("one") {
        Some(1)
    } else if s.starts_with("two") {
        Some(2)
    } else if s.starts_with("three") {
        Some(3)
    } else if s.starts_with("four") {
        Some(4)
    } else if s.starts_with("five") {
        Some(5)
    } else if s.starts_with("six") {
        Some(6)
    } else if s.starts_with("seven") {
        Some(7)
    } else if s.starts_with("eight") {
        Some(8)
    } else if s.starts_with("nine") {
        Some(9)
    } else {
        None
    }
}

fn main() {
    let sol: i32 = stdin()
        .lines()
        .map(|line| line.unwrap())
        .map(|line| {
            let line_length = line.len();

            let mut first_digit = None;
            for i in 0..line_length {
                if let Some(digit) = extract_digit_from_start(&line[i..]) {
                    first_digit = Some(digit);
                    break;
                }
            }

            let mut last_digit = None;
            for i in (0..line_length).rev() {
                if let Some(digit) = extract_digit_from_start(&line[i..]) {
                    last_digit = Some(digit);
                    break;
                }
            }

            first_digit.unwrap() * 10 + last_digit.unwrap()
        })
        .sum();

    println!("{sol}");
}
