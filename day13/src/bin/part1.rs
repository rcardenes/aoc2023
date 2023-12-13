use std::io::{BufReader, stdin};

use day13::{read_patterns, Score};

fn main() {
    let patterns = read_patterns(BufReader::new(stdin()));

    println!("Summarizing: {}",
             patterns.iter().fold(0, |acc, p| acc + match p.find_reflection_score() {
                 Score::Vertical(s) => s,
                 Score::Horizontal(s) => s * 100,
                 Score::None => 0,
             }));
}
