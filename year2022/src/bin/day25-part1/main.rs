use std::io::{stdin, BufRead};

fn parse_snafu(raw: String) -> i64 {
    let mut parsed = 0;
    for c in raw.chars() {
        let digit = match c {
            '=' => -2,
            '-' => -1,
            '0' => 0,
            '1' => 1,
            '2' => 2,
            invalid => panic!("Invalid char: {invalid}"),
        };
        parsed = parsed * 5 + digit;
    }
    return parsed;
}

fn to_snafu(mut dec: i64) -> String {
    let mut digits: Vec<i8> = vec![];
    let mut carry: i8 = 0;
    while dec > 0 {
        let digit = (dec % 5) as i8 + carry;
        if digit <= 2 {
            digits.push(digit);
            carry = 0;
        } else {
            digits.push(digit - 5);
            carry = 1;
        }
        dec /= 5;
    }

    if carry > 0 {
        digits.push(carry);
    }

    digits.reverse();

    if digits.is_empty() {
        digits.push(0);
    }

    return digits
        .iter()
        .map(|d| match d {
            -1 => '-',
            -2 => '=',
            // convert i32 to corresponding char, only u8 can be converted?
            other => ((*other as i32 + '0' as i32) as u8) as char
        })
        .collect();
}

fn main() {
    let sol: i64 = stdin()
        .lock()
        .lines()
        .map(|line| line.expect("Cannot read line"))
        .map(parse_snafu)
        .sum();
    println!("{}", to_snafu(sol));
}
