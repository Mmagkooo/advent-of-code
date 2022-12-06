use std::io::{self, BufRead};

#[derive(PartialEq)]
enum Choice {
    ROCK,
    PAPER,
    SCISSORS
}

fn parse_input(input: &str) -> ChoiceWrapper {
    assert_eq!(input.len(), 1);
    if "XA".contains(input) {
        return ChoiceWrapper{ choice: Choice::ROCK, value: 1 };
    } else if "YB".contains(input) {
        return ChoiceWrapper{ choice: Choice::PAPER, value: 2 };
    } else if "ZC".contains(input) {
        return ChoiceWrapper{ choice: Choice::SCISSORS, value: 3 };
    }
    panic!("Invalid input {}", input);
}

struct ChoiceWrapper {
    choice: Choice,
    value: i32
}

const LOSS: i32 = 0;
const DRAW: i32 = 3;
const WIN: i32 = 6;

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();

    let mut score = 0;

    for line in lines {
        let unwrapped_line = line.unwrap();
        let pair: Vec<&str> = unwrapped_line.trim().split(" ").collect();
        let opponent = parse_input(pair[0]);
        let me = parse_input(pair[1]);
        
        score += me.value; // definitely getting score for whatever I played
        
        if me.choice == opponent.choice {
            score += DRAW;
        } else if me.choice == Choice::ROCK {
            score += if opponent.choice == Choice::SCISSORS { WIN } else { LOSS };
        } else if me.choice == Choice::SCISSORS {
            score += if opponent.choice == Choice::PAPER { WIN } else { LOSS };
        } else if me.choice == Choice::PAPER {
            score += if opponent.choice == Choice::ROCK { WIN } else { LOSS };
        }
    }

    println!("{}", score);
}
