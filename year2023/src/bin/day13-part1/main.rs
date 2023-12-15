type Matrix = Vec<Vec<char>>;

fn reflect_horizontally(matrix: &Matrix) -> Option<usize> {
    let height = matrix.len();
    for i in 1..height {
        let mut upper_i: isize = i as isize - 1;
        let mut lower_i = i;

        let mut found = true;
        while upper_i >= 0 && lower_i < height {
            if matrix[upper_i as usize] != matrix[lower_i] {
                found = false;
            }

            upper_i -= 1;
            lower_i += 1;
        }

        if found {
            return Some(i);
        }
    }

    return None;
}

fn transpose(matrix: Matrix) -> Matrix {
    let mut new_matrix = Matrix::new();
    for j in 0..matrix[0].len() {
        let mut new_row = Vec::new();
        for i in 0..matrix.len() {
            new_row.push(matrix[i][j])
        }
        new_matrix.push(new_row);
    }

    return new_matrix;
}

fn reflect_vertically(matrix: &Matrix) -> Option<usize> {
    let transposed = transpose(matrix.clone());
    reflect_horizontally(&transposed)
}

fn main() {
    let mut lines = std::io::stdin()
        .lines()
        .map(|l| l.unwrap().chars().collect::<Vec<char>>());
    let mut matrix = Matrix::new();

    let mut sol = 0;
    loop {
        match lines.next() {
            Some(line) if !line.is_empty() => {
                matrix.push(line);
            }
            other => {
                if let Some(rows_above) = reflect_horizontally(&matrix) {
                    sol += 100 * rows_above;
                } else if let Some(cols_left) = reflect_vertically(&matrix) {
                    sol += cols_left
                } else {
                    panic!("No reflection!");
                }

                // continue with processing lines or finish?
                if other.is_none() {
                    break;
                } else {
                    matrix = Matrix::new();
                }
            }
        }
    }

    println!("{sol}");
}
