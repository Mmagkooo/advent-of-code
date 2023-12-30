use std::collections::HashSet;

/// (i, j)
type Location = (isize, isize);
type Grid = Vec<Vec<char>>;

const START: char = 'S';
const ROCK: char = '#';

const DIRECTIONS: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, 1), (0, -1)];

fn make_steps(
    steps_left: usize,
    location: Location,
    visited: &mut HashSet<(Location, usize)>,
    grid: &Grid,
) {
    let state = (location, steps_left);

    if visited.contains(&state) {
        return;
    }
    visited.insert(state);

    if steps_left == 0 {
        return;
    }

    let height = grid.len() as isize;
    let width = grid[0].len() as isize;

    for (di, dj) in DIRECTIONS {
        let next_location = (location.0 + di, location.1 + dj);
        if next_location.0 < 0
            || next_location.0 >= height
            || next_location.1 < 0
            || next_location.1 >= width
        {
            continue;
        }

        if grid[next_location.0 as usize][next_location.1 as usize] != ROCK {
            make_steps(steps_left - 1, next_location, visited, grid);
        }
    }
}

fn find_start_location(grid: &Grid) -> Location {
    for (i, row) in grid.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if c == &START {
                return (i as isize, j as isize);
            }
        }
    }

    panic!("No start");
}

fn main() {
    let grid: Grid = std::io::stdin()
        .lines()
        .map(|l| l.unwrap())
        .map(|l| l.chars().collect())
        .collect();

    let start_location = find_start_location(&grid);

    let max_steps = std::env::var("MAX_STEPS").unwrap().parse().unwrap();
    let mut visited = HashSet::new();
    make_steps(max_steps, start_location, &mut visited, &grid);

    let sol = visited
        .iter()
        .filter(|(_, steps_left)| *steps_left == 0)
        .count();
    println!("{}", sol);
}
