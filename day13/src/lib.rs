use std::io::BufRead;
use anyhow::{bail, Result};

#[derive(PartialEq, Debug, Clone)]
pub enum Reflection {
    Vertical(usize),
    Horizontal(usize),
    None,
}

#[derive(Debug)]
pub enum MaybeSmuged {
    Vertical { col1: usize, col2: usize, pos: usize },
    Horizontal { line1: usize, line2: usize, pos: usize },
}

#[derive(Debug, Clone)]
pub struct Pattern {
    lines: Vec<String>,
}

fn reflecting_at(lines: &Vec<String>, ignore: Option<usize>) -> Result<usize> {
    let candidates = lines[..(lines.len() - 1)].iter().zip(lines[1..].iter())
        .enumerate()
        .filter(|(_, (l1, l2))| l1 == l2)
        .map(|(n, _)| n + 1)
        .collect::<Vec<_>>();

    let last = lines.len();
    for candidate in candidates {
        let post = lines.len() - candidate;

        let (arange, prange) = if candidate > post {
            ((candidate - post)..candidate, (candidate..last).rev())
        } else {
            (0..candidate, (candidate..(candidate * 2)).rev())
        };

        if arange.zip(prange).all(|(a, b)| { lines[a] == lines[b] })
            && !ignore.is_some_and(|val| val == candidate)
        {
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

/// Returns the Hamming Distance between two same-sized strings.
/// The Hamming Distance is defined as the number of positions at which
/// two strings are different. Thus, if a == b, the distance will be
/// 0. If both strings are totally different, then the distance will be a.len()
///
/// Returns both the distance and the position of the last difference. If the
/// distance is 0, the position has no meaning and will be 0 as well
fn hamming_distance(a: &str, b: &str) -> (usize, usize) {
    let mut last = 0;
    let mut count = 0;

    for (n, (cha, chb)) in a.chars().zip(b.chars()).enumerate() {
        if cha != chb {
            count += 1;
            last = n;
        }
    }

    (count, last)
}

fn smudged_candidates(lines: &Vec<String>) -> Vec<MaybeSmuged> {
    let mut result = vec![];

    for idx_a in 0..lines.len() {
        let linea = &lines[idx_a];
        let base = idx_a + 1;
        for (idx_b, lineb) in lines[(base)..].iter().enumerate() {
            let (d, p) = hamming_distance(linea.as_str(), lineb.as_str());
            if d == 1 {
                result.push(MaybeSmuged::Horizontal { line1: idx_a, line2: idx_b + base, pos: p })
            }
        }
    }

    let transposed = transpose(lines);

    for idx_a in 0..transposed.len() {
        let linea = &transposed[idx_a];
        let base = idx_a + 1;
        for (idx_b, lineb) in transposed[(base)..].iter().enumerate() {
            let (d, p) = hamming_distance(linea.as_str(), lineb.as_str());
            if d == 1 {
                result.push(MaybeSmuged::Vertical { col1: idx_a, col2: idx_b + base, pos: p })
            }
        }
    }

    result
}

impl Pattern {
    pub fn find_reflecting_point(&self) -> Reflection {
        self.find_reflecting_point_generalized(None)
    }

    pub fn find_reflecting_point_generalized(&self, ignore: Option<Reflection>) -> Reflection {
        let ig = match ignore { Some(Reflection::Horizontal(n)) => { Some(n) } , _ => None };
        match reflecting_at(&self.lines, ig) {
            Ok(n) => Reflection::Horizontal(n),
            _ => {
                let ig = match ignore { Some(Reflection::Vertical(n)) => { Some(n) } , _ => None };
                match reflecting_at(&transpose(&self.lines), ig) {
                    Ok(n) => Reflection::Vertical(n),
                    _ => Reflection::None,
                }
            }
        }
    }

    fn flip(&self, line: usize, pos: usize) -> Self {
        let mut pat = self.clone();

        let to_repl = &mut pat.lines[line];
        let new_char = if to_repl.chars().nth(pos).unwrap() == '.' { "#" } else { "." };
        to_repl.replace_range(pos..(pos + 1), new_char);

        pat
    }

    pub fn find_smudged_reflection(&self) -> Reflection {
        let current_point = self.find_reflecting_point();

        let candidate_pairs = smudged_candidates(&self.lines);

        for pair in candidate_pairs {
            match pair {
                MaybeSmuged::Vertical { col1, col2, pos } => {
                    for c in [col1, col2] {
                        let flipped = self.flip(pos, c);
                        let refl = flipped.find_reflecting_point_generalized(Some(current_point.clone()));
                        match refl {
                            Reflection::Vertical(_) => { return refl },
                            _ => {}
                        }
                    }
                },
                MaybeSmuged::Horizontal { line1, line2, pos } => {
                    for l in [line1, line2] {
                        let flipped = self.flip(l, pos);
                        let refl = flipped.find_reflecting_point_generalized(Some(current_point.clone()));
                        match refl {
                            Reflection::Horizontal(_) => { return refl },
                            _ => {}
                        }
                    }
                },
            }
        }

        todo!()
    }
}

pub fn score(refl: Reflection) -> usize {
    match refl {
        Reflection::Vertical(s) => s,
        Reflection::Horizontal(s) => s * 100,
        Reflection::None => 0,
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
