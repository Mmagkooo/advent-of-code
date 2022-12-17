use regex::{Captures, Regex};
use std::{
    env,
    io::{self, BufRead}, process::exit,
};

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

fn dist(p1: &Point, p2: &Point) -> i32 {
    (p1.x - p2.x).abs() + (p1.y - p2.y).abs()
}

fn extract(captures: &Captures, name: &str) -> i32 {
    captures.name(name).unwrap().as_str().parse().unwrap()
}

#[derive(Debug, PartialEq)]
enum RangeEventType {
    Start,
    End,
}

/**
 * Abstraction of interacting with a [x, y] range
 */
#[derive(Debug)]
struct RangeEvent {
    x: i32,
    event_type: RangeEventType,
}

struct Sensor {
    centre: Point,
    radius: i32,
}

fn get_range_events(sensors: &Vec<Sensor>, target_y: i32) -> Vec<RangeEvent> {
    let mut range_events: Vec<RangeEvent> = vec![];
    for Sensor { centre, radius } in sensors {
        // only handle if centre.y - radius <= target_y <= centre.y + radius
        if centre.y - radius <= target_y && target_y <= centre.y + radius {
            // x < centre.x
            let x1 = -radius + (target_y - centre.y).abs() + centre.x;
            range_events.push(RangeEvent {
                x: x1,
                event_type: RangeEventType::Start,
            });

            // x >= centre.x
            let x2 = radius - (target_y - centre.y).abs() + centre.x;
            range_events.push(RangeEvent {
                x: x2,
                event_type: RangeEventType::End,
            });
        }
    }
    return range_events;
}

fn main() {
    let lines: Vec<String> = io::stdin()
        .lock()
        .lines()
        .map(|l| l.expect("Could not read line"))
        .collect();

    let re = Regex::new(
        r"^Sensor at x=(?P<cx>.+), y=(?P<cy>.+): closest beacon is at x=(?P<bx>.+), y=(?P<by>.+)$",
    )
    .unwrap();

    let mut sensors: Vec<Sensor> = vec![];
    for line in lines {
        let captures = re.captures(line.as_str()).expect("Couldn't capture");
        let centre = Point {
            x: extract(&captures, "cx"),
            y: extract(&captures, "cy"),
        };

        let beacon = Point {
            x: extract(&captures, "bx"),
            y: extract(&captures, "by"),
        };
        let radius = dist(&centre, &beacon);
        sensors.push(Sensor { centre, radius });
    }

    let argv: Vec<String> = env::args().collect();
    let limit = argv[1].parse().expect("Could not parse argv");
    for target_y in 0..limit + 1 {
        let mut range_events = get_range_events(&sensors, target_y);
        range_events.sort_by_key(|re| {
            (
                re.x,
                // starts should come before ends
                match re.event_type {
                    RangeEventType::Start => -1,
                    RangeEventType::End => 1,
                },
            )
        });

        let mut active_ranges = 0;
        let range_events_num = range_events.len();
        for (i, range_event) in range_events.iter().enumerate() {
            active_ranges += match range_event.event_type {
                RangeEventType::Start => 1,
                RangeEventType::End => -1,
            };

            // assumption, once no active ranges, it is enough to move to the next point to the right
            // unless the next range is adjacent to the last one (e.g. [1,2] and [2,3])
            let target_x = range_event.x + 1;
            if active_ranges == 0
                && target_x >= 0
                && range_event.x <= limit
                && i < range_events_num
                && range_event.x + 1 != range_events[i + 1].x
            {
                println!("Beacon: {}, {}", target_x, target_y);
                println!("{}", target_x as i64 * 4000000 + target_y as i64);
                exit(0);
            }
        }
    }
}
