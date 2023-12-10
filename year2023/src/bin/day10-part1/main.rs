use std::collections::HashMap;

/// (i, j)
type Address = (usize, usize);

#[derive(Debug, Clone, Copy)]
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

    let mut distance: i32 = 0;
    // although it is easy to determine the start direction visually, it's done computationally to be input-agnostic
    for start_direction in Direction::ALL_VALUES {
        let mut current_direction = start_direction;
        let mut current_address = start_address;
        distance = 0;
        let mut found_loop = false;
        while let Some(current_symbol) = grid.get(&current_address) {
            if current_symbol == &Symbol::START && distance > 0 {
                found_loop = true;
                break;
            }

            let acceptable_symbols = current_direction.acceptable_symbols();
            if !acceptable_symbols.contains(current_symbol) && current_symbol != &Symbol::START {
                break;
            }

            current_direction = current_symbol.next_direction(&current_direction);
            current_address = change_address(&current_address, &current_direction);
            distance += 1;
        }

        if found_loop {
            break;
        }
    }

    // move one step forward to reach start, the farthest point is halfway
    println!("{}", (distance + 1) / 2);
}
