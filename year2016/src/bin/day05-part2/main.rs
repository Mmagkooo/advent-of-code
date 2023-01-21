use std::io::{stdin, stdout, Write};

const PASSWORD_LEN: usize = 8;
type Password = [char; PASSWORD_LEN];
const EMPTY_CHAR: char = '\u{0}';

/**
 * Overwrites previous content
 */
fn print_password(password: &Password) {
    let printable: String = password
        .iter()
        .map(|c| match c {
            &EMPTY_CHAR => '_',
            valid_char => *valid_char,
        })
        .collect();

    print!("\r"); // delete previous stdout content
    print!("{printable}");
    stdout().flush().expect("Could not flush");
}

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).expect("No input");
    let mut password = [EMPTY_CHAR; PASSWORD_LEN];
    let mut found = 0;
    for i in 0.. {
        let mut digestable = input.clone();
        digestable.push_str(i.to_string().as_str());
        let digest = format!("{:x}", md5::compute(digestable));

        if digest.starts_with("00000") {
            let position = digest.chars().nth(5).unwrap().to_digit(10);
            let position: usize = match position {
                None => continue,
                Some(val) if val >= 8 => continue,
                Some(val) => val as usize,
            };

            if password[position] != EMPTY_CHAR {
                continue;
            }

            let next_digit = digest.chars().nth(6).unwrap();
            password[position] = next_digit;
            print_password(&password);
            found += 1;
            if found == PASSWORD_LEN {
                break;
            }
        }
    }
    println!();
}
