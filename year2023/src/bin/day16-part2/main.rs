use std::collections::HashSet;

/// (i, j)
type Location = (isize, isize);

#[derive(Clone, PartialEq, Eq, Hash)]
enum Direction {
    UP,
    DOWN,
    RIGHT,
    LEFT,
}

impl Direction {
    /// (di, dj)
    fn value(&self) -> (isize, isize) {
        match self {
            Direction::UP => (-1, 0),
            Direction::DOWN => (1, 0),
            Direction::RIGHT => (0, 1),
            Direction::LEFT => (0, -1),
        }
    }
}

enum Tile {
    Empty,
    SlashMirror,
    BackslashMirror,
    VerticalSplitter,
    HorizontalSplitter,
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '.' => Self::Empty,
            '/' => Self::SlashMirror,
            '\\' => Self::BackslashMirror,
            '|' => Self::VerticalSplitter,
            '-' => Self::HorizontalSplitter,
            other => panic!("Invalid tile: {other}"),
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct Beam {
    location: Location,
    direction: Direction,
}

impl Beam {
    /// move would be a better name, but it's a reserved keyword and r#move is ugly
    fn change_location(&mut self) {
        let direction = self.direction.value();
        self.location = (self.location.0 + direction.0, self.location.1 + direction.1);
    }

    fn change_direction(&self, tile: &Tile) -> Vec<Self> {
        let mut new_beams = vec![];

        match tile {
            Tile::Empty => new_beams.push(self.clone()),
            Tile::SlashMirror => {
                let new_direction = match self.direction {
                    Direction::UP => Direction::RIGHT,
                    Direction::DOWN => Direction::LEFT,
                    Direction::RIGHT => Direction::UP,
                    Direction::LEFT => Direction::DOWN,
                };
                new_beams.push(Beam {
                    location: self.location,
                    direction: new_direction,
                })
            }
            Tile::BackslashMirror => {
                let new_direction = match self.direction {
                    Direction::UP => Direction::LEFT,
                    Direction::DOWN => Direction::RIGHT,
                    Direction::RIGHT => Direction::DOWN,
                    Direction::LEFT => Direction::UP,
                };
                new_beams.push(Beam {
                    location: self.location,
                    direction: new_direction,
                })
            }
            Tile::VerticalSplitter => match self.direction {
                Direction::UP | Direction::DOWN => new_beams.push(self.clone()),
                Direction::RIGHT | Direction::LEFT => {
                    for new_direction in [Direction::UP, Direction::DOWN] {
                        new_beams.push(Beam {
                            location: self.location,
                            direction: new_direction,
                        });
                    }
                }
            },
            Tile::HorizontalSplitter => match self.direction {
                Direction::LEFT | Direction::RIGHT => new_beams.push(self.clone()),
                Direction::UP | Direction::DOWN => {
                    for new_direction in [Direction::LEFT, Direction::RIGHT] {
                        new_beams.push(Beam {
                            location: self.location,
                            direction: new_direction,
                        });
                    }
                }
            },
        };

        new_beams
    }
}

type Grid = Vec<Vec<Tile>>;

fn contains(grid: &Grid, location: &Location) -> bool {
    location.0 >= 0
        && location.0 < grid.len() as isize
        && location.1 >= 0
        && location.1 < grid[0].len() as isize
}

fn energize_tiles(grid: &Grid, initial_beam: Beam) -> usize {
    let mut beams: Vec<Beam> = vec![initial_beam.clone()];
    let mut prev_beam_history_length: isize = -1;
    let mut beam_history: HashSet<Beam> = HashSet::new();
    let mut energized_tiles: HashSet<Location> = HashSet::new();
    while beam_history.len() as isize > prev_beam_history_length {
        prev_beam_history_length = beam_history.len() as isize;

        let mut new_beams = vec![];
        for beam in beams.iter_mut() {
            beam.change_location();
            if contains(&grid, &beam.location) {
                energized_tiles.insert(beam.location);
                let tile = &grid[beam.location.0 as usize][beam.location.1 as usize];
                for generated_beam in beam.change_direction(tile) {
                    if !beam_history.contains(&generated_beam) {
                        new_beams.push(generated_beam.clone());
                        beam_history.insert(generated_beam);
                    }
                }
            }
        }

        beams = new_beams.drain(..).collect();
    }

    return energized_tiles.len();
}

fn main() {
    let grid: Grid = std::io::stdin()
        .lines()
        .map(|l| l.unwrap())
        .map(|row| row.chars().map(|c| c.into()).collect())
        .collect();

    let height = grid.len() as isize;
    let width = grid[0].len() as isize;
    let left_flank = (0..height)
        .map(|initial_i| Beam {
            location: (initial_i as isize, -1),
            direction: Direction::RIGHT,
        })
        .collect::<Vec<_>>();
    let right_flank = (0..height)
        .map(|initial_i| Beam {
            location: (initial_i as isize, width),
            direction: Direction::LEFT,
        })
        .collect();
    let top_flank = (0..width)
        .map(|initial_j| Beam {
            location: (-1, initial_j as isize),
            direction: Direction::DOWN,
        })
        .collect();
    let bottom_flank = (0..width)
        .map(|initial_j| Beam {
            location: (height, initial_j as isize),
            direction: Direction::UP,
        })
        .collect();

    let sol = [left_flank, right_flank, top_flank, bottom_flank]
        .concat()
        .into_iter()
        .map(|initial_beam| energize_tiles(&grid, initial_beam))
        .max()
        .unwrap();
    println!("{sol}");
}
