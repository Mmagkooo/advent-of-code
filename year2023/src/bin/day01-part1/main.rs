use std::io::stdin;

fn main() {
    let sol: u32 = stdin()
        .lines()
        .map(|line| line.unwrap())
        .map(|line| {
            let mut num: u32 = 0;
            for c in line.chars() {
                if c.is_numeric() {
                    num += 10 * ((c as u32) - ('0' as u32));
                    break;
                }
            }

            for c in line.chars().rev() {
                if c.is_numeric() {
                    num += (c as u32) - ('0' as u32);
                    break;
                }
            }
            num
        })
        .sum();

    println!("{sol}");
}
