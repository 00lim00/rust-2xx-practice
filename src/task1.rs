use std::io::{self, BufRead};

/*
 * Complete the 'simpleArraySum' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts INTEGER_ARRAY ar as parameter.
 */

fn simple_array_sum(ar: &[i32]) -> i32 {
    ar.iter().sum()
    //або вручну:
    /*
    let mut result = 0;
    for i in 0..ar.len() {
        result += ar[i];
    }
    return result;
    */
}

pub fn main() {
    println!("Task 1");
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let _ar_count = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let ar: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    let result = simple_array_sum(&ar);

    println!("{}", result);
}
