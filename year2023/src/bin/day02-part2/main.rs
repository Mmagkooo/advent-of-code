use std::{collections::HashMap, io::stdin};

/// input is of format "(<N> <COLOR>(,|;))+"
fn extract_draws(game_str: String) -> Vec<String> {
    // trimming because can't split on ", " and "; " without regex
    game_str
        .split([',', ';'])
        .map(|s| s.trim().to_string())
        .collect()
}

/// not color-dependent
fn extract_max_amounts(draws: Vec<String>) -> u32 {
    let mut max_amounts: HashMap<&str, u32> = HashMap::new();
    draws
        .iter()
        .map(|draw| draw.split(" "))
        .map(|mut split_content| {
            (
                split_content.next().unwrap().parse::<u32>().unwrap(),
                split_content.next().unwrap(),
            )
        })
        .for_each(|(amount, color)| {
            let new_max = match max_amounts.get(color) {
                Some(max_so_far) if amount > *max_so_far => amount,
                Some(max_so_far) => *max_so_far,
                None => amount,
            };
            max_amounts.insert(color, new_max);
        });

    max_amounts
        .into_values()
        .reduce(|so_far, current| so_far * current)
        .unwrap()
}

fn main() {
    let sol: u32 = stdin()
        .lines()
        .map(|line| line.unwrap())
        .map(|line| line.split(": ").nth(1).unwrap().to_string())
        .map(extract_draws)
        .map(extract_max_amounts)
        .sum();
    println!("{sol}");
}
