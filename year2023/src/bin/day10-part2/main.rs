use std::collections::{HashMap, HashSet};

/// (i, j)
type Address = (usize, usize);

#[derive(Debug, Clone, Copy, PartialEq)]
enum Direction {
    NORTH,
    SOUTH,
    EAST,
    WEST,
}

impl Direction {
    const ALL_VALUES: [Self; 4] = [
        Direction::NORTH,
        Direction::SOUTH,
        Direction::WEST,
        Direction::EAST,
    ];

    fn value(&self) -> (isize, isize) {
        match self {
            Direction::NORTH => (-1, 0),
            Direction::SOUTH => (1, 0),
            Direction::EAST => (0, 1),
            Direction::WEST => (0, -1),
        }
    }

    fn acceptable_symbols(&self) -> &[Symbol] {
        match self {
            Direction::NORTH => &[Symbol::F, Symbol::SEVEN, Symbol::VERTICAL],
            Direction::SOUTH => &[Symbol::L, Symbol::J, Symbol::VERTICAL],
            Direction::EAST => &[Symbol::J, Symbol::SEVEN, Symbol::HORIZONTAL],
            Direction::WEST => &[Symbol::L, Symbol::F, Symbol::HORIZONTAL],
        }
    }

    fn rotate_cw(d: &Self) -> Self {
        match d {
            Direction::NORTH => Direction::EAST,
            Direction::WEST => Direction::NORTH,
            Direction::SOUTH => Direction::WEST,
            Direction::EAST => Direction::SOUTH,
        }
    }

    fn rotate_ccw(d: &Self) -> Self {
        match d {
            Direction::NORTH => Direction::WEST,
            Direction::WEST => Direction::SOUTH,
            Direction::SOUTH => Direction::EAST,
            Direction::EAST => Direction::NORTH,
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum Symbol {
    L,
    J,
    F,
    SEVEN,
    HORIZONTAL,
    VERTICAL,
    EMPTY,
    START,
}

impl Symbol {
    fn next_direction(&self, direction: &Direction) -> Direction {
        match (self, direction) {
            (Symbol::L, Direction::SOUTH) => Direction::EAST,
            (Symbol::L, Direction::WEST) => Direction::NORTH,
            (Symbol::J, Direction::SOUTH) => Direction::WEST,
            (Symbol::J, Direction::EAST) => Direction::NORTH,
            (Symbol::F, Direction::NORTH) => Direction::EAST,
            (Symbol::F, Direction::WEST) => Direction::SOUTH,
            (Symbol::SEVEN, Direction::NORTH) => Direction::WEST,
            (Symbol::SEVEN, Direction::EAST) => Direction::SOUTH,
            (Symbol::HORIZONTAL, Direction::EAST) => Direction::EAST,
            (Symbol::HORIZONTAL, Direction::WEST) => Direction::WEST,
            (Symbol::VERTICAL, Direction::NORTH) => Direction::NORTH,
            (Symbol::VERTICAL, Direction::SOUTH) => Direction::SOUTH,
            (Symbol::START, any_dir) => *any_dir,
            invalid => panic!("Invalid combination: {invalid:?}"),
        }
    }

    fn from_char(c: char) -> Self {
        match c {
            'L' => Self::L,
            'J' => Self::J,
            'F' => Self::F,
            '7' => Self::SEVEN,
            '-' => Self::HORIZONTAL,
            '|' => Self::VERTICAL,
            '.' => Self::EMPTY,
            'S' => Self::START,
            invalid => panic!("Invalid symbol: {invalid}"),
        }
    }
}

fn change_address(address: &Address, direction: &Direction) -> Address {
    let (direction_i, direction_j) = direction.value();
    (
        (address.0 as isize + direction_i) as usize,
        (address.1 as isize + direction_j) as usize,
    )
}

/// for each loop member: try going as far inside the loop (orthogonal to pipes) as possible
fn count_enclosed(
    grid: &HashMap<Address, Symbol>,
    loop_vec: &Vec<(Address, Vec<Direction>)>,
    rotator: &dyn Fn(&Direction) -> Direction,
) -> Option<usize> {
    let loop_set: HashSet<Address> = loop_vec.iter().map(|(addr, _)| *addr).collect();
    let mut enclosed: HashSet<Address> = HashSet::new();
    for (loop_member_address, directions) in loop_vec.iter() {
        for direction in directions.iter() {
            let inner_direction = rotator(direction);
            let mut maybe_inner_address: Address = loop_member_address.clone();
            // iterate from pipes to inside (inner direction) as much as possible
            loop {
                maybe_inner_address = change_address(&maybe_inner_address, &inner_direction);
                if !grid.contains_key(&maybe_inner_address) {
                    // using wrong direction, should have used a different direction rotator
                    return None;
                }

                // breaking earlier if candidate in `enclosed` (for better performance) gives wrong result
                if loop_set.contains(&maybe_inner_address) {
                    break;
                }

                enclosed.insert(maybe_inner_address);
            }
        }
    }

    Some(enclosed.len())
}

fn main() {
    let mut start_address: Address = (0, 0);
    let mut grid: HashMap<Address, Symbol> = HashMap::new();
    std::io::stdin()
        .lines()
        .map(|l| l.unwrap())
        .enumerate()
        .for_each(|(i, line)| {
            line.char_indices().for_each(|(j, c)| {
                let address = (i, j);
                let symbol = Symbol::from_char(c);
                if let Symbol::START = symbol {
                    start_address = address.clone();
                }
                grid.insert(address, symbol);
            })
        });

    // stores sequence of addresses in the pipe loop + direction before and after address change
    let mut loop_vec: Vec<(Address, Vec<Direction>)> = vec![];
    // although it is easy to determine the start direction visually, it's done computationally to be input-agnostic
    for start_direction in Direction::ALL_VALUES {
        let mut current_direction = start_direction;
        let mut current_address = start_address;
        let mut found_pipe_loop = false;
        loop_vec = vec![];
        while let Some(current_symbol) = grid.get(&current_address) {
            if current_symbol == &Symbol::START && loop_vec.len() > 1 {
                found_pipe_loop = true;
                break;
            }

            let acceptable_symbols = current_direction.acceptable_symbols();
            if !acceptable_symbols.contains(current_symbol) && current_symbol != &Symbol::START {
                break;
            }

            let mut used_directions = vec![current_direction];

            let new_direction = current_symbol.next_direction(&current_direction);
            if new_direction != current_direction {
                used_directions.push(new_direction);
            }
            current_direction = new_direction;
            loop_vec.push((current_address, used_directions.clone()));
            current_address = change_address(&current_address, &current_direction);
        }

        if found_pipe_loop {
            break;
        }
    }

    for rotator in [Direction::rotate_ccw, Direction::rotate_cw] {
        if let Some(enclosed_count) = count_enclosed(&grid, &loop_vec, &rotator) {
            println!("{enclosed_count}");
        }
    }
}
