use std::io::{stdin, BufReader};

use anyhow::Result;
use day05::read_data;

fn main() -> Result<()>{
    let mut input = BufReader::new(stdin());
    let (seeds, maps) = read_data(&mut input)?;

    let smallest = seeds
        .iter()
        .map(|&seed|
             maps.iter()
                .fold(seed, |acc, mapping| mapping.map_value(acc)))
        .reduce(|acc, value| std::cmp::min(acc, value))
        .unwrap();
    println!("Smallest: {smallest}");

    Ok(())
}
