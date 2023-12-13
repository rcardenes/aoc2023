use std::io::BufRead;
use anyhow::{bail, Result};

#[derive(Debug)]
pub enum Score {
    Vertical(usize),
    Horizontal(usize),
    None,
}

#[derive(Debug)]
pub struct Pattern {
    lines: Vec<String>,
}

fn reflecting_at(lines: &Vec<String>) -> Result<usize> {
    let mut candidates = lines[..(lines.len() - 1)].iter().zip(lines[1..].iter())
        .enumerate()
        .filter(|(_, (l1, l2))| l1 == l2)
        .map(|(n, _)| n + 1)
        .collect::<Vec<_>>();

    candidates.reverse();

    let last = lines.len();
    for candidate in candidates {
        let post = lines.len() - candidate;

        let (arange, prange) = if candidate > post {
            ((candidate - post)..candidate, (candidate..last).rev())
        } else {
            (0..candidate, (candidate..(candidate * 2)).rev())
        };

        if arange.zip(prange).all(|(a, b)| {
            lines[a] == lines[b]
        }) {
            return Ok(candidate)
        }
    }

    bail!("No reflection!")
}

fn transpose(lines: &Vec<String>) -> Vec<String> {
    let mut transposed = vec![];

    for idx in 0..lines[0].len() {
        transposed.push(String::from_utf8(lines.iter().map(|l| l.as_bytes()[idx]).collect::<Vec<_>>()).unwrap());
    }

    transposed
}

impl Pattern {
    pub fn find_reflection_score(&self) -> Score {
        match reflecting_at(&self.lines) {
            Ok(n) => Score::Horizontal(n),
            _ => match reflecting_at(&transpose(&self.lines)) {
                Ok(n) => Score::Vertical(n),
                _ => Score::None,
            }
        }
    }
}

pub fn read_patterns<R: BufRead>(stream: R) -> Vec<Pattern> {
    let mut result = vec![];
    let mut pattern = vec![];

    for line in stream.lines() {
        let line = line.unwrap();
        if line.is_empty() {
            result.push(Pattern { lines: pattern.clone() });
            pattern.clear();
        } else {
            pattern.push(line);
        }
    }

    if !pattern.is_empty() {
        result.push(Pattern { lines: pattern });
    }

    result
}
