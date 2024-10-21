use std::io::{self, BufRead};

/*
 * Complete the 'plusMinus' function below.
 *
 * The function accepts INTEGER_ARRAY arr as parameter.
 */

fn plus_minus(arr: &[i32],) {
    let mut positives = 0;
    let mut negatives = 0;
    let mut zeros = 0;
    let n = arr.len() as f64;
    for &num in arr {
        if num > 0 {
            positives += 1;
        } else if num < 0 {
            negatives += 1;
        } else {
            zeros += 1;
        }
    }

    println!("{:.6}", positives as f64 / n);
    println!("{:.6}", negatives as f64 / n);
    println!("{:.6}", zeros as f64 / n);
}

pub fn main() {
    println!("Task 5");
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let _n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let arr: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    plus_minus(&arr);
}
