use regex::{Captures, Regex};
use std::{
    collections::HashSet,
    env,
    io::{self, BufRead},
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

fn main() {
    let target_y: i32 = env::args()
        .collect::<Vec<String>>()
        .get(1)
        .expect("program: <TARGET_Y>")
        .parse()
        .expect("program: <TARGET_Y> <--numeric");
    let lines: Vec<String> = io::stdin()
        .lock()
        .lines()
        .map(|l| l.expect("Could not read line"))
        .collect();

    let re = Regex::new(
        r"^Sensor at x=(?P<cx>.+), y=(?P<cy>.+): closest beacon is at x=(?P<bx>.+), y=(?P<by>.+)$",
    )
    .unwrap();

    let mut beacons_at_target_y: HashSet<Point> = HashSet::new();
    let mut range_events: Vec<RangeEvent> = vec![];
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
        if beacon.y == target_y {
            beacons_at_target_y.insert(beacon);
        };

        let radius = dist(&centre, &beacon);
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

    let mut range_event_iter = range_events.iter();
    let first_range_event = range_event_iter.next().unwrap();
    let mut last_x = first_range_event.x;
    let mut active_ranges = 1;
    let mut beacon_impossible = 0;
    for range_event in range_event_iter {
        if active_ranges > 0 {
            beacon_impossible += range_event.x - last_x;
        }
        active_ranges += match range_event.event_type {
            RangeEventType::Start => 1,
            RangeEventType::End => -1,
        };
        if active_ranges == 0 {
            beacon_impossible += 1;
        }
        last_x = range_event.x;
    }

    println!("{}", beacon_impossible - beacons_at_target_y.len() as i32);
}
