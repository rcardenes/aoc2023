use std::io::stdin;

use day09::find_next;

fn main() {
    let result: i64 = stdin().lines().map(|line| {
        let values = line.unwrap().split_whitespace()
            .map(|k| k.parse::<i64>().unwrap())
            .collect::<Vec<_>>();
        find_next(&values)
    }).sum();

    println!("Sum of successors: {result}");
}
