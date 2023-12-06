/// allowed seconds: t
/// accumulating_seconds: t_a
/// moving_seconds: (t - t_a)
/// distance_travelled = t_a * (t - t_a) = t*t_a - t_a^2
/// distance_needed = d
/// inequation: distance_travelled > d
///             t*t_a - t_a^2 > d
///             -t_a^2 + t*t_a - d > 0
///             t_a^2 - t*t_a + d < 0
/// equation solutions: t_a = (t +- sqrt(t^2 - 4d)) / 2
fn main() {
    let mut lines = std::io::stdin().lines().map(|l| l.unwrap()).map(|l| {
        l.split(':')
            .nth(1)
            .unwrap()
            .trim()
            .split_whitespace()
            .map(|val| val.trim())
            .collect::<Vec<&str>>()
            .join("")
            .parse()
            .unwrap()
    });

    let t = lines.next().unwrap();
    let d = lines.next().unwrap();

    let discriminant_root = ((t * t - 4 * d) as f64).sqrt();
    assert!(discriminant_root.is_sign_positive());

    let mut t_min = ((t as f64 - discriminant_root) / 2.0).ceil() as i64;
    t_min = std::cmp::max(t_min, 0);
    if !satisfies_inequation(t, d, t_min) {
        // necessary because it's a strict inequality (i.e. <, not <=)
        t_min += 1;
    }

    let mut t_max = ((t as f64 + discriminant_root) / 2.0).floor() as i64;
    t_max = std::cmp::min(t_max, t - 1);
    if !satisfies_inequation(t, d, t_max) {
        t_max -= 1;
    }

    let sol = t_max - t_min + 1;
    println!("{sol}");
}

fn satisfies_inequation(t: i64, d: i64, t_a: i64) -> bool {
    t_a * (t - t_a) > d
}
