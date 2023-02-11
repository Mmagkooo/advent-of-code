use std::{
    collections::{BinaryHeap, HashMap, HashSet},
    hash::Hash,
    io::{stdin, BufRead},
};

use itertools::Itertools;

use regex::Regex;

const FLOORS_NUM: usize = 4;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum RTG {
    Microchip(String),
    Generator(String),
}

/**
 * A wrapper so that hashing can be implemented more easily.
 */
#[derive(PartialEq, Eq, Clone)]
struct Floor {
    container: HashSet<RTG>,
}

#[derive(PartialEq, Eq)]
struct Config {
    floors: Vec<Floor>,
    elevator: usize,
    steps: Score,
    total_distance: Score,
}

impl Hash for Floor {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        let vectorized = self.container.iter().sorted().collect::<Vec<_>>();
        vectorized.hash(state);
    }
}

impl Ord for Config {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        return self.total_distance.cmp(&other.total_distance).reverse();
    }
}

impl PartialOrd for Config {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn is_floor_legal(floor: &Floor) -> bool {
    let any_generators = floor.container.iter().any(|rtg| match rtg {
        RTG::Generator(_) => true,
        RTG::Microchip(_) => false,
    });

    if any_generators {
        for rtg in floor.container.iter() {
            match rtg {
                // if microchip on floor, its generator should be
                RTG::Microchip(element) => {
                    let counterpart = RTG::Generator(element.clone());
                    if !floor.container.contains(&counterpart) {
                        return false;
                    }
                }
                RTG::Generator(_) => (),
            }
        }
    }

    return true;
}

fn is_over(floors: &Vec<Floor>) -> bool {
    for floor_i in 0..(floors.len() - 1) {
        if !floors[floor_i].container.is_empty() {
            return false;
        }
    }
    return true; // asumes everything on top floor
}

type Score = u32;

fn heuristic(floors: &Vec<Floor>) -> u32 {
    let mut ret = 0;
    let total_floors = floors.len();
    for (i, floor) in floors.iter().enumerate() {
        let floor_distance = total_floors - i;
        ret += floor_distance * floor.container.len();
    }
    return ret as u32;
}

/**
 * A* algorithm with memoization. Using the `heuristic` defined above.
 */
fn search(floors: Vec<Floor>, elevator: usize, steps: Score) -> u32 {
    let mut queue = BinaryHeap::new();
    let total_distance = steps + heuristic(&floors);
    queue.push(Config {
        floors,
        elevator,
        steps,
        total_distance,
    });

    let mut memo = HashMap::<(Vec<Floor>, usize), Score>::new();

    while !queue.is_empty() {
        let config = queue.pop().unwrap();
        let Config {
            floors,
            elevator,
            steps,
            total_distance: _,
        } = config;
        if is_over(&floors) {
            return steps;
        }

        let memoizable_config = (floors.clone(), elevator);
        match memo.get(&memoizable_config) {
            Some(old_steps) => {
                if *old_steps <= steps {
                    continue;
                }
            }
            None => (),
        }

        memo.insert(memoizable_config, steps);

        // elevator can go down or up
        for position_change in [-1, 1] {
            if position_change == -1 && elevator == 0 {
                // can't go down from bottom floor
                continue;
            }
            if position_change == 1 && elevator == FLOORS_NUM - 1 {
                // can't go up from top floor
                continue;
            }
            let next_position =
                usize::try_from(i32::try_from(elevator).unwrap() + position_change).unwrap();

            // elevator works with at least 1 element - cannot move 0

            // move 1 element
            for rtg in floors[elevator].container.iter() {
                let mut new_floors = floors.clone();
                new_floors[elevator].container.remove(rtg);
                new_floors[next_position].container.insert(rtg.clone());
                if is_floor_legal(&new_floors[elevator])
                    && is_floor_legal(&new_floors[next_position])
                {
                    let new_steps = steps + 1;
                    let h = heuristic(&new_floors);
                    queue.push(Config {
                        floors: new_floors,
                        elevator: next_position,
                        steps: new_steps,
                        total_distance: new_steps + h,
                    });
                }
            }

            // move 2 elements
            for comb in floors[elevator].container.iter().combinations(2) {
                let (rtg1, rtg2) = (comb[0], comb[1]);
                let mut new_floors = floors.clone();
                new_floors[elevator].container.remove(rtg1);
                new_floors[next_position].container.insert(rtg1.clone());

                new_floors[elevator].container.remove(rtg2);
                new_floors[next_position].container.insert(rtg2.clone());

                if is_floor_legal(&new_floors[elevator])
                    && is_floor_legal(&new_floors[next_position])
                {
                    let new_steps = steps + 1;
                    let h = heuristic(&new_floors);
                    queue.push(Config {
                        floors: new_floors,
                        elevator: next_position,
                        steps: new_steps,
                        total_distance: new_steps + h,
                    });
                }
            }
        }
    }

    panic!("Search did not find a solution!");
}

fn main() {
    let lines = stdin().lock().lines().map(|l| l.expect("Cannot read line"));

    let generator_re = Regex::new(r"(\w+) generator").unwrap();
    let microchip_re = Regex::new(r"(\w+)\-compatible microchip").unwrap();

    let mut floors = vec![
        Floor {
            container: HashSet::<RTG>::new()
        };
        FLOORS_NUM
    ];
    for (floor_i, line) in lines.enumerate() {
        for element in generator_re
            .captures_iter(line.as_str())
            .map(|cap| cap.get(1).unwrap().as_str())
        {
            floors[floor_i]
                .container
                .insert(RTG::Generator(element.to_string()));
        }

        for element in microchip_re
            .captures_iter(line.as_str())
            .map(|cap| cap.get(1).unwrap().as_str())
        {
            floors[floor_i]
                .container
                .insert(RTG::Microchip(element.to_string()));
        }
    }

    for element in ["elerium", "dilithium"] {
        let first_floor = &mut floors[0].container;
        first_floor.insert(RTG::Generator(element.to_string()));
        first_floor.insert(RTG::Microchip(element.to_string()));
    }

    let sol = search(floors, 0, 0);
    println!("{sol}"); // took 28 minutes
}
