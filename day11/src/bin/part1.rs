use std::io::{stdin, BufReader};

use day11::read_map;
fn main() {
    let map = read_map(BufReader::new(stdin()));
    let distances = map.expand(2).distances();
    println!("Sum of distances between galaxies: {}", distances.iter().sum::<usize>());
}
