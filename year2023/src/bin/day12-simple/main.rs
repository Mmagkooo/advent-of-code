use std::collections::HashSet;

fn to_amounts(candidate: &[char]) -> Vec<u8> {
    let mut amounts = vec![];
    let mut accumulated = 0_u8;
    for c in candidate {
        if c == &'#' {
            accumulated += 1;
        } else if accumulated > 0 {
            amounts.push(accumulated);
            accumulated = 0;
        }
    }

    if accumulated > 0 {
        amounts.push(accumulated);
    }

    return amounts;
}

fn extract_blueprint(s: &str) -> Vec<char> {
    s.chars().collect()
}

fn extract_spec(s: &str) -> Vec<u8> {
    s.split(",").map(|w| w.parse().unwrap()).collect()
}

fn generate_possible_arrangements(blueprint: &[char]) -> HashSet<Vec<char>> {
    let mut possible_arrangements: HashSet<_> = HashSet::new();
    let mut stack: Vec<(Vec<char>, usize)> = vec![(vec![], 0)];
    while !stack.is_empty() {
        let (mut so_far, i) = stack.pop().unwrap();
        if i == blueprint.len() {
            possible_arrangements.insert(so_far);
            continue;
        }

        for i in i..blueprint.len() {
            let c = blueprint[i];
            if c == '?' {
                for opt_c in ['.', '#'] {
                    let mut opt_v = so_far.clone();
                    opt_v.push(opt_c);
                    stack.push((opt_v, i + 1));
                }
                break;
            }
            so_far.push(c);
        }

        if so_far.len() == blueprint.len() {
            possible_arrangements.insert(so_far);
        }
    }

    return possible_arrangements;
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
            let possible_arrangements = generate_possible_arrangements(&blueprint);
            possible_arrangements
                .iter()
                .filter(|arrangement| to_amounts(arrangement) == amounts)
                .count()
        })
        .sum();
    println!("{sol}");
}
