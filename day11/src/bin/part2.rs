use std::io::{stdin, BufReader};

use day11::read_map;
fn main() {
    let map = read_map(BufReader::new(stdin()));
    let factor = 1000000;
    let distances = map.expand(factor).distances();
    println!("Sum of distances between galaxies (x{factor}): {}", distances.iter().sum::<usize>());
}
