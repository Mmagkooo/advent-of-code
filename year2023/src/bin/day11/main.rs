use std::collections::{HashMap, HashSet};
use std::hash::Hash;

/// (i, j)
type Address = (usize, usize);

fn set_to_index_map<T: Ord + Copy + Hash>(s: &HashSet<T>) -> HashMap<T, usize> {
    let mut v: Vec<T> = s.iter().map(|el| *el).collect();
    v.sort_unstable();

    HashMap::from_iter(v.iter().enumerate().map(|(i, el)| (*el, i)))
}

fn get_empty_between(v1: usize, v2: usize, index_map: &HashMap<usize, usize>) -> usize {
    let index1 = index_map.get(&v1).unwrap();
    let index2 = index_map.get(&v2).unwrap();
    v1.abs_diff(v2) - index1.abs_diff(*index2)
}

fn get_distance(p1: &Address, p2: &Address) -> usize {
    p1.0.abs_diff(p2.0) + p1.1.abs_diff(p2.1)
}

fn main() {
    let var_name = "EXPANSION_FACTOR";
    let expansion_factor: usize = std::env::var(var_name).expect(var_name).parse().unwrap();

    let mut populated_rows: HashSet<usize> = HashSet::new();
    let mut populated_columns: HashSet<usize> = HashSet::new();
    let mut points: HashSet<Address> = HashSet::new();
    std::io::stdin()
        .lines()
        .map(|l| l.unwrap())
        .enumerate()
        .for_each(|(i, l)| {
            l.chars()
                .enumerate()
                .filter(|(_, c)| c == &'#')
                .for_each(|(j, _)| {
                    points.insert((i, j));
                    populated_rows.insert(i);
                    populated_columns.insert(j);
                })
        });

    let row_index_map = set_to_index_map(&populated_rows);
    let column_index_map = set_to_index_map(&populated_columns);

    let mut sol = 0;
    let points_vec: Vec<Address> = points.iter().map(|p| *p).collect();
    for (point_index, point1) in points_vec.iter().enumerate() {
        for point2 in &points_vec[point_index + 1..] {
            let dist = get_distance(point1, point2);
            let empty_rows_between = get_empty_between(point1.0, point2.0, &row_index_map);
            let empty_columns_between = get_empty_between(point1.1, point2.1, &column_index_map);
            sol += dist + (expansion_factor - 1) * (empty_rows_between + empty_columns_between)
        }
    }

    println!("{sol}");
}
