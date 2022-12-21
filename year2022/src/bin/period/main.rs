use std::io::{stdin, BufRead};

fn check_any_period(v: &Vec<i64>) -> Option<(usize, usize)> {
    let min_period = 5;
    let max_period = v.len() / 2;
    for period in min_period..max_period + 1 {
        match check_period(v, period) {
            Some(start_index) => return Some((period, start_index)),
            None => (),
        }
    }
    return None;
}

fn check_period(v: &Vec<i64>, period: usize) -> Option<usize> {
    for start_index in 0..period {
        if check_period_from(v, period, start_index) {
            return Some(start_index);
        }
    }
    return None;
}

fn check_period_from(v: &Vec<i64>, period: usize, start_index: usize) -> bool {
    for i in (start_index + period..v.len()).step_by(period) {
        for j in 0..period {
            if i + j >= v.len() {
                return false;
            }

            if v[start_index + j] != v[i + j] {
                return false;
            }
        }
    }
    return true;
}

fn sum_slice(v: &Vec<i64>, from: usize, to: usize) -> i64 {
    let mut sum = 0;
    for i in from..to {
        sum += v[i];
    }
    return sum;
}

fn main() {
    let v: Vec<i64> = stdin()
        .lock()
        .lines()
        .nth(0)
        .unwrap()
        .expect("Could not read line")
        .split(" ")
        .map(|x| x.parse::<i64>().unwrap())
        .collect();

    let target: i64 = 1_000_000_000_000;
    println!("target: {target}");

    match check_any_period(&v) {
        Some((period, start_index)) => {
            println!("period: {period}, start_index: {start_index}");
            let sum_before_period = sum_slice(&v, 0, start_index);
            println!("sum before period: {sum_before_period}");

            let period_sum = sum_slice(&v, start_index, start_index + period);
            println!("period_sum: {period_sum}");

            let periods = (target - start_index as i64) / period as i64;
            let remainder = (target - start_index as i64) % period as i64;
            println!("periods: {periods}, remainder: {remainder}");

            let remainder_sum = sum_slice(&v, start_index, start_index + remainder as usize);
            println!("remainder sum: {remainder_sum}");
            println!("sol: {}", sum_before_period + period_sum * periods + remainder_sum);
        }
        None => println!("No period"),
    }
}
