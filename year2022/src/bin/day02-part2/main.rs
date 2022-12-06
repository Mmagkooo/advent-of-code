use std::io::{self, BufRead};

#[derive(Debug, PartialEq)]
enum Choice {
    ROCK,
    PAPER,
    SCISSORS,
}

#[derive(PartialEq)]
enum Outcome {
    WIN,
    DRAW,
    LOSS,
}

fn get_value(choice: Choice) -> i32 {
    if choice == Choice::ROCK {
        return 1;
    } else if choice == Choice::PAPER {
        return 2;
    } else if choice == Choice::SCISSORS {
        return 3;
    }
    panic!("Invalid choice {:?}", choice);
}

fn parse_choice(input: &str) -> Choice {
    assert_eq!(input.len(), 1);
    if "A" == input {
        return Choice::ROCK;
    } else if "B" == input {
        return Choice::PAPER;
    } else if "C" == input {
        return Choice::SCISSORS;
    }
    panic!("Invalid input {}", input);
}

fn parse_outcome(input: &str) -> Outcome {
    assert_eq!(input.len(), 1);
    if "X" == input {
        return Outcome::LOSS;
    } else if "Y" == input {
        return Outcome::DRAW;
    } else if "Z" == input {
        return Outcome::WIN;
    }
    panic!("Invalid outcome {}", input);
}

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();

    let mut score = 0;

    for line in lines {
        let unwrapped_line = line.unwrap();
        let pair: Vec<&str> = unwrapped_line.trim().split(" ").collect();
        let opponent_choice = parse_choice(pair[0]);
        let expected_outcome = parse_outcome(pair[1]);

        if expected_outcome == Outcome::DRAW {
            score += 3;
            score += get_value(opponent_choice);
        } else if expected_outcome == Outcome::WIN {
            score += 6;
            score += get_value(if opponent_choice == Choice::ROCK {
                Choice::PAPER
            } else if opponent_choice == Choice::PAPER {
                Choice::SCISSORS
            } else {
                Choice::ROCK
            });
        } else if expected_outcome == Outcome::LOSS {
            score += 0;
            score += get_value(if opponent_choice == Choice::ROCK {
                Choice::SCISSORS
            } else if opponent_choice == Choice::PAPER {
                Choice::ROCK
            } else {
                Choice::PAPER
            })
        }
    }

    println!("{}", score);
}
