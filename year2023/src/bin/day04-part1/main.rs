use std::collections::HashSet;

fn raw_numbers_to_set(raw_numbers: &str) -> HashSet<u32> {
    HashSet::from_iter(raw_numbers.split_whitespace().map(|n| n.parse().unwrap()))
}

fn main() {
    let sol: u32 = std::io::stdin()
        .lines()
        .map(|line| line.unwrap())
        .map(|line| line.split(": ").nth(1).unwrap().to_string())
        .map(|numbers_raw| {
            numbers_raw
                .split(" | ")
                .map(|s| s.to_string())
                .collect::<Vec<String>>()
        })
        .map(|split_content| {
            assert_eq!(split_content.len(), 2);
            (split_content[0].clone(), split_content[1].clone())
        })
        .map(|(winning_numbers_raw, my_numbers_raw)| {
            let winning_set = raw_numbers_to_set(&winning_numbers_raw);
            let my_set = raw_numbers_to_set(&my_numbers_raw);
            let intersection = winning_set.intersection(&my_set);
            intersection.count()
        })
        .filter(|intersection_size| *intersection_size > 0)
        .map(|intersection_size| 1 << (intersection_size - 1))
        .sum();
    println!("{sol}");
}
