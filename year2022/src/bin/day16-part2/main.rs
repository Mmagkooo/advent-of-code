use itertools::{self, Itertools};
use regex::Regex;
use std::{
    cmp::max,
    collections::HashMap,
    io::{stdin, BufRead},
    // i32::MAX,
};

#[derive(Debug)]
struct Valve {
    rate: i32,
    children: Vec<u8>,
}

fn contains(storage: u64, value: u8) -> bool {
    storage & (1 << value) != 0
}

fn store(storage: &mut u64, value: u8) {
    *storage |= 1 << value
}

type Configuration = (u8, u8, i32, u64);

fn find_max_pressure(
    valves: &HashMap<u8, Valve>,
    valve1_id: u8,
    valve2_id: u8,
    time_left: i32,
    open: u64,
    accumulated_rate: i32,
    memo: &mut HashMap<Configuration, i32>,
) -> i32 {
    // println!("name={name}, time_left={time_left}, acc_rate={accumulated_rate}");
    if time_left <= 0 {
        return 0;
    }

    // why 15? because that's the number of valves with flow rate > 0
    // an even better optimization would be to merge nodes
    // if open.count_ones() > 15 {
    //     return time_left * accumulated_rate;
    // }

    let configuration: Configuration = (valve1_id, valve2_id, time_left, open);
    if memo.contains_key(&configuration) {
        return memo[&configuration];
    }

    let valve1 = &valves[&valve1_id];
    let valve2 = &valves[&valve2_id];

    let mut max_pressure = 0;

    // 1. option: both try to open
    if (!contains(open, valve1_id) && valve1.rate > 0)
        && (!contains(open, valve2_id) && valve2.rate > 0)
        && (valve1_id != valve2_id)
        && time_left >= 1
    {
        let mut open = open;
        store(&mut open, valve1_id);
        store(&mut open, valve2_id);
        let candidate = find_max_pressure(
            valves,
            valve1_id,
            valve2_id,
            time_left - 1,
            open,
            accumulated_rate + valve1.rate + valve2.rate,
            memo,
        ) + accumulated_rate;
        max_pressure = max(max_pressure, candidate);
    }

    // 2. option: agent1 opens, agent2 moves
    if !contains(open, valve1_id) && valve1.rate > 0 && time_left >= 1 {
        let mut open = open;
        store(&mut open, valve1_id);
        for child_id in valve2.children.iter() {
            let candidate = find_max_pressure(
                valves,
                valve1_id,
                *child_id,
                time_left - 1,
                open,
                accumulated_rate + valve1.rate,
                memo,
            ) + accumulated_rate;
            max_pressure = max(max_pressure, candidate);
        }
    }

    // 3. option: agent1 moves, agent2 opens
    if !contains(open, valve2_id) && valve2.rate > 0 && time_left >= 1 {
        let mut open = open;
        store(&mut open, valve2_id);
        for child_id in valve1.children.iter() {
            let candidate = find_max_pressure(
                valves,
                *child_id,
                valve2_id,
                time_left - 1,
                open,
                accumulated_rate + valve2.rate,
                memo,
            ) + accumulated_rate;
            max_pressure = max(max_pressure, candidate);
        }
    }

    // 4. option: both move
    for (child1_id, child2_id) in valve1
        .children
        .iter()
        .cartesian_product(valve2.children.iter())
    {
        let candidate = find_max_pressure(
            valves,
            *child1_id,
            *child2_id,
            time_left - 1,
            open,
            accumulated_rate,
            memo,
        ) + accumulated_rate;
        max_pressure = max(max_pressure, candidate);
    }

    memo.insert(configuration, max_pressure);
    return max_pressure;
}

/**
 * First used name will get id=0, etc.
 */
fn valve_name_to_id(name: &str, memo: &mut HashMap<String, u8>) -> u8 {
    match memo.get(name) {
        Some(id) => return *id,
        None => {
            let new_id = memo.len() as u8;
            memo.insert(name.to_string(), new_id);
            return new_id;
        }
    }
}

fn main() {
    let re =
        Regex::new("^Valve (.+) has flow rate=(.+); tunnel(?:s?) lead(?:s?) to valve(?:s?) (.+)$")
            .unwrap();

    // replace string name with u8 id for better performance
    let mut valve_name_to_id_memo: HashMap<String, u8> = HashMap::new();

    let valves: HashMap<u8, Valve> = stdin()
        .lock()
        .lines()
        .map(|line| line.expect("Could not read line"))
        .map(|line| {
            re.captures(line.as_str()).map(|caps| {
                // pairs of (name, valve) to construct a map
                let valve_name =
                    valve_name_to_id(caps.get(1).unwrap().as_str(), &mut valve_name_to_id_memo);
                (
                    valve_name,
                    Valve {
                        rate: caps.get(2).unwrap().as_str().parse().unwrap(),
                        children: caps
                            .get(3)
                            .unwrap()
                            .as_str()
                            .split(", ")
                            .map(|name| valve_name_to_id(name, &mut valve_name_to_id_memo))
                            .collect(),
                    },
                )
            })
        })
        .map(|name_and_valve| name_and_valve.unwrap())
        .collect();

    let start_id = valve_name_to_id("AA", &mut valve_name_to_id_memo);
    let time_left = 26;
    let open = 0u64;
    let accumulated_rate = 0;
    let mut memo: HashMap<Configuration, i32> = HashMap::new();
    let sol = find_max_pressure(
        &valves,
        start_id,
        start_id,
        time_left,
        open,
        accumulated_rate,
        &mut memo,
    );
    println!("{sol}");
}
