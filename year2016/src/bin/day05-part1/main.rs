use std::io::stdin;

const PASSWORD_LEN: usize = 8;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).expect("No input");

    let mut password = String::new();
    for i in 0.. {
        let mut digestable = input.clone();
        digestable.push_str(i.to_string().as_str());
        let digest = format!("{:x}", md5::compute(digestable));

        if digest.starts_with("00000") {
            let next_digit = digest.chars().nth(5).unwrap();
            password.push(next_digit);
            println!("Found: {}", password);
            if password.len() == PASSWORD_LEN {
                break;
            }
        }
    }

    println!("{password}");
}
