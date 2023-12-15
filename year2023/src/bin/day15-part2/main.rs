fn hash_algorithm(word: &str) -> u32 {
    word.chars().fold(0, |acc, c| ((acc + c as u32) * 17) % 256)
}

#[derive(Clone, Debug)]
struct Lens {
    label: String,
    focal_length: i32,
}

fn main() {
    let line = std::io::stdin().lines().next().unwrap().unwrap();
    let mut boxes: Vec<Vec<Lens>> = vec![vec![]; 256];
    line.split(",").for_each(|command| {
        let label: String = command
            .chars()
            .take_while(|c| c != &'-' && c != &'=')
            .collect();
        let box_i = hash_algorithm(&label) as usize;
        let current_box = &mut boxes[box_i];

        let maybe_position = current_box.iter().position(|lens| lens.label == label);

        if command.contains('-') {
            match maybe_position {
                Some(old_position) => {
                    current_box.remove(old_position);
                }
                None => (),
            }
        } else if command.contains('=') {
            let focal_length = command.chars().last().unwrap() as i32 - '0' as i32;
            match maybe_position {
                Some(old_position) => current_box[old_position].focal_length = focal_length,
                None => current_box.push(Lens {
                    label: label.into(),
                    focal_length,
                }),
            }
        } else {
            panic!("Invalid command: {command}")
        }
    });

    let mut sol = 0;
    for (box_i, b) in boxes.iter().enumerate() {
        for (lens_i, lens) in b.iter().enumerate() {
            sol += (box_i as i32 + 1) * (lens_i as i32 + 1) * lens.focal_length;
        }
    }
    println!("{sol}");
}
