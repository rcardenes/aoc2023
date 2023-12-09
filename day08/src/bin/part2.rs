use std::io::BufReader;
use std::io::stdin;

use day08::parse_instructions;

fn main() {
    let problem = parse_instructions(BufReader::new(stdin()));

    let steps = problem.parallel_iterate('A', 'Z');

    println!("It took {steps} steps to get all the ghosts simultaneously at the end of the path");
}
