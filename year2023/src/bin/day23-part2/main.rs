use std::collections::{HashMap, VecDeque};

type Location = (isize, isize);
const START_LOCATION: Location = (0_isize, 1_isize);

#[derive(Clone, PartialEq, Eq, Hash)]
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
            Direction::LEFT => (0, -1),
            Direction::RIGHT => (0, 1),
        }
    }

    const ALL: [Self; 4] = [Self::UP, Self::DOWN, Self::RIGHT, Self::LEFT];
}

const WALL: char = '#';

type Distances = HashMap<Location, usize>;
type Graph = HashMap<Location, Distances>;

fn calculate_next_locations(
    current_location: &Location,
    prev_location: &Location,
    input_chars: &Vec<Vec<char>>,
) -> Vec<Location> {
    let height = input_chars.len() as isize;
    let width = input_chars[0].len() as isize;

    let mut next_locations = vec![];
    for direction in Direction::ALL {
        let (di, dj) = direction.value();
        let next_location = (current_location.0 + di, current_location.1 + dj);
        if &next_location == prev_location {
            continue;
        }

        if next_location.0 < 0
            || next_location.0 >= height
            || next_location.1 < 0
            || next_location.1 >= width
        {
            continue;
        }

        let next_char = input_chars[next_location.0 as usize][next_location.1 as usize];
        if next_char == WALL {
            continue;
        }

        next_locations.push(next_location);
    }

    next_locations
}

fn to_graph(input_chars: &Vec<Vec<char>>, end_location: Location) -> Graph {
    let mut graph: Graph = Default::default();

    let mut queue: Vec<_> = vec![(START_LOCATION, (1_isize, 1_isize))];
    while !queue.is_empty() {
        let (original_location, mut current_location) = queue.pop().unwrap();

        if end_location == original_location {
            continue;
        }

        // find next crossroads
        let mut distance = 1;
        let mut prev_location = original_location;
        loop {
            let next_locations =
                calculate_next_locations(&current_location, &prev_location, input_chars);

            match next_locations.len() {
                0 => panic!("Should never be here"),
                1 => {
                    distance += 1;
                    let next_location = next_locations[0];

                    let neighbors_to_original = graph.entry(original_location).or_default();

                    if let Some(old_distance) = neighbors_to_original.get(&next_location) {
                        if distance > *old_distance {
                            neighbors_to_original.insert(next_location, distance);

                            graph
                                .get_mut(&next_location)
                                .unwrap()
                                .insert(original_location, distance);
                        }
                        break;
                    }

                    if next_location == end_location {
                        graph
                            .entry(original_location)
                            .or_default()
                            .insert(next_location, distance);
                        break;
                    }

                    prev_location = current_location;
                    current_location = next_location;
                }
                2 | 3 => {
                    for next_location in next_locations {
                        let neighbors_to_original = graph.entry(original_location).or_default();
                        neighbors_to_original.insert(current_location, distance);

                        graph
                            .entry(current_location)
                            .or_default()
                            .insert(original_location, distance);

                        queue.push((current_location, next_location));
                    }
                    break;
                }
                _ => panic!("Invalid next locations: {next_locations:?}"),
            }
        }
    }

    return graph;
}

type LocationId = u64;
type LocationIdMap = HashMap<Location, LocationId>;
type LocationHistory = LocationId;

fn find_max_distance(
    graph: &Graph,
    end_location: Location,
    location_to_id: &LocationIdMap,
) -> usize {
    type State = (Location, LocationHistory);
    let mut state_history: HashMap<State, usize> = Default::default();

    let mut max_distance = 0;

    let mut queue: VecDeque<(State, usize)> = Default::default();
    let initial_location_history = 0;
    let initial_state = (START_LOCATION, initial_location_history);
    let initial_distance = 0;
    queue.push_back((initial_state, initial_distance));
    while !queue.is_empty() {
        let (state, distance) = queue.pop_front().unwrap();
        let (location, mut location_history) = state;

        // check if end
        if location == end_location {
            if distance > max_distance {
                max_distance = distance;
            }
            continue;
        }

        // check if been
        let id = location_to_id.get(&location).unwrap();
        if (1 << id) & location_history != 0 {
            continue;
        }
        location_history |= 1 << id;

        // check if already been at this location with this location history
        if let Some(old_distance) = state_history.get(&state) {
            if old_distance >= &distance {
                continue;
            }
        }
        state_history.insert(state, distance);

        for (neighbor, neighbor_distance) in graph.get(&location).unwrap() {
            let next_state = (*neighbor, location_history);
            queue.push_back((next_state, distance + neighbor_distance));
        }
    }

    return max_distance;
}

fn add_id_if_missing(location: &Location, id_map: &mut LocationIdMap) {
    if !id_map.contains_key(&location) {
        id_map.insert(*location, id_map.len() as LocationId);
    }
}

fn generate_id_map(locations: &Graph) -> LocationIdMap {
    let mut id_map: LocationIdMap = Default::default();
    for (location, neighbors) in locations {
        add_id_if_missing(location, &mut id_map);
        for neighbor in neighbors.keys() {
            add_id_if_missing(neighbor, &mut id_map);
        }
    }
    return id_map;
}

fn main() {
    let input_chars: Vec<Vec<char>> = std::io::stdin()
        .lines()
        .map(|l| l.unwrap().chars().collect())
        .collect();

    let height = input_chars.len() as isize;
    let width = input_chars[0].len() as isize;
    let end_location = (height - 1, width - 2);

    let graph = to_graph(&input_chars, end_location);

    // since history will be in the form of a 64-bit number
    assert!(graph.len() < 64);

    let mut locations: Vec<Location> = graph.keys().map(|l| *l).collect();
    locations.sort(); // to ensure more intuitive ids if debugging
    let location_to_id = generate_id_map(&graph);

    let sol = find_max_distance(&graph, end_location, &location_to_id);
    println!("{}", sol);
}
