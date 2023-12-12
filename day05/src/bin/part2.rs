use std::io::{stdin, BufReader};

use anyhow::Result;
use day05::{read_seed_ranges, Map, sort_and_merge};

fn main() -> Result<()>{
    let mut input = BufReader::new(stdin());
    let seed_ranges = read_seed_ranges(&mut input)?;
    let mut maps = vec![];

    while let Some(map) = Map::read(&mut input)? {
        maps.push(map);
    }

    let locations = maps
        .iter()
        .fold(sort_and_merge(seed_ranges), |acc, m| m.map_ranges(acc));

    eprintln!("The lowest location is: {}", locations[0].start);

    Ok(())
}
