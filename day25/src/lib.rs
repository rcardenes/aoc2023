pub mod graph;

use std::io::BufRead;

use graph::Graph;

pub fn read_problem<R: BufRead>(stream: R) -> (Graph, (String, String)) {
    let lines = stream.lines().map(|l| l.unwrap());

    let mut graph = Graph::default();
    let mut ends = vec![];
    for line in lines {
        let (orig, destinations) = line.split_once(": ").unwrap();
        if ends.len() < 2 {
            ends.push(orig.to_string());
        }
        for dest in destinations.trim().split_whitespace() {
            graph.add_edge(orig, dest);
        }
    }

    (graph, (ends.remove(0), ends.remove(0)))
}
