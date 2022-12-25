use std::{
    collections::VecDeque,
    io::{stdin, BufRead},
};

#[derive(Clone, Debug, Eq, PartialEq)]
struct ValueWrapper {
    id: usize,
    value: i64,
}

fn move_element(deque: &mut VecDeque<ValueWrapper>, from: usize, to: usize) {
    if from < to {
        for i in from..to {
            deque.swap(i, i + 1);
        }
    } else {
        for i in (to + 1..=from).rev() {
            deque.swap(i, i - 1);
        }
    }
}

const DECRIPTION_KEY: i64 = 811589153;

fn main() {
    let mut deque: VecDeque<ValueWrapper> = stdin()
        .lock()
        .lines()
        .map(|line| line.expect("Could not read line"))
        .enumerate()
        .map(|(id, line)| ValueWrapper {
            id,
            value: line.parse::<i64>().unwrap() * DECRIPTION_KEY,
        })
        .collect();

    let modulo: i64 = deque.len() as i64 - 1; // -1 because it's a circle, so one less position is visited when moving
    let order = deque.clone();

    for _ in 0..10 {
        for value_wrapper in order.iter() {
            let original_index = deque.iter().position(|vw| *vw == *value_wrapper).unwrap();
            let new_index =
                ((original_index as i64 + value_wrapper.value) % modulo + modulo) % modulo;

            move_element(&mut deque, original_index, new_index as usize);
        }
    }

    let zero_index = deque.iter().position(|vw| vw.value == 0).unwrap();
    let mut sol = 0;
    for offset in [1000, 2000, 3000] {
        let val = deque[(zero_index + offset) % deque.len()].value;
        println!("Adding {val}");
        sol += val
    }
    println!("{sol}");
}
