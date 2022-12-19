use std::{
    cmp::{max, min},
    collections::HashSet,
    io::{stdin, BufRead},
};

const DIMS: usize = 3;
type Location = [i32; 3];

fn rek(
    loc: Location,
    min_loc: &Location,
    max_loc: &Location,
    cubes: &HashSet<Location>,
    visited: &mut HashSet<Location>,
) {
    if cubes.contains(&loc) {
        return;
    }

    if visited.contains(&loc) {
        return;
    }
    visited.insert(loc);

    for dim in 0..DIMS {
        if loc[dim] < min_loc[dim] || loc[dim] > max_loc[dim] {
            return;
        }
    }

    for dim in 0..DIMS {
        for delta in [-1, 0, 1] {
            let mut next_loc = loc.clone();
            next_loc[dim] += delta;
            rek(next_loc, min_loc, max_loc, cubes, visited);
        }
    }
}

fn main() {
    let mut min_loc = [1000; DIMS];
    let mut max_loc = [0; DIMS];
    let cubes: Vec<Location> = stdin()
        .lock()
        .lines()
        .map(|l| l.expect("Could not read line"))
        .map(|s| {
            let mut loc = [0; DIMS];
            s.split(",")
                .map(|d| d.parse().unwrap())
                .enumerate()
                .for_each(|(i, d)| {
                    min_loc[i] = min(min_loc[i], d);
                    max_loc[i] = max(max_loc[i], d);
                    loc[i] = d;
                });
            return loc;
        })
        .collect();
    println!("Initial cubes: {}", cubes.len());

    // ensure whole space wraps actual cubes
    for i in 0..DIMS {
        min_loc[i] -= 1;
        max_loc[i] += 1;
    }

    // define outer cube
    let mut whole_space: HashSet<Location> = HashSet::new();
    for i in min_loc[0]..max_loc[0] + 1 {
        for j in min_loc[1]..max_loc[1] + 1 {
            for k in min_loc[2]..max_loc[2] + 1 {
                whole_space.insert([i, j, k]);
            }
        }
    }
    println!("Whole space: {}", whole_space.len());

    // get all cubes that can be touched from the outside
    let cubes_set: HashSet<Location> = HashSet::from_iter(cubes.into_iter());
    let mut reachable_from_outside = HashSet::new();
    println!("Reachable from outside: {}", reachable_from_outside.len());
    rek(
        min_loc,
        &min_loc,
        &max_loc,
        &cubes_set,
        &mut reachable_from_outside,
    );

    let monolith_set = &whole_space - &reachable_from_outside;
    let cubes: Vec<Location> = Vec::from_iter(monolith_set.into_iter());
    println!("Final cubes: {}", cubes.len());

    let cubes_num = cubes.len();
    let mut sol = cubes_num * 6;
    for ci in 0..cubes_num {
        for cj in ci + 1..cubes_num {
            let mut diff = 0;
            for i in 0..3 {
                diff += (cubes[ci][i] - cubes[cj][i]).abs();
            }
            if diff == 1 {
                sol -= 2;
            }
        }
    }

    println!("{sol}");
}
