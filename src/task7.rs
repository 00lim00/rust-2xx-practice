use std::io::{self, BufRead};

fn min_max_sum(arr: &[i64]) {
    let mut arr_of_sum: Vec<i64> = Vec::new();

    for j in 0..arr.len() {
        let mut result: i64 = 0;
        for mut i in j..arr.len() - 1 + j {
            i %= arr.len();
            result += arr[i];
        }
        arr_of_sum.push(result);
    }

    let min_sum = arr_of_sum.iter().min().unwrap();
    let max_sum = arr_of_sum.iter().max().unwrap();

    print!("{} {}", min_sum, max_sum);
}

pub fn main() {
    println!("Task 7");
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let arr: Vec<i64> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i64>().unwrap())
        .collect();

    min_max_sum(&arr);
}