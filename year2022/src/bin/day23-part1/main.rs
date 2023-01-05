use std::{
    cmp::{max, min},
    collections::{HashMap, HashSet},
    env,
    io::{stdin, BufRead},
};

type Point = (i32, i32);

const DIRECTIONS_NUM: i32 = 8;
const DIRECTIONS: [Point; DIRECTIONS_NUM as usize] = [
    (-1, 0),  // N
    (-1, 1),  // NE
    (0, 1),   // E
    (1, 1),   // SE
    (1, 0),   // S
    (1, -1),  // SW
    (0, -1),  // W
    (-1, -1), // NW
];

const CARDINAL_DIRECTIONS_NUM: i32 = 4;
const CARDINAL_DIRECTION_INDICES: [i32; CARDINAL_DIRECTIONS_NUM as usize] = [0, 4, 6, 2];

fn count_neighbors(elf: Point, elves: &HashSet<Point>) -> u32 {
    let mut neighbors_cnt: u32 = 0;
    for direction in DIRECTIONS {
        let neighbor = (elf.0 + direction.0, elf.1 + direction.1);
        if elves.contains(&neighbor) {
            neighbors_cnt += 1;
        }
    }

    return neighbors_cnt;
}

fn move_in_direction(elf: Point, direction_index: i32, elves: &HashSet<Point>) -> Option<Point> {
    let neighbors = [0, -1, 1]
        .map(|dir_index_delta: i32| {
            DIRECTIONS
                [((direction_index + dir_index_delta + DIRECTIONS_NUM) % DIRECTIONS_NUM) as usize]
        })
        .map(|direction| (elf.0 + direction.0, elf.1 + direction.1));

    return match neighbors.iter().any(|neighbor| elves.contains(&neighbor)) {
        true => None,
        false => Some(neighbors[0]),
    };
}

fn parse_input() -> HashSet<Point> {
    let lines = stdin().lock().lines();
    let mut elves = HashSet::<Point>::new();
    for (i, line) in lines.enumerate() {
        for (j, symbol) in line.expect("Could not read line").chars().enumerate() {
            if symbol == '#' {
                elves.insert((i as i32, j as i32));
            }
        }
    }

    return elves;
}

fn print_elves(elves: &HashSet<Point>) {
    let start = -1;
    let end = 10;

    for _ in 0..2 {
        print!(" ");
    }
    for j in start..end {
        print!("{}", (j + 10) % 10);
    }
    println!();

    for _ in 0..2 {
        print!(" ");
    }
    for _ in start..end {
        print!("-");
    }
    println!();

    for i in start..end {
        print!("{}|", (i + 10) % 10);
        for j in start..end {
            let symbol = match elves.contains(&(i, j)) {
                true => '#',
                false => '.',
            };
            print!("{}", symbol);
        }
        println!();
    }
    println!();
}

fn main() {
    let mut elves = parse_input();

    let argv: Vec<String> = env::args().collect();
    let steps: usize = argv[1].parse().expect("Expected CLI args: <STEPS>");
    print_elves(&elves);

    let mut starting_direction_index_index = 0;
    for _ in 0..steps {
        let mut new_elves = HashSet::<Point>::new();
        let mut targeted_by = HashMap::<Point, i32>::new();
        let mut elf_to_new_elf = HashMap::<Point, Point>::new();
        for elf in elves.clone() {
            if count_neighbors(elf, &elves) == 0 {
                new_elves.insert(elf);
            } else {
                let mut direction_index_index: i32 = starting_direction_index_index;
                for _ in 0..CARDINAL_DIRECTIONS_NUM {
                    let direction_index =
                        CARDINAL_DIRECTION_INDICES[direction_index_index as usize];
                    match move_in_direction(elf, direction_index, &elves) {
                        Some(new_elf) => {
                            let amount = match targeted_by.get(&new_elf) {
                                Some(amount) => *amount,
                                None => 0,
                            };
                            targeted_by.insert(new_elf, amount + 1);
                            elf_to_new_elf.insert(elf, new_elf);
                            break;
                        }
                        None => (),
                    };
                    direction_index_index = (direction_index_index + 1) % CARDINAL_DIRECTIONS_NUM;
                }

                // if couldn't move in any direction, stay in the same place
                if !elf_to_new_elf.contains_key(&elf) {
                    new_elves.insert(elf);
                }
            }
        }

        for (elf, new_elf) in elf_to_new_elf {
            if targeted_by.get(&new_elf).unwrap() == &1 {
                new_elves.insert(new_elf);
            } else {
                new_elves.insert(elf);
            }
        }
        elves = new_elves;
        print_elves(&elves);

        starting_direction_index_index = (starting_direction_index_index + 1) % CARDINAL_DIRECTIONS_NUM;
    }

    let mut min_x = i32::MAX;
    let mut max_x = i32::MIN;
    let mut min_y = i32::MAX;
    let mut max_y = i32::MIN;

    for elf in elves.iter() {
        min_x = min(min_x, elf.0);
        max_x = max(max_x, elf.0);
        min_y = min(min_y, elf.1);
        max_y = max(max_y, elf.1);
    }

    let min_rectangle_area = (max_x - min_x + 1) * (max_y - min_y + 1);
    println!("{}", min_rectangle_area - elves.len() as i32);
}
