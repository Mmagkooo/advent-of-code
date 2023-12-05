struct Range {
    destination_range_start: u32,
    source_range_start: u32,
    range_length: u32,
}

struct RangeMap {
    ranges: Vec<Range>,
}

impl RangeMap {
    fn find_match(&self, value: u32) -> u32 {
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
        let v: Vec<u32> = line.split(" ").map(|n| n.parse().unwrap()).collect();
        assert_eq!(v.len(), 3);
        Self {
            destination_range_start: v[0],
            source_range_start: v[1],
            range_length: v[2],
        }
    }

    fn find_match(&self, value: u32) -> Option<u32> {
        if (value >= self.source_range_start)
            && value < (self.source_range_start + self.range_length)
        {
            Some(value - self.source_range_start + self.destination_range_start)
        } else {
            None
        }
    }
}

fn parse_seeds_line(line: String) -> Vec<u32> {
    line.strip_prefix("seeds: ")
        .unwrap()
        .split(" ")
        .map(|n| n.parse().unwrap())
        .collect()
}

fn main() {
    let mut lines = std::io::stdin().lines().map(|l| l.unwrap());

    let mut seeds = parse_seeds_line(lines.next().unwrap());
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

        seeds
            .iter_mut()
            .for_each(|value| *value = range_map.find_match(*value));
    }

    println!("{}", seeds.iter().min().unwrap());
}
