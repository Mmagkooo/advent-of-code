use std::collections::{HashMap, HashSet};

/// (i, j)
type Address = (usize, usize);
type StoneGridMap = HashMap<Address, char>;

const _ROCK: char = '#';
const STONE: char = 'O';
const EMPTY: char = '.';

#[derive(PartialEq, Eq, Clone)]
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

    fn tilt_south(&mut self) {
        for i in (0..self.height).rev() {
            for j in 0..self.width {
                let c = *self.map.get(&(i, j)).unwrap();
                if c != STONE {
                    continue;
                }

                self.map.insert((i, j), EMPTY);
                let mut curr_col_i = i;
                while curr_col_i < self.height {
                    if Some(&EMPTY) == self.map.get(&(curr_col_i + 1, j)) {
                        curr_col_i += 1;
                    } else {
                        break;
                    }
                }

                self.map.insert((curr_col_i, j), STONE);
            }
        }
    }

    /// <---
    fn tilt_west(&mut self) {
        for i in 0..self.height {
            for j in 1..self.width {
                let c = *self.map.get(&(i, j)).unwrap();
                if c != STONE {
                    continue;
                }

                self.map.insert((i, j), EMPTY);
                let mut curr_row_j = j;
                while curr_row_j > 0 {
                    if Some(&EMPTY) == self.map.get(&(i, curr_row_j - 1)) {
                        curr_row_j -= 1;
                    } else {
                        break;
                    }
                }

                self.map.insert((i, curr_row_j), STONE);
            }
        }
    }

    /// --->
    fn tilt_east(&mut self) {
        for i in 0..self.height {
            for j in (0..self.width).rev() {
                let c = *self.map.get(&(i, j)).unwrap();
                if c != STONE {
                    continue;
                }

                self.map.insert((i, j), EMPTY);
                let mut curr_row_j = j;
                while curr_row_j < self.width {
                    if Some(&EMPTY) == self.map.get(&(i, curr_row_j + 1)) {
                        curr_row_j += 1;
                    } else {
                        break;
                    }
                }

                self.map.insert((i, curr_row_j), STONE);
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

    /// Returns (discarded, period)
    fn period(&self) -> (Vec<u32>, Vec<u32>) {
        let mut grid = self.clone();

        let mut period_vec = vec![grid.calculate_load()];
        let mut grid_memo: HashSet<StoneGrid> = HashSet::new();

        let mut i = 0;
        loop {
            grid.tilt_north();
            grid.tilt_west();
            grid.tilt_south();
            grid.tilt_east();

            i += 1;
            println!("after cycle {}:\n{grid}", i);

            match grid_memo.contains(&grid) {
                true => {
                    let period_start = period_vec
                        .iter()
                        .position(|load| *load == grid.calculate_load())
                        .unwrap();
                    let (discarded, period) = period_vec.split_at(period_start);
                    return (discarded.to_vec(), period.to_vec());
                }
                false => {
                    period_vec.push(grid.calculate_load());
                    grid_memo.insert(grid.clone());
                }
            };
        }
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
    let n_cycles: usize = std::env::var("CYCLES").unwrap().parse().unwrap();

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

    let grid = StoneGrid::new(map);

    let (discarded, period) = grid.period();
    println!("Discarded: {discarded:?}, Period: {:?}", period);

    let sol = if n_cycles < discarded.len() {
        // this case is not likely to be input, but covered here for completeness' sake
        discarded[n_cycles]
    } else {
        period[(n_cycles - discarded.len()) % period.len()]
    };

    println!("{}", sol);
}
