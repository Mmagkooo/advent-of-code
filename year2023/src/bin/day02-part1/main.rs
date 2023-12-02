use std::io::stdin;

/// input is of format "<N> <COLOR>"
fn drawn_ok(s: &str) -> bool {
    let parts: Vec<&str> = s.split(" ").collect();
    assert_eq!(parts.len(), 2);
    let amount: u32 = parts[0].parse().unwrap();
    let limit = match parts[1] {
        "red" => 12,
        "green" => 13,
        "blue" => 14,
        other => panic!("Invalid color: {other}"),
    };
    return amount <= limit;
}

/// input is of format "(<N> <COLOR>(,|;))+"
fn extract_draws(game_str: &str) -> Vec<&str> {
    // trimming because can't split on ", " and "; " without regex
    game_str.split([',', ';']).map(|s| s.trim()).collect()
}

/// input is of format "Game <N>"
/// output is the game index N
fn extract_game_i(s: &str) -> u32 {
    s.split(" ").nth(1).unwrap().parse().unwrap()
}

fn main() {
    let sol: u32 = stdin()
        .lines()
        .map(|line| line.unwrap())
        .map(|line| line.split(": ").map(|s| s.to_string()).collect())
        .map(|split_content: Vec<String>| {
            assert_eq!(split_content.len(), 2);
            (split_content[0].clone(), split_content[1].clone())
        })
        .filter_map(|(game_i_str, game_str)| {
            match extract_draws(&game_str).into_iter().all(drawn_ok) {
                true => Some(extract_game_i(&game_i_str)),
                false => None,
            }
        })
        .sum();
    println!("{sol}");
}
