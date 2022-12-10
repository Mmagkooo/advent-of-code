
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct Directory {
    direct_size: i32,
    children: Vec<String>,
}

// include space so that trim doesn't have to be applied
const CD_PREFIX: &str = "$ cd ";

// assuming no filename will contain this string
const SEP: &str = ", ";

fn get_size(dir_name: &str, disk: &mut HashMap<String, Directory>, sol: &mut i32, limit: i32) -> i32 {
    let dir = disk.get(dir_name).unwrap();
    let children = dir.children.clone();

    let mut size = dir.direct_size;

    for child_name in children {
        let full_child_name = [dir_name, child_name.as_str()].join(SEP);
        size += get_size(full_child_name.as_str(), disk, sol, limit);
    }

    if size >= limit {
        println!("good candidate: {dir_name}, size: {size}");
        if size <= *sol {
            println!("updating sol");
            *sol = size;
        }
    }
    return size;
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];
    let file = File::open(filename).expect("Could not open file");
    let reader = BufReader::new(file);

    let lines: Vec<String> = reader
        .lines()
        .map(|line| line.expect("Could not read line").trim().to_string())
        .collect();

    let mut disk: HashMap<String, Directory> = HashMap::new();
    let mut full_dir_name: Vec<String> = vec![];

    for line in lines {
        if line.starts_with(CD_PREFIX) {
            let dir_name = line.strip_prefix(CD_PREFIX).unwrap();
            if dir_name == ".." {
                full_dir_name.pop();
            } else {
                full_dir_name.push(dir_name.to_string());
            }
            let full_dir_name = full_dir_name.join(SEP);
            if !disk.contains_key(&full_dir_name) {
                disk.insert(
                    full_dir_name,
                    Directory {
                        direct_size: 0,
                        children: vec![],
                    },
                );
            }
        } else if line.starts_with("$ ls") {
            // will list content of curr dir
        } else {
            // otherwise it's a result of ls
            let parts: Vec<&str> = line.split(" ").collect();
            let curr_dir = disk.get_mut(&full_dir_name.join(SEP)).unwrap();

            let is_dir = parts[0] == "dir";
            if is_dir {
                let child_name = parts[1].to_string();
                curr_dir.children.push(child_name);
            } else {
                let file_size: i32 = parts[0].parse().unwrap();
                curr_dir.direct_size += file_size;
            }
        }
    }

    let needed_unused = 30_000_000;

    // pass in dummy values as sol and limit because we only need to get size of /
    let used_space = get_size("/", &mut disk, &mut 0, 0);
    let unused_space = 70_000_000 - used_space;
    let limit = needed_unused - unused_space;
    println!("Limit (at least needed): {limit}");

    let mut sol = 1_000_000_000; // something big
    get_size("/", &mut disk, &mut sol, limit);
    println!("{sol}");
}
