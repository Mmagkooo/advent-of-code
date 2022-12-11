use std::{
    collections::VecDeque,
    io::{self, BufRead},
};

#[derive(Clone, Debug)]
enum Operator {
    ADD,
    MUL,
    SQUARE,
}

#[derive(Clone, Debug)]
struct Monkey {
    items: VecDeque<i32>,
    operator: Operator,
    operation_arg: Option<i32>,
    testing_modulo: i32,
    success_target: usize,
    failure_target: usize,
    times_inspected: i32,
}

fn get_last_word<T: std::str::FromStr>(line: &str) -> Result<T, T::Err> {
    line.split(" ").last().unwrap().parse()
}

fn main() {
    let lines: Vec<String> = io::stdin()
        .lock()
        .lines()
        .map(|line| line.expect("Cannot read line"))
        .collect();

    let mut monkeys: Vec<Monkey> = vec![];
    let mut i = 0; // line index
    while i < lines.len() {
        let items: VecDeque<i32> = lines[i + 1]
            .trim()
            .strip_prefix("Starting items: ")
            .unwrap()
            .split(", ")
            .map(|item| item.parse().unwrap())
            .collect();

        let (operator, operation_arg) = if lines[i + 2].ends_with("* old") {
            (Operator::SQUARE, None)
        } else {
            let operation_arg: Option<i32> = Some(get_last_word(&lines[i + 2]).unwrap());
            if lines[i + 2].contains("+") {
                (Operator::ADD, operation_arg)
            } else {
                (Operator::MUL, operation_arg)
            }
        };

        let testing_modulo: i32 = get_last_word(&lines[i + 3]).unwrap();

        let success_target: usize = get_last_word(&lines[i + 4]).unwrap();
        let failure_target: usize = get_last_word(&lines[i + 5]).unwrap();

        monkeys.push(Monkey {
            items,
            operator,
            operation_arg,
            testing_modulo,
            success_target,
            failure_target,
            times_inspected: 0,
        });

        i += 7; // on to the next monkey
    }

    let number_of_rounds = 20;
    for _ in 0..number_of_rounds {
        for i in 0..monkeys.len() {
            // work on a copy of monkey, later assign this copy
            let mut monkey = monkeys.get_mut(i).unwrap().clone();

            let number_of_items: i32 = monkey.items.len() as i32;
            for _ in 0..number_of_items {
                // inspect item
                let mut worry_level = monkey.items.pop_front().unwrap();
                worry_level = match monkey.operator {
                    Operator::ADD => worry_level + monkey.operation_arg.unwrap(),
                    Operator::MUL => worry_level * monkey.operation_arg.unwrap(),
                    Operator::SQUARE => worry_level * worry_level,
                };

                worry_level /= 3;

                let target_monkey_index = if worry_level % monkey.testing_modulo == 0 {
                    monkey.success_target
                } else {
                    monkey.failure_target
                };
                monkeys[target_monkey_index].items.push_back(worry_level);
            }

            assert!(monkey.items.is_empty());
            monkey.times_inspected += number_of_items;
            monkeys[i] = monkey;
        }
    }

    monkeys.sort_by(|m1, m2| m1.times_inspected.cmp(&m2.times_inspected));
    println!("{monkeys:?}");
    println!("{}", monkeys[monkeys.len()-1].times_inspected * monkeys[monkeys.len()-2].times_inspected);
}
