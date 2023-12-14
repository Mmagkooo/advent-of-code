use std::collections::HashMap;

fn extract_blueprint(s: &str) -> Vec<char> {
    s.chars().collect()
}

fn extract_spec(s: &str) -> Vec<u8> {
    s.split(",").map(|w| w.parse().unwrap()).collect()
}

fn terminable(c: &char) -> bool {
    c == &'.' || c == &'?'
}

fn count_possible_arrangements(mut blueprint: &[char], amounts: &[u8], memo: &mut HashMap<(Vec<char>, Vec<u8>), usize>) -> usize {
    let memorable = (blueprint.to_vec(), amounts.to_vec());
    match memo.get(&memorable) {
        Some(val) => return val.clone(),
        None => ()
    };

    if amounts.is_empty() {
        return blueprint.iter().all(terminable).into();
    }

    // strip leading '.'
    let mut start_index = 0;
    for c in blueprint {
        if c == &'.' {
            start_index += 1
        } else {
            break;
        }
    }

    if start_index == blueprint.len() {
        return amounts.is_empty().into();
    } else {
        blueprint = &blueprint[start_index..];
    }

    let mut acc_bath_amount: i32 = 0;
    let mut acc_wildcard_amount: i32 = 0;
    for (blueprint_i, c) in blueprint.iter().enumerate() {
        if c == &'#' {
            acc_bath_amount += 1;
        } else if c == &'?' {
            acc_wildcard_amount += 1;
        } else {
            // if all collected chars are ?, we might have been wrong about it being a bath block
            return if acc_bath_amount == 0 {
                let next_blueprint = blueprint
                    .get(blueprint_i + 1..blueprint.len())
                    .unwrap_or(&[]);
                count_possible_arrangements(next_blueprint, amounts, memo)
            } else {
                0
            };
        }

        if acc_bath_amount > amounts[0] as i32 {
            return 0;
        }

        let acc_amount = acc_bath_amount + acc_wildcard_amount;

        if acc_amount == amounts[0] as i32 {
            // #. + rest; the 2 refers to looking 2 places ahead
            let next_blueprint_2 = blueprint
                .get(blueprint_i + 2..blueprint.len())
                .unwrap_or(&[]);

            let next_amounts = amounts.get(1..amounts.len()).unwrap_or(&[]);
            let ret = match blueprint.get(blueprint_i + 1) {
                None => {
                    // if this was the last checked amount, we've successfully exhausted the blueprint
                    next_amounts.is_empty().into()
                }
                Some('.') => {
                    count_possible_arrangements(next_blueprint_2, next_amounts, memo)
                        + if acc_bath_amount == 0 {
                            count_possible_arrangements(next_blueprint_2, amounts, memo)
                        } else {
                            0
                        }
                }
                Some('?') => {
                    // blueprint[0] is also the first char of the potential bath block
                    count_possible_arrangements(next_blueprint_2, next_amounts, memo)
                        + if blueprint[0] == '?' {
                            count_possible_arrangements(&blueprint[1..], amounts, memo)
                        } else {
                            0
                        }
                }
                Some('#') => {
                    if blueprint[0] == '?' {
                        count_possible_arrangements(&blueprint[1..], amounts, memo)
                    } else {
                        0
                    }
                }
                Some(other) => panic!("Invalid char: {other}"),
            };
            memo.insert(memorable.clone(), ret);
            return ret;
        }
    }

    return 0;
}

fn unfold(s: &str, sep: &str, factor: usize) -> String {
    let mut unfolded: Vec<&str> = vec![];
    (0..factor).for_each(|_| unfolded.push(s));
    unfolded.join(sep)
}

fn main() {
    // part1: factor=1, part2: factor=5
    let unfolding_factor = std::env::var("UNFOLDING_FACTOR").unwrap().parse().unwrap();

    let sol: usize = std::io::stdin()
        .lines()
        .map(|l| l.unwrap())
        .map(|l| l.split(" ").map(|w| w.to_string()).collect::<Vec<String>>())
        .map(|split_content| {
            assert_eq!(split_content.len(), 2);
            (
                extract_blueprint(&unfold(&split_content[0], "?", unfolding_factor)),
                extract_spec(&unfold(&split_content[1], ",", unfolding_factor)),
            )
        })
        .map(|(blueprint, amounts)| {
            count_possible_arrangements(&blueprint, &amounts, &mut HashMap::new())
        })
        .sum();
    println!("{sol}");
}
