use std::io::{BufReader, stdin};

use day14::read_problem;

fn main() {
    let tmap = read_problem(BufReader::new(stdin()));

    println!("{}", tmap.roll_north().load_on_beams());
}
