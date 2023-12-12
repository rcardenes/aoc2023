use std::{cmp::PartialOrd, io::BufRead, ops::Range};

use anyhow::Result;
use range_ext::intersect::{Intersect, IntersectionExt};

#[derive(Debug)]
struct MappingResult {
    transformed: Vec<Range<usize>>,
    original: Vec<Range<usize>>,
}

impl MappingResult {
    fn only_orig(original: Vec<Range<usize>>) -> Self {
        MappingResult {
            transformed: vec![],
            original,
        }
    }

    fn only_trans(transformed: Vec<Range<usize>>) -> Self {
        MappingResult {
            transformed,
            original: vec![],
        }
    }

    fn new(transformed: Vec<Range<usize>>, original: Vec<Range<usize>>) -> Self {
        MappingResult { transformed, original }
    }
}

#[derive(PartialEq, Debug)]
pub struct Mapping {
    source: Range<usize>,
    dest: Range<usize>,
    length: usize,
}

impl Ord for Mapping {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.source.start, self.dest.start, self.length).cmp(&(other.source.start, other.dest.start, other.length))
    }
}

impl PartialOrd for Mapping {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Mapping {
    pub fn new(dest_start: usize, source_start: usize, length: usize) -> Self {
        Mapping {
            source: source_start..(source_start + length),
            dest: dest_start..(dest_start + length),
            length,
        }
    }

    fn map(&self, value: usize) -> Option<usize> {
        if self.source.contains(&value) {
            Some(self.dest.start + (value - self.source.start))
        } else {
            None
        }
    }

    fn map_range(&self, rng: &Range<usize>) -> MappingResult {
        match rng.intersect_ext(&self.source) {
            IntersectionExt::Less |
                IntersectionExt::Greater => MappingResult::only_orig(vec![rng.clone()]),
            IntersectionExt::Same => MappingResult::only_trans(
                vec![self.dest.clone()]
                ),
            IntersectionExt::Within => { // The source mapping is fully within the range that was passed
                let diff = rng.start - self.source.start;
                let length = rng.end - rng.start;
                let start = self.dest.start + diff;
                MappingResult::only_trans(
                    vec![start..(start + length)]
                    )
            },
            IntersectionExt::Over => { // The source mapping fully contains the range that was passed
                let transformed = vec![self.dest.clone()];
                let mut original = vec![];
                if self.source.start > rng.start {
                    original.push(rng.start..self.source.start)
                };
                if self.source.end < rng.end {
                    original.push(self.source.end..rng.end)
                };
                MappingResult::new(transformed, original)
            }
            IntersectionExt::LessOverlap => {
                let offset = rng.end - self.source.start;

                MappingResult::new(
                    vec![self.dest.start..(self.dest.start + offset)],
                    vec![rng.start..self.source.start]
                    )
            }
            IntersectionExt::GreaterOverlap => {
                let offset = rng.start - self.source.start;
                MappingResult::new(
                    vec![(self.dest.start + offset)..self.dest.end],
                    vec![(rng.start + (self.length - offset))..rng.end]
                    )
            }
            IntersectionExt::Empty => unimplemented!() // Won't happen, anyway
        }
    }
}

impl Eq for Mapping {}

#[derive(PartialEq, Debug)]
pub struct Map {
    source: String,
    destination: String,
    pub mappings: Vec<Mapping>,
}

impl Map {
    pub fn new(source: &str, destination: &str, mappings: Vec<Mapping>) -> Self {
        let mut mappings = mappings;
        mappings.sort();

        Map {
            source: source.into(),
            destination: destination.into(),
            mappings,
        }
    }

    pub fn transition(&self) -> String {
        format!("{} -> {}", self.source, self.destination)
    }

    pub fn read<R: BufRead>(stream: &mut R) -> Result<Option<Self>> {
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

    pub fn map_ranges(&self, rngs: Vec<Range<usize>>) -> Vec<Range<usize>> {
        let result = self.mappings.iter()
            .fold(
                MappingResult::only_orig(rngs),
                |acc, mapping| {
                    let mut so_far = acc.transformed;
                    let (transformed, original): (Vec<_>, Vec<_>) = acc.original
                        .iter()
                        .map(|x| mapping.map_range(x))
                        .map(|res| (res.transformed, res.original))
                        .unzip();
                    so_far.extend(transformed.into_iter().flatten());
                    MappingResult::new(
                        so_far,
                        sort_and_merge(original.into_iter().flatten().collect())
                        )
                });

        sort_and_merge(vec![result.original, result.transformed].into_iter().flatten().collect())
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

pub fn read_seed_ranges<R: BufRead>(stream: &mut R) -> Result<Vec<Range<usize>>> {
    let mut buf = String::from("");

    stream.read_line(&mut buf)?;
    let (_, raw_numbers) = buf.split_once(": ").unwrap();

    let numbers = raw_numbers
       .split_whitespace()
       .map(|number| number.parse::<usize>().unwrap())
       .collect::<Vec<_>>();

    let ranges = &numbers
        .chunks(2)
        .map(|chunk| (chunk[0]..chunk[0]+chunk[1]))
        .collect::<Vec<_>>();

    // Skip the following line
    stream.read_line(&mut buf)?;

    Ok(ranges.to_vec())
}

pub fn read_data<R: BufRead>(stream: &mut R) -> Result<(Vec<usize>, Vec<Map>)> {
    let seed_numbers = read_seed_numbers(stream)?;
    let mut maps = vec![];

    while let Some(map) = Map::read(stream)? {
        maps.push(map);
    }

    Ok((seed_numbers, maps))
}

pub fn sort_and_merge(ranges: Vec<Range<usize>>) -> Vec<Range<usize>> {
    let mut ranges = ranges;

    ranges.sort_by_key(|rng| rng.start);
    let mut result = vec![];

    for source_range in ranges {
        if result.len() == 0 {
            result.push(source_range);
        } else {
            let last = result.last_mut().unwrap();
            match last.intersect_ext(&source_range) {
                IntersectionExt::Same | IntersectionExt::Over => {}
                IntersectionExt::Less if source_range.start > last.end => {
                    result.push(source_range)
                }
                IntersectionExt::Less | IntersectionExt::LessOverlap => {
                    *last = last.start..source_range.end;
                }
                _ => unimplemented!() // doesn't happen because the vector is sorted
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use crate::{read_seed_numbers, Map, Mapping};

    const SMALL_INPUT: &str = include_str!("../input.small");

    fn get_single_map() -> Map {
        Map::new("seed", "soil", vec![
                 Mapping::new(50, 98, 2),
                 Mapping::new(52, 50, 48),
        ])
    }

    #[test]
    fn parse_input() {
        let mut input = SMALL_INPUT.as_bytes();
        assert_eq!(read_seed_numbers(&mut input).unwrap(), vec![79, 14, 55, 13]);

        let map1 = Map::read(&mut input).unwrap();
        let other = get_single_map();
        assert_eq!(map1, Some(other));
    }
}
