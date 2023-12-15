fn hash_algorithm(word: &str) -> u32 {
    word.chars().fold(0, |acc, c| ((acc + c as u32) * 17) % 256)
}

fn main() {
    let line = std::io::stdin().lines().next().unwrap().unwrap();
    let sol: u32 = line.split(",").map(hash_algorithm).sum();
    println!("{sol}");
}
