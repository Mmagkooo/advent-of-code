use std::collections::HashSet;

#[derive(Debug, Clone)]
struct Elf {
    initial_index: usize,
    // number_of_presents: usize, - not really important
}

fn get_across(i: usize, total: usize) -> usize {
    ((total - i) / 2 + 2 * i) % total
}

fn main() {
    let argv: Vec<String> = std::env::args().collect();
    let initial_elves_number: usize = argv[1].parse().unwrap();

    let mut elves = Vec::<Elf>::new();
    for i in 0..initial_elves_number {
        elves.push(Elf {
            initial_index: i + 1,
        });
    }

    // - n is the number of elves
    // Each loop iteration discards half of elves, so there are O(log(n)) iterations.
    // In every loop iteration, we iterate over O(n) elves.
    // This gives a total time complexity of O(n*log(n))
    while elves.len() > 1 {
        let current_elves_number = elves.len();
        let mut discardables = HashSet::new();
        let first_discarded = get_across(0, current_elves_number);
        for i in 0..first_discarded {
            let across = get_across(i, current_elves_number);
            discardables.insert(across);
        }

        let first_index_next_round = if discardables.contains(&(first_discarded + 1)) {
            first_discarded + 2
        } else {
            first_discarded + 1
        };
        
        let mut new_elves = vec![];
        for i in 0..current_elves_number {
            let actual_index = (first_index_next_round + i) % current_elves_number;
            if !discardables.contains(&actual_index) {
                new_elves.push(elves[actual_index].clone());
            }
        }

        elves = new_elves;
    }

    assert_eq!(elves.len(), 1);
    println!("{}", elves[0].initial_index);
}
