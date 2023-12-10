use std::{io::BufRead, collections::HashSet};
use anyhow::Result;

pub struct Map {
    rows: Vec<Vec<char>>,
    start: (usize, usize),
}

impl Map {
    fn new(lines: Vec<String>, start: (usize, usize)) -> Self {
        let mut rows: Vec<Vec<char>> = lines
            .iter()
            .map(|s| s.chars().collect::<Vec<_>>())
            .collect();

        let (srow, scol) = start;
        let max_col = rows[0].len() - 1;
        let max_row = rows.len() - 1;

        // Turn the starting point into its real pipe shape
        let left = if scol == 0 { '.' } else { rows[srow][scol - 1] };
        let right = if scol == max_col { '.' } else { rows[srow][scol + 1] };
        let top = if srow == 0 { '.' } else { rows[srow - 1][scol] };
        let bottom = if srow == max_row { '.' } else { rows[srow + 1][scol] };

        rows[srow][scol] = match (top, left, right, bottom) {
            ('|', _, _, '|') => '|',
            (_, '-', '-', _) => '-',
            (t, _, r, _) if "|7F".contains(t) && "-7J".contains(r) => 'L',
            (t, l, _, _) if "|7F".contains(t) && "-LF".contains(l) => 'J',
            (_, l, _, b) if "|LJ".contains(b) && "-LF".contains(l) => '7',
            (_, _, r, b) if "|LJ".contains(b) && "-7J".contains(r) => 'F',
            _ => {
                unimplemented!()
            } // Won't happen
        };

        Map { rows, start }
    }

    pub fn print(&self) {
        for row in self.rows.iter() {
            let s: String = row.iter().collect();
            eprintln!("{s}");
        }
    }

    pub fn find_furthest(&self) -> usize {
        let mut generations = 0;
        let mut known = HashSet::new();
        let mut generation = vec![self.start];

        known.insert(self.start);

        while !generation.is_empty() {
            generations +=1;

            let mut new_generation = Vec::new();

            for &(row, col) in generation.iter() {
                known.insert((row, col));
                match self.rows[row][col] {
                    '-' => { new_generation.extend([(row, col-1), (row, col+1)]) }
                    '|' => { new_generation.extend([(row-1, col), (row+1, col)]) }
                    'L' => { new_generation.extend([(row-1, col), (row, col+1)]) }
                    'J' => { new_generation.extend([(row-1, col), (row, col-1)]) }
                    '7' => { new_generation.extend([(row+1, col), (row, col-1)]) }
                    'F' => { new_generation.extend([(row+1, col), (row, col+1)]) }
                    _ => unimplemented!() // Won't happen
                }
            }

            let unique: HashSet<(usize, usize)> = HashSet::from_iter(new_generation.iter().copied());

            generation = Vec::from_iter(unique.difference(&known).into_iter().copied());
        }

        generations - 1
    }
}

pub fn parse_input<R: BufRead>(stream: R) -> Result<Map> {
    let lines: Vec<String> = stream
        .lines()
        .map(|rl| rl.map_err(drop))
        .flatten()
        .collect();

    let mut start = (0, 0);
    for (row, line) in lines.iter().enumerate() {

        if let Some(index) = line.find('S') {
            start = (row, index);
        }
    }


    Ok(Map::new(lines, start))
}

#[cfg(test)]
mod tests {
}
