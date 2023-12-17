use std::collections::{BinaryHeap, HashMap};

/// (i, j)
type Location = (isize, isize);

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
enum Direction {
    UP,
    DOWN,
    RIGHT,
    LEFT,
}

impl Direction {
    /// (di, dj)
    const fn value(&self) -> (isize, isize) {
        match self {
            Direction::UP => (-1, 0),
            Direction::DOWN => (1, 0),
            Direction::RIGHT => (0, 1),
            Direction::LEFT => (0, -1),
        }
    }

    const fn turn_left(&self) -> Self {
        match self {
            Direction::UP => Direction::LEFT,
            Direction::DOWN => Direction::RIGHT,
            Direction::RIGHT => Direction::UP,
            Direction::LEFT => Direction::DOWN,
        }
    }

    const fn turn_right(&self) -> Self {
        match self {
            Direction::UP => Direction::RIGHT,
            Direction::DOWN => Direction::LEFT,
            Direction::RIGHT => Direction::DOWN,
            Direction::LEFT => Direction::UP,
        }
    }
}

type Grid = Vec<Vec<u32>>;

const MAX_STRAIGHT_STEPS: u32 = 3;

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
struct State {
    location: Location,
    straight_steps: u32,
    direction: Direction,
}

#[derive(PartialEq, Eq)]
struct ScoredState {
    state: State,
    heat_loss_so_far: u32,
    estimated_total_loss: u32,
}

impl PartialOrd for ScoredState {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ScoredState {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // want to get minimum first
        other.estimated_total_loss.cmp(&self.estimated_total_loss)
    }
}

#[inline]
fn is_end(location: &Location, height: isize, width: isize) -> bool {
    location.0 == height - 1 && location.1 == width - 1
}

#[inline]
fn location_legal(location: &Location, height: isize, width: isize) -> bool {
    location.0 >= 0 && location.0 < height && location.1 >= 0 && location.1 < width
}

fn search(grid: &Grid) -> u32 {
    let mut seen = HashMap::<State, u32>::new();

    let height = grid.len() as isize;
    let width = grid[0].len() as isize;

    let heuristic = |location: &Location| (height - location.0 + width - location.1) as u32;

    let mut queue = BinaryHeap::<ScoredState>::new();

    let initial_location = (0, 0);
    for initial_direction in [Direction::DOWN, Direction::RIGHT] {
        queue.push(ScoredState {
            state: State {
                location: initial_location,
                straight_steps: 0,
                direction: initial_direction,
            },
            heat_loss_so_far: 0,
            estimated_total_loss: heuristic(&initial_location),
        })
    }

    // cost estimation only needed for sorting
    while let Some(ScoredState {
        state: current_state,
        heat_loss_so_far,
        ..
    }) = queue.pop()
    {
        // the location is legal - checked before adding to the queue
        let current_location = current_state.location;

        if let Some(old_heat_loss) = seen.get(&current_state) {
            if heat_loss_so_far >= *old_heat_loss {
                continue;
            }
        }
        seen.insert(current_state.clone(), heat_loss_so_far);

        if is_end(&current_state.location, height, width) {
            return heat_loss_so_far;
        }

        // determining next step
        let mut next_directions = vec![
            (current_state.direction.turn_left(), 1),
            (current_state.direction.turn_right(), 1),
        ];
        if current_state.straight_steps < MAX_STRAIGHT_STEPS {
            next_directions.push((current_state.direction, current_state.straight_steps + 1));
        }

        for (next_direction, next_straight_steps) in next_directions {
            let (change_i, change_j) = next_direction.value();
            let next_location = (current_location.0 + change_i, current_location.1 + change_j);
            if !location_legal(&next_location, height, width) {
                continue;
            }
            let next_heat_loss =
                heat_loss_so_far + grid[next_location.0 as usize][next_location.1 as usize];
            queue.push(ScoredState {
                state: State {
                    location: next_location,
                    straight_steps: next_straight_steps,
                    direction: next_direction,
                },
                heat_loss_so_far: next_heat_loss,
                estimated_total_loss: next_heat_loss + heuristic(&next_location),
            })
        }
    }

    panic!("Exhausted all states; solution not found!");
}

fn main() {
    let grid: Grid = std::io::stdin()
        .lines()
        .map(|l| l.unwrap().chars().map(|c| c as u32 - '0' as u32).collect())
        .collect();

    println!("{}", search(&grid));
}
