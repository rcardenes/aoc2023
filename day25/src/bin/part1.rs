use std::io::{BufReader, stdin};
use day25::*;

fn main() {
    let (graph, (source, sink)) = read_problem(BufReader::new(stdin()));
    let residual = graph.ford_fulkerson(&source, &sink);
    let (p1, p2) = residual.partition(&source);

    eprintln!("Result: {} * {} = {}", p1.len(), p2.len(), p1.len() * p2.len());
}
