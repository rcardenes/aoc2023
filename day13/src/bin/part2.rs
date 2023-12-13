use std::io::{BufReader, stdin};

use day13::{read_patterns, score};

fn main() {
    let patterns = read_patterns(BufReader::new(stdin()));

    println!("Summarizing: {}",
             patterns.iter().fold(0, |acc, p| acc + score(p.find_smudged_reflection())));
}
