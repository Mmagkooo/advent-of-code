use std::{
    collections::HashMap,
    io::{stdin, BufRead},
};

use itertools::Itertools;

type Point = (i32, i32);
type Blizzards = HashMap<Point, Vec<char>>;

const UP: Point = (-1, 0);
const RIGHT: Point = (0, 1);
const DOWN: Point = (1, 0);
const LEFT: Point = (0, -1);

type Config = (
    Point, // expedition
    i32,   // time modulo
    RaceNumber,
);

struct Field {
    blizzards: Blizzards,
    height: i32,
    width: i32,
    period: i32,
}

fn get_new_blizzards(
    field: &Field,
    time_modulo: i32,
    new_blizzards: &mut Blizzards,
    memo: &mut HashMap<i32, Blizzards>,
) {
    match memo.get(&time_modulo) {
        Some(blizzards) => {
            *new_blizzards = blizzards.clone();
            return;
        }
        None => (),
    };

    for (point, point_blizzards) in field.blizzards.iter() {
        for blizzard in point_blizzards {
            let direction = match blizzard {
                '<' => LEFT,
                '>' => RIGHT,
                'v' => DOWN,
                '^' => UP,
                invalid => panic!("Invalid char: {invalid}"),
            };

            let new_point = (
                (point.0 + direction.0 + field.height) % field.height,
                (point.1 + direction.1 + field.width) % field.width,
            );

            let mut new_point_blizzards = match new_blizzards.get(&new_point) {
                Some(point_blizzards) => point_blizzards.clone(),
                None => vec![],
            };
            new_point_blizzards.push(*blizzard);
            new_blizzards.insert(new_point, new_point_blizzards);
        }
    }

    memo.insert(time_modulo, new_blizzards.clone());
}

const MAX_TIME: i32 = 1000; // heuristic

fn is_in_bounds(point: &Point, field: &Field) -> bool {
    point == &(-1, 0)
        || point == &(field.height, field.width - 1)
        || (point.0 >= 0 && point.0 < field.height && point.1 >= 0 && point.1 < field.width)
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
enum RaceNumber {
    FIRST, // going from start to end
    SECOND, // returning to start
    THIRD, // and to end again
}

fn step(
    field: &Field,
    expedition: Point,
    time: i32,
    mut race_number: RaceNumber,
    memo: &mut HashMap<Config, i32>, // for checking if expedition was visited more efficiently at time_module
    blizzard_memo: &mut HashMap<i32, Blizzards>, // for more quickly fetching the next blizzard configuration
) {
    // check validity
    let blizzards_at_expedition = field.blizzards.get(&expedition);
    if blizzards_at_expedition.is_some() && blizzards_at_expedition.unwrap().len() > 0 {
        return;
    }

    if time > MAX_TIME {
        return;
    }

    // check memory
    let time_modulo = time % field.period;
    let config: Config = (expedition, time_modulo, race_number);
    match memo.get(&config) {
        Some(old_time) => {
            if time >= *old_time {
                return;
            }
        }
        None => (),
    }
    memo.insert(config, time);

    // check if end
    match race_number {
        RaceNumber::FIRST => {
            if expedition == (field.height, field.width - 1) {
                println!("Reached end for the first time in {}", time);
                race_number = RaceNumber::SECOND;
            }
        }
        RaceNumber::SECOND => {
            if expedition == (-1, 0) {
                println!("Back to start in {}", time);
                race_number = RaceNumber::THIRD;
            }
        }
        RaceNumber::THIRD => {
            if expedition == (field.height, field.width - 1) {
                println!("Reached end in {}", time);
                return;
            }
        }
    };

    // move blizzards by one step
    let mut new_blizzards = Blizzards::new();
    get_new_blizzards(field, time_modulo, &mut new_blizzards, blizzard_memo);

    let new_field = Field {
        blizzards: new_blizzards,
        height: field.height,
        width: field.width,
        period: field.period,
    };

    // visit adjacent
    for direction in [(0, 0), UP, RIGHT, DOWN, LEFT] {
        let new_expedition = (expedition.0 + direction.0, expedition.1 + direction.1);
        if is_in_bounds(&new_expedition, &new_field) {
            step(
                &new_field,
                new_expedition,
                time + 1,
                race_number,
                memo,
                blizzard_memo,
            );
        }
    }
}

fn main() {
    let mut blizzards = Blizzards::new();
    let mut lines: Vec<String> = stdin()
        .lock()
        .lines()
        .skip(1) // drop first line as it only contains walls
        .map(|line| line.expect("Could not read line"))
        .collect();

    lines.pop(); // drop last line

    for (i, line) in lines.iter().enumerate() {
        for (j, field_symbol) in line.chars().dropping(1).dropping_back(1).enumerate() {
            if "><v^".contains(field_symbol) {
                blizzards.insert((i as i32, j as i32), vec![field_symbol]);
            } else if field_symbol == '.' {
                // pass
            } else {
                panic!("Something's wrong");
            }
        }
    }

    let height = lines.len() as i32;
    let width = lines.iter().nth(0).unwrap().len() as i32 - 2;
    let field: Field = Field {
        blizzards,
        height,
        width,
        period: height * width,
    };

    let mut memo = HashMap::new();
    let mut blizzard_memo = HashMap::new();
    step(
        &field,
        (-1, 0),
        0,
        RaceNumber::FIRST,
        &mut memo,
        &mut blizzard_memo,
    );

    // end was reached on multiple occassions, need to find quickest one
    let min_time = memo
        .iter()
        .map(|((point, _, race_number), value)| (point, race_number, value))
        .filter(|(point, race_number, _)| {
            **point == (height, width - 1) && **race_number == RaceNumber::THIRD
        })
        .map(|(_, _, value)| value)
        .min()
        .unwrap();
    // optimal route is not necessarily the sum of optimal subroutes (optimal firs +, optimal second + optimal third)
    println!("{}", min_time);
}
