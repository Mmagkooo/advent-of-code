use std::collections::HashMap;

#[derive(Ord, PartialOrd, PartialEq, Eq, Debug, Hash, Copy, Clone)]
enum Card {
    N2,
    N3,
    N4,
    N5,
    N6,
    N7,
    N8,
    N9,
    T,
    J,
    Q,
    K,
    A,
}

impl Card {
    fn from_char(s: char) -> Self {
        match s {
            '2' => Self::N2,
            '3' => Self::N3,
            '4' => Self::N4,
            '5' => Self::N5,
            '6' => Self::N6,
            '7' => Self::N7,
            '8' => Self::N8,
            '9' => Self::N9,
            'T' => Self::T,
            'J' => Self::J,
            'Q' => Self::Q,
            'K' => Self::K,
            'A' => Self::A,
            other => panic!("Cannot parse as Card: {other}"),
        }
    }
}

#[derive(Eq, Debug)]
struct Hand {
    cards: [Card; 5],
    bid: u32,
    strength: Strength,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
enum Strength {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl Hand {
    fn new(cards: [Card; 5], bid: u32) -> Self {
        let mut map = HashMap::new();
        for card in cards.iter() {
            *map.entry(*card).or_insert(0) += 1;
        }
        Self {
            cards,
            bid,
            strength: Self::strength(map),
        }
    }

    fn strength(map: HashMap<Card, i32>) -> Strength {
        let values = map.values().collect::<Vec<_>>();
        match map.len() {
            1 => Strength::FiveOfAKind,
            2 => match values.contains(&&4) {
                true => Strength::FourOfAKind,
                false => Strength::FullHouse,
            },
            3 => match values.contains(&&3) {
                true => Strength::ThreeOfAKind,
                false => Strength::TwoPair,
            },
            4 => Strength::OnePair,
            5 => Strength::HighCard,
            invalid => panic!("Invalid map length: {invalid}"),
        }
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.strength.cmp(&other.strength) {
            std::cmp::Ordering::Equal => self.cards.cmp(&other.cards),
            other => other,
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let mut hands: Vec<Hand> = std::io::stdin()
        .lines()
        .map(|l| l.unwrap())
        .map(|l| l.split(" ").map(|s| s.to_string()).collect::<Vec<String>>())
        .map(|split_content| {
            assert_eq!(split_content.len(), 2);
            Hand::new(
                split_content[0]
                    .chars()
                    .map(Card::from_char)
                    .collect::<Vec<Card>>()
                    .try_into()
                    .unwrap(),
                split_content[1].parse().unwrap(),
            )
        })
        .collect();

    hands.sort();
    let sol: u32 = hands
        .into_iter()
        .enumerate()
        .map(|(i, hand)| (i as u32 + 1) * hand.bid)
        .sum();
    println!("{sol}");
}
