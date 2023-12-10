use std::io::{BufReader, stdin};

use anyhow::Result;
use day10::parse_input;

fn main() -> Result<()> {
    let map = parse_input(BufReader::new(stdin()))?;
    map.print();
    println!("Total steps: {}", map.find_furthest());

    Ok(())
}
