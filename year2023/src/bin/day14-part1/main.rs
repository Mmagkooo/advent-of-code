use std::collections::HashMap;

/// (i, j)
type Address = (usize, usize);
type StoneGridMap = HashMap<Address, char>;

const _ROCK: char = '#';
const STONE: char = 'O';
const EMPTY: char = '.';

struct StoneGrid {
    map: StoneGridMap,
    height: usize,
    width: usize,
}

impl StoneGrid {
    fn new(map: StoneGridMap) -> Self {
        let mut max_i = 0;
        let mut max_j = 0;
        for (i, j) in map.keys() {
            max_i = std::cmp::max(max_i, *i);
            max_j = std::cmp::max(max_j, *j);
        }

        Self {
            map,
            height: max_i + 1,
            width: max_j + 1,
        }
    }

    fn tilt_north(&mut self) {
        for i in 1..self.height {
            for j in 0..self.width {
                let c = *self.map.get(&(i, j)).unwrap();
                if c != STONE {
                    continue;
                }

                self.map.insert((i, j), EMPTY);
                let mut curr_col_i = i;
                while curr_col_i > 0 {
                    if Some(&EMPTY) == self.map.get(&(curr_col_i - 1, j)) {
                        curr_col_i -= 1;
                    } else {
                        break;
                    }
                }

                self.map.insert((curr_col_i, j), STONE);
            }
        }
    }

    fn calculate_load(&self) -> u32 {
        let mut load = 0;
        let height = self.height as u32;
        for ((i, _), c) in self.map.iter() {
            if *c == STONE {
                load += height - *i as u32
            }
        }

        return load;
    }
}

impl std::hash::Hash for StoneGrid {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        for i in 0..self.height {
            for j in 0..self.width {
                self.map.get(&(i, j)).hash(state)
            }
        }
    }
}

impl std::fmt::Display for StoneGrid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..self.height {
            for j in 0..self.width {
                let c = self.map.get(&(i, j)).unwrap();
                write!(f, "{}", c)?;
            }
            write!(f, "\n")?;
        }

        write!(f, "")
    }
}

fn main() {
    let mut map: StoneGridMap = StoneGridMap::new();
    std::io::stdin()
        .lines()
        .map(|l| l.unwrap())
        .enumerate()
        .for_each(|(i, line)| {
            line.chars().enumerate().for_each(|(j, c)| {
                map.insert((i, j), c);
            });
        });

    let mut grid = StoneGrid::new(map);
    println!("initial grid:\n{grid}");
    grid.tilt_north();
    println!("after tilt:\n{grid}");
    println!("{}", grid.calculate_load());
}
