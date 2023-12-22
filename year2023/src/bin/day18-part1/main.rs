use std::collections::HashSet;

/// (i, j)
type Location = (isize, isize);

type History = HashSet<Location>;

enum Direction {
    R,
    L,
    U,
    D,
}

impl Direction {
    const fn value(&self) -> (isize, isize) {
        match self {
            Direction::R => (0, 1),
            Direction::L => (0, -1),
            Direction::U => (-1, 0),
            Direction::D => (1, 0),
        }
    }

    const ALL_VALUES: [(isize, isize); 4] = [
        Self::R.value(),
        Self::L.value(),
        Self::U.value(),
        Self::D.value(),
    ];

    fn from_str(s: &str) -> Self {
        match s {
            "R" => Self::R,
            "L" => Self::L,
            "U" => Self::U,
            "D" => Self::D,
            other => panic!("Invalid Direction: {other}"),
        }
    }
}

fn flood_fill(history: &mut History, top_location: Location) {
    let start_location = (top_location.0 + 1, top_location.1);

    let mut queue = vec![start_location];
    while !queue.is_empty() {
        let current_location = queue.pop().unwrap();
        if history.contains(&current_location) {
            continue;
        }
        history.insert(current_location);

        for (next_i, next_j) in Direction::ALL_VALUES {
            queue.push((current_location.0 + next_i, current_location.1 + next_j))
        }
    }
}

fn main() {
    let mut location: Location = (0, 0);
    let mut history: History = History::from_iter(vec![location]);
    let mut top_locations = vec![location];
    std::io::stdin()
        .lines()
        .map(|l| {
            l.unwrap()
                .split_whitespace()
                .map(|s| s.into())
                .collect::<Vec<String>>()
        })
        .for_each(|line_parts| {
            assert_eq!(line_parts.len(), 3);
            let direction = Direction::from_str(&line_parts[0]);
            let steps: usize = line_parts[1].parse().unwrap();

            let (di, dj) = direction.value();
            for _ in 0..steps {
                location = (location.0 + di, location.1 + dj);
                history.insert(location);

                match location.0.cmp(&top_locations[0].0) {
                    std::cmp::Ordering::Less => {
                        top_locations = vec![location];
                    }
                    std::cmp::Ordering::Equal => {
                        top_locations.push(location);
                    }
                    std::cmp::Ordering::Greater => (),
                }
            }
        });

    // count the fields in the inside by floodfilling from the middle of the topmost edge
    top_locations.sort();
    top_locations.dedup();
    assert!(top_locations.len() >= 3);

    // assuming there are no sharp turns, i.e. no two parallel touching trench branches
    let mid_top_location = top_locations[1];
    flood_fill(&mut history, mid_top_location);

    println!("{}", history.len());
}
