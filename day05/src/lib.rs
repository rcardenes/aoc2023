use std::io::BufRead;

use anyhow::Result;

#[derive(PartialEq, PartialOrd, Ord, Debug)]
struct Mapping {
    dest_start: usize,
    source_start: usize,
    length: usize,
}

impl Mapping {
    fn new(dest_start: usize, source_start: usize, length: usize) -> Self {
        Mapping {
            dest_start,
            source_start,
            length,
        }
    }

    fn map(&self, value: usize) -> Option<usize> {
        let source_end = self.source_start + self.length;
        if (self.source_start..source_end).contains(&value) {
            Some(self.dest_start + (value - self.source_start))
        } else {
            None
        }
    }
}

impl Eq for Mapping {}

#[derive(PartialEq, Debug)]
pub struct Map {
    source: String,
    destination: String,
    mappings: Vec<Mapping>,
}

impl Map {
    fn new(source: &str, destination: &str, mappings: Vec<Mapping>) -> Self {
        let mut mappings = mappings;
        mappings.sort();

        Map {
            source: source.into(),
            destination: destination.into(),
            mappings,
        }
    }

    fn read<R: BufRead>(stream: &mut R) -> Result<Option<Self>> {
        let mut buf = String::from("");

        // Read and parse the map header
        stream.read_line(&mut buf)?;

        if buf.len() == 0 {
            return Ok(None);
        }

        let (map_name, _) = buf.split_once(' ').unwrap();
        let (source, destination) = map_name.split_once("-to-").unwrap();
        let source = source.to_string();
        let destination = destination.to_string();
        let mut mappings = vec![];
        loop {
            buf.clear();
            stream.read_line(&mut buf)?;
            if buf.trim().len() == 0 {
                break;
            }

            let numbers = buf
                .split_whitespace()
                .map(|num| num.parse::<usize>().unwrap())
                .collect::<Vec<_>>();

            mappings.push(Mapping::new(numbers[0], numbers[1], numbers[2]));
        }

        Ok(Some(Map::new(&source, &destination, mappings)))
    }

    pub fn map_value(&self, value: usize) -> usize {
        for mapping in self.mappings.iter() {
            if let Some(new_value) = mapping.map(value) {
                return new_value
            }
        }

        value
    }
}

pub fn read_seed_numbers<R: BufRead>(stream: &mut R) -> Result<Vec<usize>> {
    let mut buf = String::from("");

    stream.read_line(&mut buf)?;
    let (_, seed_numbers) = buf.split_once(": ").unwrap();

    let result = seed_numbers
       .split_whitespace()
       .map(|number| number.parse::<usize>().unwrap())
       .collect::<Vec<_>>();

    // Skip the following line
    stream.read_line(&mut buf)?;

    Ok(result)
}

pub fn read_data<R: BufRead>(stream: &mut R) -> Result<(Vec<usize>, Vec<Map>)> {
    let seed_numbers = read_seed_numbers(stream)?;
    let mut maps = vec![];

    while let Some(map) = Map::read(stream)? {
        maps.push(map);
    }

    Ok((seed_numbers, maps))
}


#[cfg(test)]
mod tests {
    use crate::{read_seed_numbers, Map, Mapping};

    const SMALL_INPUT: &str = include_str!("../input.small");

    #[test]
    fn parse_input() {
        let mut input = SMALL_INPUT.as_bytes();
        assert_eq!(read_seed_numbers(&mut input).unwrap(), vec![79, 14, 55, 13]);

        let map1 = Map::read(&mut input).unwrap();
        let other = Map::new("seed", "soil", vec![
                             Mapping::new(50, 98, 2),
                             Mapping::new(52, 50, 48),
        ]);
        assert_eq!(map1, Some(other));
    }
}
