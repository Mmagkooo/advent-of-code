use std::collections::HashSet;

type Matrix = Vec<Vec<char>>;

fn reflect_horizontally(matrix: &Matrix) -> HashSet<usize> {
    let mut reflection_lines = HashSet::new();
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
            reflection_lines.insert(i);
        }
    }

    return reflection_lines;
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

fn reflect_vertically(matrix: &Matrix) -> HashSet<usize> {
    let transposed = transpose(matrix.clone());
    reflect_horizontally(&transposed)
}

fn clear_smudge(matrix: &Matrix, i: usize, j: usize) -> Matrix {
    let mut new_matrix = matrix.clone();
    new_matrix[i][j] = match new_matrix[i][j] {
        '.' => '#',
        '#' => '.',
        inv => panic!("Invalid char: {inv}"),
    };
    new_matrix
}

fn extract_new_reflection(
    old_reflections: &HashSet<usize>,
    new_reflections: &HashSet<usize>,
) -> usize {
    let (bigger_reflections, smaller_reflections) = if old_reflections.len() > new_reflections.len()
    {
        (old_reflections, new_reflections)
    } else {
        (new_reflections, old_reflections)
    };

    let new_reflection: Vec<_> = bigger_reflections
        .difference(&smaller_reflections)
        .collect();
    assert_eq!(new_reflection.len(), 1);
    *new_reflection[0]
}

fn reflect_with_smudge(
    matrix: &Matrix,
    old_horizontal_reflections: &HashSet<usize>,
    old_vertical_reflections: &HashSet<usize>,
) -> usize {
    for i in 0..matrix.len() {
        for j in 0..matrix[0].len() {
            let new_matrix = clear_smudge(&matrix, i, j);

            let new_horizontal_reflections = reflect_horizontally(&new_matrix);
            if !new_horizontal_reflections.is_empty() && &new_horizontal_reflections != old_horizontal_reflections {
                return 100
                    * extract_new_reflection(
                        old_horizontal_reflections,
                        &new_horizontal_reflections,
                    );
            }

            let new_vertical_reflections = reflect_vertically(&new_matrix);
            if !new_vertical_reflections.is_empty() && &new_vertical_reflections != old_vertical_reflections {
                return extract_new_reflection(old_vertical_reflections, &new_vertical_reflections);
            }
        }
    }

    panic!("Didn't match");
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
                let horizontal_reflections = reflect_horizontally(&matrix);
                let vertical_reflections = reflect_vertically(&matrix);
                sol += reflect_with_smudge(&matrix, &horizontal_reflections, &vertical_reflections);

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
