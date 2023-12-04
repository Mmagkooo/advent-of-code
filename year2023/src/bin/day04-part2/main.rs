use std::collections::{HashMap, HashSet};

fn raw_numbers_to_set(raw_numbers: &str) -> HashSet<u32> {
    HashSet::from_iter(raw_numbers.split_whitespace().map(|n| n.parse().unwrap()))
}

fn main() {
    let mut card_id_to_scratchcards: HashMap<u32, usize> = HashMap::new();
    card_id_to_scratchcards.insert(1, 1);
    std::io::stdin()
        .lines()
        .map(|line| line.unwrap())
        .map(|line| {
            line.split(": ")
                .map(|s| s.to_string())
                .collect::<Vec<String>>()
        })
        .map(|split_content| {
            assert_eq!(split_content.len(), 2);
            (
                split_content[0]
                    .strip_prefix("Card")
                    .unwrap()
                    .trim()
                    .parse::<u32>()
                    .unwrap(),
                split_content[1].clone(),
            )
        })
        .map(|(card_id, numbers_raw)| {
            (
                card_id,
                numbers_raw
                    .split(" | ")
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>(),
            )
        })
        .map(|(card_id, split_content)| {
            assert_eq!(split_content.len(), 2);
            (
                card_id,
                split_content[0].clone(),
                split_content[1].clone(),
            )
        })
        .map(|(card_id, winning_numbers_raw, my_numbers_raw)| {
            let winning_set = raw_numbers_to_set(&winning_numbers_raw);
            let my_set = raw_numbers_to_set(&my_numbers_raw);
            let intersection = winning_set.intersection(&my_set);
            (card_id, intersection.count())
        })
        .for_each(|(card_id, intersections)| {
            let multiplier = *card_id_to_scratchcards.entry(card_id).or_insert(1);
            for future_card_id in (card_id + 1)..(card_id + intersections as u32 + 1) {
                *card_id_to_scratchcards.entry(future_card_id).or_insert(1) += multiplier;
            }
        });

    let sol: usize = card_id_to_scratchcards.values().sum();
    println!("{sol}");
}
