use std::collections::BTreeMap;

/// (i, j)
type Location = (i32, i32);

#[derive(PartialEq, Eq, Copy, Clone, Debug, Hash, PartialOrd, Ord)]
enum Direction {
    R,
    L,
    U,
    D,
}

impl Direction {
    const fn value(&self) -> (i32, i32) {
        match self {
            Direction::R => (0, 1),
            Direction::L => (0, -1),
            Direction::U => (-1, 0),
            Direction::D => (1, 0),
        }
    }

    fn from_str(s: &str) -> Self {
        match s {
            "0" => Self::R,
            "1" => Self::D,
            "2" => Self::L,
            "3" => Self::U,
            other => panic!("Invalid Direction: {other}"),
        }
    }
}

#[derive(Debug, Clone)]
struct HorizontalLine {
    direction: Direction,
    width: i32,
}

#[derive(Debug)]
struct VerticalLine {
    j: i32,
    i_up: i32,
    i_down: i32,
    direction: Direction,
}

/// Applies 2d sweep line
fn main() {
    let mut vertical_lines = vec![];
    let mut horizontal_lines: BTreeMap<i32, Vec<HorizontalLine>> = Default::default();
    let mut location: Location = (0, 0);
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
            let direction = Direction::from_str(&line_parts[2][7..8]);
            let steps = i32::from_str_radix(&line_parts[2][2..7], 16).unwrap();
            let (di, dj) = direction.value();

            let next_location = (location.0 + di * steps, location.1 + dj * steps);

            match direction {
                Direction::R | Direction::L => {
                    horizontal_lines
                        .entry(location.0)
                        .or_insert(vec![])
                        .push(HorizontalLine {
                            direction,
                            width: location.1.abs_diff(next_location.1) as i32 + 1,
                        });
                }
                Direction::U | Direction::D => vertical_lines.push(VerticalLine {
                    j: location.1,
                    i_up: std::cmp::min(location.0, next_location.0),
                    i_down: std::cmp::max(location.0, next_location.0),
                    direction,
                }),
            };

            location = next_location;
        });

    vertical_lines.sort_by_key(|l| l.j);

    let mut total_volume: i64 = 0;
    let horizontal_end_direction = horizontal_lines.last_key_value().unwrap().1[0].direction;
    let vertical_end_direction = vertical_lines.last().unwrap().direction;

    let mut prev_i = *horizontal_lines.first_key_value().unwrap().0;
    let mut prev_width = 0;
    for (i, lines_at_i) in horizontal_lines {
        // first add everything up to the current i, but not including it
        total_volume += (i - prev_i) as i64 * prev_width;
        prev_i = i;

        let mut current_width = 0;
        let eligible_vertical_lines: Vec<&VerticalLine> = vertical_lines
            .iter()
            .filter(|vl| vl.i_up <= i && i < vl.i_down) // leaving out beginnings of new areas
            .collect();

        // calculate new width for next iteration
        let mut prev_j = 0; // shall be overwritten
        for vertical_line in eligible_vertical_lines.iter() {
            if vertical_line.direction == vertical_end_direction {
                current_width += vertical_line.j as i32 - prev_j + 1;
            }
            prev_j = vertical_line.j as i32;
        }
        prev_width = current_width as i64;

        // add those lines that are endings of current area
        let ending_horizontal_lines: Vec<_> = lines_at_i
            .iter()
            .filter(|l| l.direction == horizontal_end_direction)
            .collect();

        let mut ending_addition = 0;
        for ending_line in ending_horizontal_lines.iter() {
            ending_addition += ending_line.width as i64;
        }

        if ending_addition > 0 {
            // we might have counted some vertical lines - need to subtract those
            let mut extra = 0;
            for vertical_line in eligible_vertical_lines {
                if vertical_line.i_up == i {
                    extra += 1;
                }
            }
            ending_addition -= extra;
            total_volume += ending_addition;
        }
    }

    println!("{}", total_volume);
}
