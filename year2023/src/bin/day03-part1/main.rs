use std::io::stdin;

/// Returns `true` if `c` is any char other than decimal digit or period
fn confirming_symbol(c: &char) -> bool {
    !c.is_numeric() && c != &'.'
}

/// Returns `true` if area around lines[i][start_j:end_j + 1] contains a confirming_symbol
fn neighborhood_confirming(lines: &Vec<Vec<char>>, i: isize, start_j: isize, end_j: isize) -> bool {
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
                return true;
            }
        }
    }

    return false;
}

fn main() {
    let lines: Vec<Vec<char>> = stdin()
        .lines()
        .map(|l| l.unwrap().chars().collect())
        .collect();

    let mut total_value = 0;
    let width = lines[0].len();

    for (i, line) in lines.iter().enumerate() {
        let mut j: usize = 0;
        while j < width {
            if line[j].is_numeric() {
                // iterate over sequence of chars and check if its neighbors are safe
                let mut current_value = 0;
                let start_j = j;
                while j < width && line[j as usize].is_numeric() {
                    current_value = current_value * 10 + (line[j as usize] as i32 - '0' as i32);
                    j += 1;
                }

                if neighborhood_confirming(&lines, i as isize, start_j as isize, (j - 1) as isize) {
                    total_value += current_value;
                }
            }

            j += 1;
        }
    }

    println!("{total_value}");
}
