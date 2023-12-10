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

    pub fn find_furthest(&self) -> usize {
        let mut generations = 0;

        iterate_over_path(self.start, &self.rows, |_| { generations += 1 });

        generations - 1
    }

    pub fn count_inside(&self) -> usize {
        let mut map = (0..self.rows.len())
            .map(|_| format!("{:.<width$}", "", width = self.rows[0].len()).chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        iterate_over_path(
            self.start,
            &self.rows,
            |gen: &mut Vec<(usize, usize)>| {
                for &(row, col) in gen.iter() {
                    map[row][col] = self.rows[row][col].to_uppercase().next().unwrap();
                }
            });

        map.iter().map(|row| apply_raycast(row)).sum::<usize>()
    }

    pub fn print(&self) {
        print_map(&self.rows)
    }
}

fn apply_raycast(row: &Vec<char>) -> usize {
    let mut prev = '.';
    let mut count = 0;
    let mut inside = false;

    for &ch in row.iter() {
        match ch {
            '.' if inside => { count += 1 },
            '|'|'F'|'L' => { (prev, inside) = (ch, !inside) }
            'J' => {
                if prev != 'F' { inside = !inside };
                prev = 'J'
            }
            '7' => {
                if prev != 'L' { inside = !inside };
                prev = '7'
            }
            _ => {}
        }
    }

    count
}

fn iterate_over_path(start: (usize, usize), map: &Vec<Vec<char>>, mut action: impl FnMut(&mut Vec<(usize, usize)>)) {
    let mut known = HashSet::new();
    let mut generation = vec![start];

    known.insert(start);

    while !generation.is_empty() {
        action(&mut generation);

        let mut new_generation = Vec::new();

        for &(row, col) in generation.iter() {
            known.insert((row, col));
            match map[row][col] {
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
}

fn print_map(rows: &Vec<Vec<char>>) {
    for row in rows.iter() {
        let s: String = row.iter().collect();
        eprintln!("{s}");
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
    use crate::apply_raycast;

    #[test]
    fn raycasting() {
        assert_eq!(apply_raycast(&".F-7.".chars().collect()), 0);
        assert_eq!(apply_raycast(&".|.|.".chars().collect()), 1);
        assert_eq!(apply_raycast(&"SJ.L7".chars().collect()), 1);
        assert_eq!(apply_raycast(&".FJ|.".chars().collect()), 0);
        assert_eq!(apply_raycast(&"|F--J".chars().collect()), 0);
        assert_eq!(apply_raycast(&"FJL7L7LJLJ||LJ.L-7..".chars().collect()), 1);
        assert_eq!(apply_raycast(&"L--J.L7...LJF7F-7L7.".chars().collect()), 3);
        assert_eq!(apply_raycast(&"L---JF-JLJ....FJLJ..".chars().collect()), 4);
    }
}
