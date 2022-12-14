/**
 * Can be rewritten with the following in mind:
 * - parsing can be done with the JSON crate
 * - instead of using struct Element, perhaps enum Element could be used
 */

use std::io::{self, BufRead};

#[derive(Clone, Debug)]
struct Element {
    number: Option<i32>,
    vector: Vec<Element>,
}

fn parse_line(line: &str) -> Vec<Element> {
    let tokens: Vec<char> = line.chars().collect();
    let (element, token_index) = parse_element(&tokens, 0);
    assert_eq!(token_index, tokens.len());
    assert!(element.number.is_none());
    return element.vector;
}

fn parse_number(tokens: &Vec<char>, mut index: usize) -> (Option<i32>, usize) {
    let mut number = 0;
    while tokens[index].is_numeric() {
        let next_digit = tokens[index] as i32 - '0' as i32;
        number += 10 * number + next_digit;
        index += 1;
    }
    return (Some(number), index);
}

fn parse_vector(tokens: &Vec<char>, mut index: usize) -> (Vec<Element>, usize) {
    assert_eq!(tokens[index], '[');
    index += 1;

    let mut vector: Vec<Element> = vec![];
    while tokens[index] != ']' {
        if tokens[index] == ',' {
            index += 1;
        } else {
            let (new_element, new_index) = parse_element(tokens, index);
            index = new_index;
            vector.push(new_element);
        }
    }

    return (vector, index + 1); // move one after ']'
}

fn parse_element(tokens: &Vec<char>, index: usize) -> (Element, usize) {
    let mut ret_element = Element {
        number: None,
        vector: vec![],
    };

    if tokens[index].is_numeric() {
        let (number, new_index) = parse_number(tokens, index);
        ret_element.number = number;
        return (ret_element, new_index);
    }

    let (vector, new_index) = parse_vector(tokens, index);
    ret_element.vector = vector;
    return (ret_element, new_index);
}

fn compare(left: &Vec<Element>, right: &Vec<Element>) -> i32 {
    for (left_el, right_el) in left.iter().zip(right) {
        let left_el_is_int = left_el.number.is_some();
        let right_el_is_int = right_el.number.is_some();

        if left_el_is_int && right_el_is_int {
            if left_el.number != right_el.number {
                return left_el.number.unwrap() - right_el.number.unwrap();
            }
        } else if !left_el_is_int && !right_el_is_int {
            let list_comparison = compare(&left_el.vector, &right_el.vector);
            if list_comparison != 0 {
                return list_comparison;
            }
        } else {
            let list_comparison = if left_el_is_int {
                compare(&vec![left_el.clone()], &right_el.vector)
            } else {
                compare(&left_el.vector, &vec![right_el.clone()])
            };
            if list_comparison != 0 {
                return list_comparison;
            }
        }
    }

    return left.len() as i32 - right.len() as i32;
}

fn main() {
    let lines = io::stdin().lock().lines();
    let lines: Vec<String> = lines
        .map(|line| line.expect("Could not read line"))
        .collect();

    let mut sol = 0;
    for line_i in (0..lines.len()).step_by(3) {
        let left = parse_line(&lines[line_i]);
        let right = parse_line(&lines[line_i + 1]);

        if compare(&left, &right) < 0 {
            let i = line_i / 3 + 1;
            sol += i;
        }
    }

    println!("{sol}");
}
