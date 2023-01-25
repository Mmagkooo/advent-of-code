use std::io::{stdin, BufRead};

fn parse(line: &String, prefix: &str, delimiter: &str) -> (usize, usize) {
    let parts: Vec<usize> = line
        .strip_prefix(prefix)
        .unwrap()
        .split(delimiter)
        .map(|s| s.parse().unwrap())
        .collect();
    assert_eq!(parts.len(), 2);
    return (parts[0], parts[1]);
}

fn parse_rect(line: &String) -> (usize, usize) {
    parse(line, "rect ", "x")
}

fn parse_rotate_row(line: &String) -> (usize, usize) {
    parse(line, "rotate row y=", " by ")
}

fn parse_rotate_col(line: &String) -> (usize, usize) {
    parse(line, "rotate column x=", " by ")
}

const FIELD_HEIGHT: usize = 6;
const FIELD_WIDTH: usize = 50;
type Field = [[bool; FIELD_WIDTH]; FIELD_HEIGHT];

fn set_rect(field: &mut Field, width: usize, height: usize) {
    for i in 0..height {
        for j in 0..width {
            field[i][j] = true;
        }
    }
}

fn rotate_row(field: &mut Field, row: usize, step: usize) {
    let mut new = [false; FIELD_WIDTH];
    for col in 0..FIELD_WIDTH {
        new[(col + step) % FIELD_WIDTH] = field[row][col];
    }
    for col in 0..FIELD_WIDTH {
        field[row][col] = new[col];
    }
}

fn rotate_col(field: &mut Field, col: usize, step: usize) {
    let mut new = [false; FIELD_HEIGHT];
    for row in 0..FIELD_HEIGHT {
        new[(row + step) % FIELD_HEIGHT] = field[row][col];
    }
    for row in 0..FIELD_HEIGHT {
        field[row][col] = new[row];
    }
}

fn print_field(field: &Field) {
    for row in 0..field.len() {
        for col in 0..field[row].len() {
            let c = match field[row][col] {
                true => '#',
                false => '.',
            };
            print!("{c}");
        }
        println!();
    }
}

fn main() {
    let lines: Vec<String> = stdin()
        .lock()
        .lines()
        .map(|l| l.expect("Could not read line"))
        .collect();

    let mut field: Field = [[false; FIELD_WIDTH]; FIELD_HEIGHT];
    for l in lines {
        if l.starts_with("rect ") {
            let (height, width) = parse_rect(&l);
            set_rect(&mut field, height, width);
        } else if l.starts_with("rotate row ") {
            let (row, by) = parse_rotate_row(&l);
            rotate_row(&mut field, row, by);
        } else if l.starts_with("rotate column ") {
            let (col, by) = parse_rotate_col(&l);
            rotate_col(&mut field, col, by);
        } else {
            panic!("Invalid command: {l}");
        }
    }

    print_field(&field);
}
