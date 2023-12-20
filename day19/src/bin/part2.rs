use std::io::{BufReader, stdin};
use day19::*;

fn main() {
    let (evaluator, _) = read_problem(BufReader::new(stdin()));
    let ev_tree = evaluator.as_tree();
    let combinations = ev_tree.traverse().iter().map(|a| a.combinations()).collect::<Vec<_>>();
    eprintln!("Sum: {}", combinations.iter().sum::<u64>());
}
