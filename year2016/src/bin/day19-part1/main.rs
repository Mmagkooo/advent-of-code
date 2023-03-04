fn find_next_index(arr: &Vec<i32>, mut index: i32, arr_size: usize) -> i32 {
    index %= arr_size as i32;
    while arr[index as usize] == 0 {
        index = (index + 1) % arr_size as i32;
    }
    return index;
}

fn main() {
    let argv: Vec<String> = std::env::args().collect();
    let elves_number: usize = argv[1].parse().unwrap();

    // initially every elf has 1 present
    let mut presents = vec![1; elves_number];

    let mut elf_index = 0;
    loop {
        elf_index = find_next_index(&presents, elf_index, elves_number);
        let neighbor_index = find_next_index(&presents, elf_index + 1, elves_number);

        if elf_index == neighbor_index {
            break;
        }

        presents[elf_index as usize] += presents[neighbor_index as usize];
        presents[neighbor_index as usize] = 0;

        elf_index = neighbor_index;
    }

    // elf_index got increased but since we're 0-indexing, it's ok
    println!("{}", elf_index + 1);
}
