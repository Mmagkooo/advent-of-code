use std::{
    collections::HashMap,
    io::{stdin, BufRead},
    sync::mpsc,
    thread,
};

use regex::Regex;

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
struct Factory {
    // goods
    ore: i32,
    clay: i32,
    obsidian: i32,

    // robots
    ore_robots: i32,
    clay_robots: i32,
    obsidian_robots: i32,
    geode_robots: i32,
}

impl Factory {
    fn new() -> Self {
        Self {
            // goods
            ore: 0,
            clay: 0,
            obsidian: 0,

            // robots
            ore_robots: 1,
            clay_robots: 0,
            obsidian_robots: 0,
            geode_robots: 0,
        }
    }

    fn update_material(&mut self, factory: &Factory) {
        self.ore += factory.ore_robots;
        self.clay += factory.clay_robots;
        self.obsidian += factory.obsidian_robots;
    }
}

struct Blueprint {
    ore_robot_price: i32,
    clay_robot_price: i32,
    obsidian_robot_price: (i32, i32), // ore and clay
    geode_robot_price: (i32, i32),    // ore and obsidian
}

#[derive(Hash, PartialEq, Eq)]
struct MemoConfig {
    factory: Factory,
    time: i32,
}

fn get_max_geode(
    factory: &Factory,
    blueprint: &Blueprint,
    time: i32,
    memo: &mut HashMap<MemoConfig, i32>,
) -> i32 {
    if time == 0 {
        return 0;
    }

    let memo_config = MemoConfig {
        factory: factory.clone(),
        time: time.clone(),
    };
    match memo.get(&memo_config) {
        Some(old_value) => return *old_value,
        _ => (),
    }

    // remember now, add in the end, robots get updated in the meantime
    let latest_generated_geode = factory.geode_robots;
    let mut max_geode = 0;

    let mut update_best = |candidate_factory: &Factory| {
        let candidate_geode = get_max_geode(candidate_factory, blueprint, time - 1, memo);
        if candidate_geode > max_geode {
            max_geode = candidate_geode;
        }
    };

    // build robots

    // geode robot
    if factory.ore >= blueprint.geode_robot_price.0
        && factory.obsidian >= blueprint.geode_robot_price.1
    {
        let mut new_factory = factory.clone();

        new_factory.ore -= blueprint.geode_robot_price.0;
        new_factory.obsidian -= blueprint.geode_robot_price.1;
        new_factory.geode_robots += 1;

        new_factory.update_material(&factory);
        update_best(&new_factory);
    }

    // obsidian robot
    if factory.ore >= blueprint.obsidian_robot_price.0
        && factory.clay >= blueprint.obsidian_robot_price.1
    {
        let mut new_factory = factory.clone();

        new_factory.ore -= blueprint.obsidian_robot_price.0;
        new_factory.clay -= blueprint.obsidian_robot_price.1;
        new_factory.obsidian_robots += 1;

        new_factory.update_material(&factory);
        update_best(&new_factory);
    }

    // clay robot
    if factory.ore >= blueprint.clay_robot_price {
        let mut new_factory = factory.clone();

        new_factory.ore -= blueprint.clay_robot_price;
        new_factory.clay_robots += 1;

        new_factory.update_material(&factory);
        update_best(&new_factory);
    }

    // ore robot
    if factory.ore >= blueprint.ore_robot_price {
        let mut new_factory = factory.clone();

        new_factory.ore -= blueprint.ore_robot_price;
        new_factory.ore_robots += 1;

        new_factory.update_material(&factory);
        update_best(&new_factory);
    }

    // no robot
    {
        let mut new_factory = factory.clone();

        new_factory.update_material(&factory);
        update_best(&new_factory);
    }

    memo.insert(memo_config, max_geode);
    return max_geode + latest_generated_geode;
}

fn main() {
    let re = Regex::new(r"(\d+)").unwrap();
    let blueprints: Vec<Blueprint> = stdin()
        .lock()
        .lines()
        .map(|line| line.expect("Could not read line"))
        .map(|line| {
            let caps: Vec<i32> = re
                .captures_iter(&line)
                .map(|el| el.get(0).unwrap().as_str().parse().unwrap())
                .collect();

            Blueprint {
                ore_robot_price: caps[1],
                clay_robot_price: caps[2],
                obsidian_robot_price: (caps[3], caps[4]),
                geode_robot_price: (caps[5], caps[6]),
            }
        })
        .collect();

    let time = 24;

    let (tx, rx) = mpsc::channel();

    for (i, blueprint) in blueprints.into_iter().enumerate() {
        let factory = Factory::new();
        let tx_i = tx.clone();
        thread::spawn(move || {
            let mut memo = HashMap::new();
            let geode = get_max_geode(&factory, &blueprint, time, &mut memo);
            println!("id: {}, geode: {}", i + 1, geode);
            let quality_level = (i as i32 + 1) * geode;
            tx_i.send(quality_level).unwrap();
        });
    }

    // All senders (the original and the clones) need to be dropped for
    // the receiver to stop blocking to receive messages with Receiver::recv
    drop(tx);

    let mut sol = 0;
    for quality_level in rx {
        sol += quality_level;
    }

    println!("{sol}");
}
