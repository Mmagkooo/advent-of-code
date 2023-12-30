use std::collections::HashSet;

/// (i, j)
type Location = (isize, isize);
type Grid = Vec<Vec<char>>;

const START: char = 'S';
const ROCK: char = '#';
const EMPTY: char = '.';
const MAX_DISTANCE: isize = 26501365;

const DIRECTIONS: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, 1), (0, -1)];

fn char_at(i: isize, j: isize, grid: &Grid) -> char {
    let height = grid.len() as isize;
    let width = grid[0].len() as isize;

    let real_i = ((i % height) + height) % height;
    let real_j = ((j % height) + width) % width;

    grid[real_i as usize][real_j as usize]
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

fn get_parity_counts(
    top_left_location: &Location,
    start_location: &Location,
    grid: &Grid,
) -> Vec<isize> {
    let mut parity_counter: Vec<_> = vec![0, 0];
    let height = grid.len() as isize;
    let width = grid[0].len() as isize;

    let mut seen: HashSet<Location> = HashSet::new();

    // try from all four corners
    let mut queue = vec![];
    for corner_delta in [
        (0, 0),
        (height - 1, 0),
        (0, width - 1),
        (height - 1, width - 1),
    ] {
        let corner_location = (
            top_left_location.0 + corner_delta.0,
            top_left_location.1 + corner_delta.1,
        );
        assert_eq!(char_at(corner_location.0, corner_location.1, &grid), EMPTY);
        queue.push(corner_delta);
    }

    while !queue.is_empty() {
        let (di, dj) = queue.pop().unwrap();

        if di < 0 || di >= height || dj < 0 || dj >= width {
            continue;
        }

        let current_location = (top_left_location.0 + di, top_left_location.1 + dj);
        if seen.contains(&current_location) {
            continue;
        }
        seen.insert(current_location.clone());

        let current_char = char_at(current_location.0, current_location.1, grid);
        if current_char == ROCK {
            continue;
        }

        let distance = location_distance(&current_location, &start_location);
        if distance > MAX_DISTANCE as usize {
            continue;
        }
        let distance_parity = distance % 2;
        parity_counter[distance_parity] += 1;

        for (ni, nj) in DIRECTIONS {
            let next_deltas = (di + ni, dj + nj);
            queue.push(next_deltas);
        }
    }

    assert_eq!(parity_counter.len(), 2);
    parity_counter
}

fn assert_main_axes_empty(start_location: &Location, grid: &Grid) {
    let height = grid.len();
    let width = grid[0].len();
    for i in 0..height {
        assert_ne!(grid[i][start_location.1 as usize], ROCK);
    }

    for j in 0..width {
        assert_ne!(grid[start_location.0 as usize][j], ROCK);
    }
}

fn location_distance(loc_a: &Location, loc_b: &Location) -> usize {
    loc_a.0.abs_diff(loc_b.0) + loc_a.1.abs_diff(loc_b.1)
}

fn main() {
    let grid: Grid = std::io::stdin()
        .lines()
        .map(|l| l.unwrap())
        .map(|l| l.chars().collect())
        .collect();
    let height = grid.len() as isize;
    let width = grid[0].len() as isize;

    let start_location = find_start_location(&grid);

    // this program only works under certain assumptions:
    assert_eq!(MAX_DISTANCE % height, start_location.0);
    assert_eq!(MAX_DISTANCE % width, start_location.1);
    assert_main_axes_empty(&start_location, &grid);
    assert_eq!(height, width);
    assert_eq!(height % 2, 1);

    let grids_up = MAX_DISTANCE / height; // how many grids it stretches vertically
    let grids_side = MAX_DISTANCE / width; // how many grids it stretches horizontally

    let mut sol = 0;

    let base_parity_count = get_parity_counts(&(0, 0), &start_location, &grid);
    let target_parity = MAX_DISTANCE as usize % 2;

    // top
    let top_i = start_location.0 - MAX_DISTANCE;
    // bottom indicates which part of the grid is considered - the small right triangle
    let small_bottom_right =
        get_parity_counts(&(top_i, -width), &start_location, &grid)[target_parity];
    // considers everything except the top left triangle
    let big_bottom_right =
        get_parity_counts(&(top_i + height, -width), &start_location, &grid)[target_parity];

    let small_bottom_left =
        get_parity_counts(&(top_i, width), &start_location, &grid)[target_parity];
    let big_bottom_left =
        get_parity_counts(&(top_i + height, width), &start_location, &grid)[target_parity];

    let central_top = get_parity_counts(&(top_i, 0), &start_location, &grid)[target_parity];
    sol += small_bottom_right + central_top + small_bottom_left;

    // middle row
    let leftmost_j = -grids_side * width;
    let leftmost = get_parity_counts(&(0, leftmost_j), &start_location, &grid)[target_parity];
    sol += leftmost;

    let rightmost_j = grids_side * width;
    let righmost = get_parity_counts(&(0, rightmost_j), &start_location, &grid)[target_parity];
    sol += righmost;

    // counting whole grids between (not including) leftmost and rightmost, excluding centre
    let middle_row_width = (grids_side - 1) * 2 + 1;
    let start_parity = (target_parity + grids_side as usize - 1) % 2;
    sol += (middle_row_width / 2 + 1) * base_parity_count[start_parity];
    sol += (middle_row_width / 2) * base_parity_count[1 - start_parity];

    // bottom
    let bottom_i = grids_up * height;
    let small_top_right =
        get_parity_counts(&(bottom_i, -width), &start_location, &grid)[target_parity];
    let big_top_right =
        get_parity_counts(&(bottom_i - height, -width), &start_location, &grid)[target_parity];

    let small_top_left =
        get_parity_counts(&(bottom_i, width), &start_location, &grid)[target_parity];
    let big_top_left =
        get_parity_counts(&(bottom_i - height, width), &start_location, &grid)[target_parity];

    let central_bottom = get_parity_counts(&(bottom_i, 0), &start_location, &grid)[target_parity];
    sol += small_top_right + central_bottom + small_top_left;

    // not including the middle row - already added
    // not including the top row and the bottom row - they are not fully in the distance range
    for row_i in 1..=grids_up - 1 {
        let row_width = (grids_side - row_i - 1) * 2 + 1;

        // using factor of 2 to account for rows above and below the middle
        sol += (row_width / 2 + 1) * base_parity_count[start_parity] * 2;
        sol += (row_width / 2) * base_parity_count[1 - start_parity] * 2;

        // partial grids - above
        sol += small_bottom_right;
        sol += big_bottom_right;

        sol += small_bottom_left;
        sol += big_bottom_left;

        // partial grids - above
        sol += small_top_right;
        sol += big_top_right;

        sol += small_top_left;
        sol += big_top_left;
    }

    println!("{sol}");
}
