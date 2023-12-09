use std::io::stdin;

use day09::find_prev;

fn main() {
    let result: i64 = stdin().lines().map(|line| {
        let values = line.unwrap().split_whitespace()
            .map(|k| k.parse::<i64>().unwrap())
            .collect::<Vec<_>>();
        find_prev(&values)
    }).sum();

    println!("Sum of predecessors: {result}");
}
