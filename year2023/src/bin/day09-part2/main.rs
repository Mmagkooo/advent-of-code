fn find_prev(seq: &[i32]) -> i32 {
    if seq.iter().all(|e| *e == 0) {
        return 0;
    }

    let mut diffs = vec![];
    for i in 1..seq.len() {
        diffs.push(seq[i] - seq[i - 1]);
    }

    let prev_diff = find_prev(&diffs);
    seq.first().unwrap() - prev_diff
}

fn main() {
    let sol: i32 = std::io::stdin()
        .lines()
        .map(|l| l.unwrap())
        .map(|l| {
            l.split_whitespace()
                .map(|val| val.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .map(|seq| find_prev(&seq))
        .sum();
    println!("{sol}");
}
