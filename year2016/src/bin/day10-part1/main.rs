use std::{
    collections::{HashMap, VecDeque},
    io::{stdin, BufRead},
};

#[derive(Debug)]
enum Target {
    Bot(i32),
    Output(i32),
}

type Rule = (Target, Target);

fn extract_target(kind: &str, raw_num: &str) -> Target {
    let num = raw_num.parse().unwrap();
    match kind {
        "bot" => Target::Bot(num),
        "output" => Target::Output(num),
        invalid => panic!("Invalid target: {invalid}"),
    }
}

fn get_bot_with_multiple_values(bots: &HashMap<i32, Vec<i32>>) -> (&i32, &Vec<i32>) {
    for (bot_num, values) in bots.iter() {
        if values.len() > 1 {
            return (bot_num, values);
        }
    }

    panic!("No bot with multiple values");
}

fn relocate(
    target: &Target,
    value: i32,
    bots: &mut HashMap<i32, Vec<i32>>,
    outputs: &mut HashMap<i32, Vec<i32>>,
    bot_queue: &mut VecDeque<(i32, Vec<i32>)>,
) {
    match target {
        Target::Bot(bot_num) => {
            let target_values = bots.entry(*bot_num).or_insert(vec![]);
            target_values.push(value);
            assert!(target_values.len() <= 2);
            if target_values.len() == 2 {
                bot_queue.push_front((*bot_num, target_values.clone()));
            }
        }
        Target::Output(output_num) => {
            outputs.entry(*output_num).or_insert(vec![]).push(value);
        }
    }
}

fn main() {
    // could use something lighter than vector
    let mut bots = HashMap::<i32, Vec<i32>>::new();
    let mut outputs = HashMap::<i32, Vec<i32>>::new();
    let mut rules = HashMap::<i32, Rule>::new();

    let lines: Vec<String> = stdin()
        .lock()
        .lines()
        .map(|l| l.expect("Cannot read line"))
        .collect();

    // initial population of bots + rules extraction
    for line in lines {
        let words: Vec<&str> = line.split_whitespace().collect();
        match words[0] {
            "value" => {
                let value = words[1].parse().unwrap();
                let target_bot = words.last().unwrap().parse().unwrap();
                bots.entry(target_bot).or_insert(vec![]).push(value);
            }
            "bot" => {
                let source: i32 = words[1].parse().unwrap();
                let low_target = extract_target(words[5], words[6]);
                let high_target = extract_target(words[10], words[11]);
                rules.insert(source, (low_target, high_target));
            }
            invalid => panic!("Invalid line start: {invalid}"),
        }
    }

    let (start_bot, start_values) = get_bot_with_multiple_values(&bots);
    let mut bot_queue = VecDeque::<(i32, Vec<i32>)>::from([(*start_bot, start_values.clone())]);
    while !bot_queue.is_empty() {
        let (source_bot, values) = bot_queue.pop_back().unwrap();

        assert_eq!(values.len(), 2);

        let low_value = *values.iter().min().unwrap();
        let high_value = *values.iter().max().unwrap();

        if low_value == 17 && high_value == 61 {
            println!("{source_bot}");
            break;
        }

        let (low_target, high_target) = rules.get(&source_bot).unwrap();
        relocate(
            low_target,
            low_value,
            &mut bots,
            &mut outputs,
            &mut bot_queue,
        );
        relocate(
            high_target,
            high_value,
            &mut bots,
            &mut outputs,
            &mut bot_queue,
        );
    }
}
