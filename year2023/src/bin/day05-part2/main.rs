use std::{sync::mpsc, thread};

type Numeric = u64;

#[derive(Clone)]
struct Range {
    destination_range_start: Numeric,
    source_range_start: Numeric,
    range_length: Numeric,
}

#[derive(Clone)]
struct RangeMap {
    ranges: Vec<Range>,
}

impl RangeMap {
    fn find_match(&self, value: Numeric) -> Numeric {
        for range in self.ranges.iter() {
            if let Some(m) = range.find_match(value) {
                return m;
            }
        }

        // default if no ranges are suitable: value is mapped to value
        value
    }
}

impl Range {
    fn new(line: String) -> Self {
        let v: Vec<Numeric> = line.split(" ").map(|n| n.parse().unwrap()).collect();
        assert_eq!(v.len(), 3);
        Self {
            destination_range_start: v[0],
            source_range_start: v[1],
            range_length: v[2],
        }
    }

    fn find_match(&self, value: Numeric) -> Option<Numeric> {
        if (value >= self.source_range_start)
            && value < (self.source_range_start + self.range_length)
        {
            Some(value - self.source_range_start + self.destination_range_start)
        } else {
            None
        }
    }
}

fn parse_seeds_line(line: String) -> Vec<Numeric> {
    line.strip_prefix("seeds: ")
        .unwrap()
        .split(" ")
        .map(|n| n.parse().unwrap())
        .collect()
}

fn main() {
    let mut lines = std::io::stdin().lines().map(|l| l.unwrap());

    let seeds = parse_seeds_line(lines.next().unwrap());

    let mut range_maps: Vec<RangeMap> = vec![];
    lines.next(); // empty line
    while lines.next().is_some() {
        // the loop check consumes the map title

        let mut ranges: Vec<Range> = vec![];
        while let Some(line) = lines.next() {
            if line.is_empty() {
                break;
            }
            ranges.push(Range::new(line));
        }
        let range_map = RangeMap { ranges };
        range_maps.push(range_map);
    }

    let (sender, receiver) = mpsc::channel::<Numeric>();

    for (seed_i, seed_start) in seeds.iter().enumerate().step_by(2) {
        let seed_start = *seed_start;
        let range_length = seeds[seed_i + 1];
        let range_maps = range_maps.clone();
        let sender_i = sender.clone();
        thread::spawn(move || {
            let mut min_location = Numeric::MAX;
            for mut seed in seed_start..(seed_start + range_length) {
                for range_map in range_maps.iter() {
                    seed = range_map.find_match(seed);
                }
                min_location = std::cmp::min(min_location, seed);
            }
            sender_i.send(min_location).unwrap();
            println!("Finished seed_i: {seed_i}, min_location: {min_location}");
        });
    }

    drop(sender); // must drop all instances of tx to start receiving values by rx
    println!("{}", receiver.iter().min().unwrap());
}
