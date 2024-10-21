use std::io::{self, BufRead};

/*
 * Complete the 'compareTriplets' function below.
 *
 * The function is expected to return an INTEGER_ARRAY.
 * The function accepts following parameters:
 *  1. INTEGER_ARRAY a
 *  2. INTEGER_ARRAY b
 */

fn compare_triplets(a: &[i32], b: &[i32]) -> Vec<i32> {
    let mut result: Vec<i32> = vec![0, 0];
    for i in 0..a.len() {
        if a[i] > b[i] {
            result[0] += 1;
        }else if a[i] < b[i] {
            result[1] += 1;
        }
    }
    return result;
}

pub fn main() {
    println!("Task 2")
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let a: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let b: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let result = compare_triplets(&a, &b);

    for i in 0..result.len() {
        print!("{}", result[i]);

        if i != result.len() - 1 {
            print!(" ");
        }
    }

    println!();
}
