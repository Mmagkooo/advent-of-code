use std::{collections::HashMap, io::stdin};

/// Returns `true` if `c` is gear (i.e. '*')
fn confirming_symbol(c: &char) -> bool {
    c == &'*'
}

/// (i, j) - zero-indexed
type Address = (isize, isize);

/// If area around lines[i][start_j:end_j + 1] contains the confirming_symbol, its address is returned
fn find_gear_address(
    lines: &Vec<Vec<char>>,
    i: isize,
    start_j: isize,
    end_j: isize,
) -> Option<Address> {
    let width = lines[0].len() as isize;
    let height = lines.len() as isize;

    for ni in (i - 1)..=(i + 1) {
        if ni < 0 || ni >= height {
            continue;
        }

        for nj in (start_j - 1)..=(end_j + 1) {
            if nj < 0 || nj >= width {
                continue;
            }

            if confirming_symbol(&lines[ni as usize][nj as usize]) {
                return Some((ni, nj));
            }
        }
    }

    return None;
}

fn main() {
    let lines: Vec<Vec<char>> = stdin()
        .lines()
        .map(|l| l.unwrap().chars().collect())
        .collect();

    let width = lines[0].len();

    let mut address_to_values: HashMap<Address, Vec<i32>> = HashMap::new();

    for (i, line) in lines.iter().enumerate() {
        let mut j: usize = 0;
        while j < width {
            if line[j].is_numeric() {
                // iterate over sequence of chars and check if its neighbors are safe
                let mut current_value = 0;
                let start_j = j as isize;
                while j < width && line[j as usize].is_numeric() {
                    current_value = current_value * 10 + (line[j as usize] as i32 - '0' as i32);
                    j += 1;
                }

                if let Some(gear_address) =
                    find_gear_address(&lines, i as isize, start_j, (j - 1) as isize)
                {
                    match address_to_values.get_mut(&gear_address) {
                        Some(values) => values.push(current_value),
                        None => {
                            address_to_values.insert(gear_address, vec![current_value]);
                        }
                    }
                }
            }

            j += 1;
        }
    }

    let sol: i32 = address_to_values
        .iter()
        .map(|(_, values)| values)
        .filter(|values| values.len() == 2)
        .map(|values| values[0] * values[1])
        .sum();

    println!("{sol}");
}
