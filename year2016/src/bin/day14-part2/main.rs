struct HashStorage {
    salt: String,
    storage: Vec<String>,
}

impl HashStorage {
    fn new(salt: String) -> Self {
        Self {
            salt,
            storage: Vec::new(),
        }
    }

    fn get(&mut self, i: usize) -> String {
        let storage_len = self.storage.len();
        for j in storage_len..=i {
            let mut digestable = self.salt.clone();
            digestable.push_str(j.to_string().as_str());
            let mut digest = format!("{:x}", md5::compute(digestable));
            for _ in 0..2016 {
                digest = format!("{:x}", md5::compute(digest));
            }
            self.storage.push(digest);
        }

        return self.storage[i].clone();
    }
}

fn get_first_triple_char(s: &String) -> Option<char> {
    let size = s.len();
    let char_vector: Vec<char> = s.chars().collect();
    for i in 2..size {
        if char_vector[i] == char_vector[i - 1] && char_vector[i] == char_vector[i - 2] {
            return Some(char_vector[i]);
        }
    }
    return None;
}

fn main() {
    let argv: Vec<String> = std::env::args().collect();
    assert_eq!(argv.len(), 2);
    let salt = argv[1].clone();

    let mut hash_storage = HashStorage::new(salt);

    let mut i = 0;
    let mut found = 0;
    while found < 64 {
        println!("Checking i={i}");
        let h = hash_storage.get(i);
        match get_first_triple_char(&h) {
            Some(c) => {
                let quintuple = c.to_string().repeat(5);
                for j in i + 1..=i + 1000 {
                    let future_hash = hash_storage.get(j);
                    if future_hash.contains(&quintuple) {
                        found += 1;
                        println!("Found key number {i}");
                        break;
                    }
                }
            }
            None => (),
        }

        i += 1;
    }

    println!("{}", i - 1); // incremented one time too much
}
