use std::{io::BufRead, collections::HashSet};

#[derive(PartialEq, PartialOrd, Debug, Hash)]
pub struct Coord {
    row: usize,
    col: usize,
}

impl Coord {
    fn manhattan(&self, other: &Coord) -> usize {
        self.row.abs_diff(other.row) + self.col.abs_diff(other.col)
    }

    fn transform(&self, factor: usize, rows: &[usize], cols: &[usize]) -> Coord {
        let row_count = rows.iter().take_while(|&r| *r < self.row).count();
        let col_count = cols.iter().take_while(|&r| *r < self.col).count();

        Coord {
            row: self.row + (self.row - row_count) * (factor - 1),
            col: self.col + (self.col - col_count) * (factor - 1),
        }
    }
}

#[derive(Debug)]
pub struct SkyMap {
    coords: Vec<Coord>
}

impl SkyMap {
    pub fn expand(&self, factor: usize) -> SkyMap {
        let mut unique_rows = HashSet::new();
        let mut unique_cols = HashSet::new();

        for coord in self.coords.iter() {
            unique_rows.insert(coord.row);
            unique_cols.insert(coord.col);
        }

        let mut rows: Vec<_> = unique_rows.into_iter().collect();
        let mut cols: Vec<_> = unique_cols.into_iter().collect();
        rows.sort();
        cols.sort();

        let coords = self.coords
            .iter()
            .map(|c| c.transform(factor, rows.as_slice(), cols.as_slice()))
            .collect();

        SkyMap { coords }
    }

    pub fn distances(&self) -> Vec<usize> {
        let num_coords = self.coords.len();
        let mut result = vec![];

        for i in 0..(num_coords - 1) {
            for j in (i+1)..num_coords {
                result.push(self.coords[i].manhattan(&self.coords[j]));
            }
        }

        result
    }
}

pub fn read_map<R: BufRead>(stream: R) -> SkyMap {
    let it = stream.lines();
    let mut coords = vec![];
    for (n, row) in it.into_iter().enumerate() {
        for (col, _) in row.unwrap().match_indices('#') {
            coords.push(Coord { row: n, col });
        }
    }

    SkyMap { coords }
}

#[cfg(test)]
mod tests {
}
