use std::{
    cmp::Ordering,
    io::{stdin, BufRead},
};

#[derive(Debug)]
enum EventKind {
    START,
    END,
}

#[derive(Debug)]
struct Event {
    value: i64,
    kind: EventKind,
}

fn main() {
    let lines: Vec<String> = stdin()
        .lock()
        .lines()
        .map(|l| l.expect("Cannot read"))
        .collect();

    let mut events = vec![];
    for line in lines {
        let words: Vec<&str> = line.split("-").collect();
        assert_eq!(words.len(), 2);

        let start: i64 = words[0].parse().unwrap();
        events.push(Event {
            value: start,
            kind: EventKind::START,
        });

        let end: i64 = words[1].parse().unwrap();
        events.push(Event {
            value: end + 1, // not inclusive
            kind: EventKind::END,
        });
    }

    events.sort_by(|e1, e2| match e1.value.cmp(&e2.value) {
        Ordering::Equal => match e1.kind {
            EventKind::START => Ordering::Less,
            EventKind::END => Ordering::Greater,
        },
        ord => ord,
    });

    let mut range_counter = 0;
    let mut last = -1;
    for Event { value, kind } in events {
        match kind {
            EventKind::START => range_counter += 1,
            EventKind::END => {
                range_counter += -1;
                last = value;
            }
        };

        if range_counter == 0 {
            break;
        }
    }

    println!("{}", last);
}
