use std::io::BufReader;
use std::io::stdin;

use day08::parse_instructions;

fn main() {
    let problem = parse_instructions(BufReader::new(stdin()));

    let steps = problem.iterate("AAA", |s| { s == "ZZZ" });

    println!("It took {} steps to find the ZZZ node", steps);
}
