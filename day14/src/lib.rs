use std::io::BufRead;

use md5::Digest;

#[derive(Debug)]
pub struct TerrainMap {
    rows: Vec<String>,
}

impl TerrainMap {
    /// Transpose the whole map
    fn transpose(&self) -> TerrainMap {
        let t_rows: Vec<String> = (0..(self.rows[0].len()))
            .map(|col| self.rows.iter().map(|r| r.chars().nth(col).unwrap()).collect())
            .collect();
        
        TerrainMap { rows: t_rows }
    }

    /// Flip horizontally
    fn flip(&self) -> TerrainMap {
        TerrainMap {
            rows: self.rows
                .iter()
                .map(|r| r.chars().rev().collect())
                .collect()
        }
    }

    pub fn hash(&self) -> Digest{
        md5::compute(self.rows.join(""))
    }

    pub fn spin_cycle(self) -> TerrainMap {
        (0..4).fold(self, |acc, _| acc.roll_north().rotate())
    }

    pub fn roll_north(&self) -> TerrainMap {
        TerrainMap { rows: self.transpose().rows
                                .iter()
                                .map(|r| roll_left(r.as_str()))
                                .collect()
        }.transpose()
    }

    /// Rotates the map clockwise
    pub fn rotate(&self) -> TerrainMap {
        self.transpose()
            .flip()
    }

    pub fn load_on_beams(&self) -> usize {
        let factor = self.rows.len();
        let transposed = self.transpose();

        transposed.rows
            .iter()
//            .map(|r| roll_left(r.as_str()))
            .map(|r| r.chars()
                 .enumerate()
                 .filter(|(_, c)| *c == 'O')
                 .map(|(k, _)| factor - k)
                 .sum::<usize>())
            .sum::<usize>()
    }
}

#[derive(Debug)]
enum RockGroup {
    Rounded { lowest: usize, highest: usize, number: usize },
    Blocking { pos: usize },
}

fn find_groups(row: &str) -> Vec<RockGroup> {
    let mut result = vec![];
    let mut current = None;

    for (k, ch) in row.chars().enumerate() {
        match ch {
            '#' => {
                if let &Some(RockGroup::Rounded{ .. }) = &current {
                    result.push(current.unwrap());
                }
                current = Some(RockGroup::Blocking { pos: k})
            }
            'O' => {
                if let Some(RockGroup::Rounded { lowest, number, .. }) = current {
                    current = Some(RockGroup::Rounded { lowest, highest: k, number: number + 1 });
                } else {
                    if !current.is_none() { result.push(current.unwrap()); }
                    current = Some(RockGroup::Rounded { lowest: k, highest: k, number: 1 });
                }
            }
            _ => {}
        }
    }

    if !current.is_none() {
        result.push(current.unwrap())
    }

    result
}

fn roll_left(row: &str) -> String {
    let groups = find_groups(row);
    let mut result = String::from(row);

    let mut lowest_fall = 0;

    for group in groups {
        match group {
            RockGroup::Blocking { pos } => lowest_fall = pos + 1,
            RockGroup::Rounded { highest, number, .. } => {
                let padding = highest + 1 - (lowest_fall + number);
                result.replace_range(lowest_fall..=highest, &format!("{:O<number$}{:.<padding$}", "", ""));
            }
        }
    }

    result
}

pub fn read_problem<R: BufRead>(stream: R) -> TerrainMap {
    let rows: Vec<String> = stream.lines().map(|l| l.unwrap()).collect();

    TerrainMap { rows }
}
