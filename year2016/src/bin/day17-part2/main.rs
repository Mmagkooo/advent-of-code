use std::collections::VecDeque;

type Point = (i32, i32);

struct Config {
    position: Point,
    path: String,
}

const ROOM_SIZE: i32 = 4;

fn is_legal(position: &Point) -> bool {
    position.0 >= 0 && position.0 < ROOM_SIZE && position.1 >= 0 && position.1 < ROOM_SIZE
}

const DIRS: [(char, Point); 4] = [('U', (-1, 0)), ('D', (1, 0)), ('L', (0, -1)), ('R', (0, 1))];

fn main() {
    let argv: Vec<String> = std::env::args().collect();
    let passcode = argv[1].clone();

    let goal = (ROOM_SIZE - 1, ROOM_SIZE - 1);

    let mut longest_path = String::new();

    // BFS
    let mut queue: VecDeque<Config> = VecDeque::from([Config {
        position: (0, 0),
        path: passcode.clone(),
    }]);
    while !queue.is_empty() {
        let Config { position, path } = queue.pop_front().unwrap();
        if !is_legal(&position) {
            continue;
        }

        if position == goal {
            if path.len() > longest_path.len() {
                longest_path = path.clone();
            }
            continue;
        }

        let digest = format!("{:x}", md5::compute(&path));

        for (c, (dir_c, dir_move)) in digest[..4].chars().zip(DIRS) {
            match c {
                // door closed
                '0'..='9' | 'a' => (),
                // door open
                'b'..='f' => {
                    let next_position = (position.0 + dir_move.0, position.1 + dir_move.1);
                    let mut next_path = path.clone();
                    next_path.push(dir_c);
                    queue.push_back(Config {
                        position: next_position,
                        path: next_path,
                    })
                }
                inv => panic!("Invalid char: {inv}"),
            }
        }
    }

    println!("{}", longest_path.len() - passcode.len());
}
