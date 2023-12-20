use std::io::{BufReader, stdin};
use day19::*;

fn main() {
    let (evaluator, parts) = read_problem(BufReader::new(stdin()));

    let accepted_parts = parts.iter().filter(|&p| evaluator.is_accepted(p));

    println!("Sum of ratings: {}", accepted_parts.map(|p| p.combined_rating()).sum::<usize>());
}
