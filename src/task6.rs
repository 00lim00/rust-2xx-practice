use std::io::{self, BufRead};

/*
 * Complete the 'staircase' function below.
 *
 * The function accepts INTEGER n as parameter.
 */

fn staircase(n: i32) {
    let sign = '#';
    for i in 0..n {
        for _j in 0..n - 1 - i {
            print!(" ");
        }
        for _l in 0..i + 1 {
            print!("{}", sign);
        }
        println!("");
    }
    //Or
    /*for y in 0..n {
        for x in 0..n {
            let c = if x + 1 + y < n { ' ' } else { '#' };
            print!("{c}");
        }
        println!();
    }*/
}

pub fn main() {
    println!("Task 6");
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    staircase(n);
}